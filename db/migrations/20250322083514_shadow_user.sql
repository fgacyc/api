-- migrate:up
CREATE TABLE shadow_user (
  name TEXT,
  email TEXT,
  username TEXT,
  given_name TEXT,
  family_name TEXT,
  gender gender,
  ic_number TEXT,
  phone_number TEXT,
  nickname TEXT,
  avatar_url TEXT,
  address address,
  date_of_birth TIMESTAMPTZ,
  cg TEXT,
  pastoral_status TEXT,
  ministry TEXT,
  deleted BOOLEAN,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (name, phone_number)
);

-- migrate:down
DROP TABLE shadow_user;
