-- Add up migration script here
CREATE TYPE gender AS ENUM ('male', 'female');

CREATE TYPE _address AS (
  line_one TEXT,
  line_two TEXT,
  city TEXT,
  state TEXT,
  country TEXT,
  postal_code TEXT
);
COMMENT ON COLUMN _address.line_one IS 'Address line 1 (e.g., street, PO Box, or company name).';
COMMENT ON COLUMN _address.line_two IS 'Address line 2 (e.g., apartment, suite, unit, or building).';
COMMENT ON COLUMN _address.city IS 'City, district, suburb, town, or village.';
COMMENT ON COLUMN _address.state IS 'State, county, province, or region.';
COMMENT ON COLUMN _address.country IS 'Two-letter country code (ISO 3166-1 alpha-2).';
COMMENT ON COLUMN _address.postal_code IS 'ZIP or postal code.';

CREATE DOMAIN address AS _address CHECK (
  (value).line_one IS NOT NULL AND
  (value).city IS NOT NULL AND
  (value).state IS NOT NULL AND
  (value).country IS NOT NULL AND
  (value).postal_code IS NOT NULL
);

CREATE TABLE satellite (
  id SERIAL,
  name TEXT NOT NULL,
  address address NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id)
);
COMMENT ON COLUMN satellite.id IS 'Unique identifier of a satellite (e.g., 1, 2, etc.).';
COMMENT ON COLUMN satellite.name IS 'Name of a satellite (e.g., Puchong).';
COMMENT ON COLUMN satellite.address IS 'Address of a satellite.';
COMMENT ON COLUMN satellite.created_at IS 'Creation time of a satellite.';
COMMENT ON COLUMN satellite.updated_at IS 'Last updated time of a satellite.';

CREATE TABLE cg (
  id UUID DEFAULT gen_random_uuid(),
  no SERIAL NOT NULL,
  name TEXT,
  variant CHAR,
  satellite_id INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id),
  UNIQUE (no, satellite_id),
  FOREIGN KEY (satellite_id) REFERENCES satellite(id) ON UPDATE CASCADE
);
COMMENT ON COLUMN cg.id IS 'Unique identifier of a connect group (e.g., 9bf932bb-b8a2-41e6-a2a5-f667648ba380).';
COMMENT ON COLUMN cg.no IS 'Sequence number of a connect group (e.g., 1, 2, etc.).';
COMMENT ON COLUMN cg.name IS 'Name of a connect group.';
COMMENT ON COLUMN cg.variant IS 'Variant of a connect group (e.g., J, S, T, W, A, B, C).';
COMMENT ON COLUMN cg.satellite_id IS 'Satellite that the connect group belongs to.';
COMMENT ON COLUMN cg.created_at IS 'Creation time of a connect group.';
COMMENT ON COLUMN cg.updated_at IS 'Last updated time of a connect group.';

CREATE TABLE ministry_team (
  name TEXT,
  description TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (name)
);

CREATE TABLE ministry_department (
  name TEXT,
  description TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (name)
);

CREATE TABLE ministry (
  name TEXT,
  description TEXT NOT NULL,
  department TEXT NOT NULL,
  team TEXT NOT NULL,
  PRIMARY KEY (name),
  FOREIGN KEY (department) REFERENCES ministry_department(name) ON UPDATE CASCADE,
  FOREIGN KEY (team) REFERENCES ministry_team(name) ON UPDATE CASCADE
);

CREATE TABLE "user" (
  id TEXT,
  no SERIAL NOT NULL,
  email TEXT NOT NULL,
  email_verified BOOLEAN NOT NULL DEFAULT FALSE,
  username TEXT NOT NULL,
  given_name TEXT NOT NULL,
  family_name TEXT NOT NULL,
  name TEXT NOT NULL,
  gender gender NOT NULL,
  ic_number TEXT NOT NULL,
  phone_number TEXT NOT NULL,
  phone_number_verified BOOLEAN NOT NULL DEFAULT FALSE,
  nickname TEXT,
  avatar_url TEXT,
  address address,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id),
  UNIQUE (no),
  UNIQUE (email),
  UNIQUE (username)
);

-- To start the id from 100
ALTER SEQUENCE user_no_seq RESTART WITH 100; 

CREATE TABLE pastoral_role (
  id TEXT,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  weight INTEGER NOT NULL,
  PRIMARY KEY (id),
  UNIQUE (name)
);
COMMENT ON COLUMN pastoral_role.name IS 'The name of the pastoral role (e.g., CGL, OM, etc.).';
COMMENT ON COLUMN pastoral_role.weight IS 'The weight of the pastoral role.';

CREATE TABLE ministry_role (
  id TEXT,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  weight INTEGER NOT NULL,
  PRIMARY KEY (id),
  UNIQUE(name)
);
COMMENT ON COLUMN ministry_role.name IS 'The name of the ministry role (e.g., HOD, Team Lead, etc.).';
COMMENT ON COLUMN ministry_role.weight IS 'The weight of the ministry role.';

CREATE TABLE user_cg (
  user_id TEXT,
  cg_id UUID,
  user_role TEXT NOT NULL,
  PRIMARY KEY (user_id, cg_id),
  FOREIGN KEY (user_id) REFERENCES "user"(id) ON UPDATE CASCADE,
  FOREIGN KEY (cg_id) REFERENCES cg(id) ON UPDATE CASCADE,
  FOREIGN KEY (user_role) REFERENCES pastoral_role(id) ON UPDATE CASCADE
);

CREATE TABLE user_ministry (
  user_id TEXT,
  ministry_id UUID,
  user_role TEXT NOT NULL,
  PRIMARY KEY (user_id, ministry_id),
  FOREIGN KEY (user_id) REFERENCES "user"(id) ON UPDATE CASCADE,
  FOREIGN KEY (ministry_id) REFERENCES ministry(id) ON UPDATE CASCADE,
  FOREIGN KEY (user_role) REFERENCES ministry_role(id) ON UPDATE CASCADE
);
