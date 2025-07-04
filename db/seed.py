import os
import asyncio
import asyncpg
import csv
import random
import string

CSV_FILE = "seed_data.csv"
DATABASE_URL = os.getenv("DATABASE_URL")

CONNECT_GROUP_CATEGORY_ID = (
    "connect_group_category_43fd21a8808444d4ac51"  # represents None
)
PASTORAL_ROLES = [
    ("Connect Group Leader", "", 4),
    ("Coach", "", 3),
    ("Team Leader", "", 3),
    ("Small Group Leader", "", 5),
    ("Ordinary Member", "", 6),
]


def generate_random_string(length: int):
    """Generates a random string of a specified length."""
    # Define the possible characters: lowercase, uppercase letters, and digits
    characters = string.ascii_letters + string.digits
    # Use random.choice to pick a character for each position in the string
    random_string = "".join(random.choice(characters) for i in range(length))
    return random_string


async def seed_pastoral_roles(conn):
    for name, desc, weight in PASTORAL_ROLES:
        await conn.execute(
            """
            INSERT INTO pastoral_role (id, name, description, weight)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (name) DO NOTHING
        """,
            f"rol_{generate_random_string(20)}",
            name,
            desc,
            weight,
        )


async def get_or_create_satellite(conn, satellite_name):
    row = await conn.fetchrow(
        """
        SELECT id FROM satellite WHERE name=$1
    """,
        satellite_name,
    )
    if row:
        return row["id"]
    sat_id = f"satellite_{generate_random_string(20)}"
    await conn.execute(
        """
        INSERT INTO satellite (id, name, address)
        VALUES ($1, $2, NULL)
    """,
        sat_id,
        satellite_name,
    )
    return sat_id


async def get_or_create_connect_group(conn, cg_name, satellite_id):
    row = await conn.fetchrow(
        """
        SELECT id FROM connect_group WHERE name=$1 AND satellite_id=$2
    """,
        cg_name,
        satellite_id,
    )
    if row:
        return row["id"]
    cg_id = f"connect_group_{generate_random_string(20)}"
    # Replace 'your-category-id' with actual category id or handle appropriately
    await conn.execute(
        """
        INSERT INTO connect_group (id, name, variant, satellite_id, category_id)
        VALUES ($1, $2, NULL, $3, $4) 
    """,
        cg_id,
        cg_name,
        satellite_id,
        CONNECT_GROUP_CATEGORY_ID,
    )
    return cg_id


async def get_pastoral_role_id(conn, role_name):
    row = await conn.fetchrow(
        """
        SELECT id FROM pastoral_role WHERE name=$1
    """,
        role_name,
    )
    if not row:
        raise ValueError(f"Pastoral role {role_name} not found")
    return row["id"]


async def get_or_create_user(conn, full_name):
    row = await conn.fetchrow(
        """
        SELECT id FROM "user" WHERE name=$1
    """,
        full_name,
    )
    if row:
        return row["id"]
    row = await conn.fetchrow(
        """
        SELECT * FROM create_shadow_user(
            NULL,  -- email
            $1,  -- name
            NULL, -- username
            NULL, -- given_name
            NULL, -- family_name
            NULL, -- gender
            NULL, -- ic_number
            NULL, -- phone_number
            NULL, -- phone_number_verified
            NULL, -- nickname
            NULL, -- avatar_url
            NULL,  -- date_of_birth
            '{}'::jsonb, -- metadata
            NULL,  -- connect_group_id
            NULL   -- user_role
        )
    """,
        full_name,
    )
    return row["id"]


async def link_user_to_group(conn, user_id, group_id, role_id):
    await conn.execute(
        """
        INSERT INTO user_connect_group (user_id, connect_group_id, user_role)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, connect_group_id) DO NOTHING
    """,
        user_id,
        group_id,
        role_id,
    )


def map_pastoral_status(s: str):
    match s.lower():
        case "cgl":
            return "Connect Group Leader"
        case "coach":
            return "Coach"
        case "tl":
            return "Team Leader"
        case "sgl":
            return "Small Group Leader"
        case "om":
            return "Ordinary Member"


def map_satellite(s: str):
    match s.lower():
        case "seremban":
            return "FGACYC Seremban"


async def main():
    conn = await asyncpg.connect(DATABASE_URL)
    await seed_pastoral_roles(conn)

    with open(CSV_FILE, newline="", encoding="utf-8") as tsvfile:
        reader = csv.DictReader(tsvfile, delimiter="\t")
        for row in reader:
            full_name = row["Full Name"].strip()
            dob = row["D.O.B."].strip()
            pastoral_status = map_pastoral_status(row["Pastoral Status"].strip())
            cg_name = row["CG Name"].strip()
            satellite_name = map_satellite(row["Satellite"].strip())

            if (
                not full_name
                or not pastoral_status
                or not cg_name
                or not satellite_name
            ):
                print(f"[SKIP] Incomplete data in row: {row}")
                continue

            # Insert satellite
            satellite_id = await get_or_create_satellite(conn, satellite_name)

            # Insert connect group
            cg_id = await get_or_create_connect_group(conn, cg_name, satellite_id)

            # Insert user
            user_id = await get_or_create_user(conn, full_name)

            # Find pastoral role id
            role_id = await get_pastoral_role_id(conn, pastoral_status)

            # Link user to connect group
            await link_user_to_group(conn, user_id, cg_id, role_id)

            print(
                f"[OK] Seeded user {full_name} with role {pastoral_status} in CG {cg_name}"
            )

    await conn.close()


if __name__ == "__main__":
    asyncio.run(main())
