-- migrate:up
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
  (VALUE).line_one IS NOT NULL AND
  (VALUE).city IS NOT NULL AND
  (VALUE).state IS NOT NULL AND
  (VALUE).country IS NOT NULL AND
  (VALUE).postal_code IS NOT NULL
  OR VALUE IS NULL
);

CREATE TABLE satellite (
  id TEXT,
  no SERIAL NOT NULL,
  name TEXT NOT NULL,
  address address NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id)
);
COMMENT ON COLUMN satellite.id IS 'Unique identifier of a satellite (e.g., satellite_01H7JNPD7J67AA5AD87Q4SZDF9).';
COMMENT ON COLUMN satellite.no IS 'Sequence number of a satellite (e.g., 1, 2, etc.).';
COMMENT ON COLUMN satellite.name IS 'Name of a satellite (e.g., Puchong).';
COMMENT ON COLUMN satellite.address IS 'Address of a satellite.';
COMMENT ON COLUMN satellite.created_at IS 'Creation time of a satellite.';
COMMENT ON COLUMN satellite.updated_at IS 'Last updated time of a satellite.';

CREATE TABLE connect_group (
  id TEXT,
  no SERIAL NOT NULL,
  name TEXT,
  variant CHAR(2),
  satellite_id TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id),
  UNIQUE (satellite_id, no, variant),
  FOREIGN KEY (satellite_id) REFERENCES satellite(id) ON UPDATE CASCADE
);
COMMENT ON COLUMN connect_group.id IS 'Unique identifier of a connect_group (e.g., connect_group_01H7JNPD7J67AA5AD87Q4SZDF9).';
COMMENT ON COLUMN connect_group.no IS 'Sequence number of a connect group (e.g., 1, 2, etc.).';
COMMENT ON COLUMN connect_group.name IS 'Name of a connect group.';
COMMENT ON COLUMN connect_group.variant IS 'Variant of a connect group (e.g., J, S, T, W, A, B, C).';
COMMENT ON COLUMN connect_group.satellite_id IS 'Satellite that the connect group belongs to.';
COMMENT ON COLUMN connect_group.created_at IS 'Creation time of a connect group.';
COMMENT ON COLUMN connect_group.updated_at IS 'Last updated time of a connect group.';

CREATE TABLE ministry_team (
  id TEXT,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id)
);
COMMENT ON COLUMN ministry_team.id IS 'Unique identifier of a ministry team. (e.g., ministry_team_01H7JNPD7J67AA5AD87Q4SZDF9)';
COMMENT ON COLUMN ministry_team.name IS 'Name of a ministry team (e.g., People Experience, Creative, etc.)';
COMMENT ON COLUMN ministry_team.description IS 'Description of a ministry team.';
COMMENT ON COLUMN ministry_team.created_at IS 'Creation time of a ministry team.';
COMMENT ON COLUMN ministry_team.updated_at IS 'Last updated time of a ministry team.';

CREATE TABLE ministry_department (
  id TEXT,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id)
);
COMMENT ON COLUMN ministry_department.id IS 'Unique identifier of a ministry department. (e.g., ministry_department_01H7JNPD7J67AA5AD87Q4SZDF9)';
COMMENT ON COLUMN ministry_department.name IS 'Name of a ministry department (e.g., People, Tech, etc.)';
COMMENT ON COLUMN ministry_department.description IS 'Description of a ministry department.';
COMMENT ON COLUMN ministry_department.created_at IS 'Creation time of a ministry department.';
COMMENT ON COLUMN ministry_department.updated_at IS 'Last updated time of a ministry department.';

CREATE TABLE ministry (
  id TEXT,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  department_id TEXT NOT NULL,
  team_id TEXT NOT NULL,
  satellite_id TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id),
  FOREIGN KEY (department_id) REFERENCES ministry_department(id) ON UPDATE CASCADE,
  FOREIGN KEY (team_id) REFERENCES ministry_team(id) ON UPDATE CASCADE,
  FOREIGN KEY (satellite_id) REFERENCES satellite(id) ON UPDATE CASCADE
);
COMMENT ON COLUMN ministry.id IS 'Unique identifier of a ministry. (e.g., ministry_01H7JNPD7J67AA5AD87Q4SZDF9)';
COMMENT ON COLUMN ministry.name IS 'Name of a ministry (e.g., Usher, Software Developer, etc.)';
COMMENT ON COLUMN ministry.description IS 'Description of a ministry.';
COMMENT ON COLUMN ministry.team_id IS 'Team that a ministry belongs to.';
COMMENT ON COLUMN ministry.department_id IS 'Department that a ministry belongs to.';
COMMENT ON COLUMN ministry.satellite_id IS 'Satellite that a ministry belongs to.';
COMMENT ON COLUMN ministry.created_at IS 'Creation time of a ministry.';
COMMENT ON COLUMN ministry.updated_at IS 'Last updated time of a ministry.';

CREATE TABLE "user" (
  id TEXT,
  no SERIAL,
  email TEXT NOT NULL,
  email_verified BOOLEAN NOT NULL DEFAULT FALSE,
  name TEXT NOT NULL,
  username TEXT,
  given_name TEXT,
  family_name TEXT,
  gender gender,
  ic_number TEXT,
  phone_number TEXT,
  phone_number_verified BOOLEAN DEFAULT FALSE,
  nickname TEXT,
  avatar_url TEXT,
  address address,
  date_of_birth TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id),
  UNIQUE (no),
  UNIQUE (email),
  UNIQUE (username)
);
COMMENT ON COLUMN "user".id IS 'Unique identifier of a user. (e.g., auth0|01H7JNPD7J67AA5AD87Q4SZDF9)';

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
COMMENT ON COLUMN pastoral_role.id IS 'The id of the pastoral role (e.g., rol_2Nx8e5Tik0UnX4c1).';
COMMENT ON COLUMN pastoral_role.name IS 'The name of the pastoral role (e.g., CGL, OM, etc.).';
COMMENT ON COLUMN pastoral_role.description IS 'The description of the pastoral role.';
COMMENT ON COLUMN pastoral_role.weight IS 'The weight of the pastoral role.';

CREATE TABLE ministry_role (
  id TEXT,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  weight INTEGER NOT NULL,
  PRIMARY KEY (id),
  UNIQUE(name)
);
COMMENT ON COLUMN ministry_role.id IS 'The id of the ministry role (e.g., rol_2Nx8e5Tik0UnX4c1).';
COMMENT ON COLUMN ministry_role.name IS 'The name of the ministry role (e.g., HOD, Team Lead, etc.).';
COMMENT ON COLUMN ministry_role.description IS 'The description of the ministry role.';
COMMENT ON COLUMN ministry_role.weight IS 'The weight of the ministry role.';

CREATE TABLE user_connect_group (
  user_id TEXT,
  connect_group_id TEXT,
  user_role TEXT NOT NULL,
  PRIMARY KEY (user_id, connect_group_id),
  FOREIGN KEY (user_id) REFERENCES "user"(id) ON UPDATE CASCADE,
  FOREIGN KEY (connect_group_id) REFERENCES connect_group(id) ON UPDATE CASCADE,
  FOREIGN KEY (user_role) REFERENCES pastoral_role(id) ON UPDATE CASCADE
);

CREATE TABLE user_ministry (
  user_id TEXT,
  ministry_id TEXT,
  user_role TEXT NOT NULL,
  PRIMARY KEY (user_id, ministry_id),
  FOREIGN KEY (user_id) REFERENCES "user"(id) ON UPDATE CASCADE,
  FOREIGN KEY (ministry_id) REFERENCES ministry(id) ON UPDATE CASCADE,
  FOREIGN KEY (user_role) REFERENCES ministry_role(id) ON UPDATE CASCADE
);

CREATE TABLE currency (
  code CHAR(3),
  num INTEGER NOT NULL,
  denominator INTEGER NOT NULL,
  name TEXT NOT NULL,
  countries TEXT[] NOT NULL,
  PRIMARY KEY(code)
);
COMMENT ON COLUMN currency.code IS 'Currency code according to ISO4217 (e.g., USD, EUR, MYR, etc.).';
COMMENT ON COLUMN currency.countries IS 'An array of country code according to ISO3166-1 (e.g., MY, US, SG, etc.).';

CREATE TABLE event_type (
  name TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY(name)
);
COMMENT ON COLUMN event_type.name IS 'Unique name of an event type (e.g., camp, conference, etc.).';
COMMENT ON COLUMN event_type.created_at IS 'Creation time of a event type.';
COMMENT ON COLUMN event_type.updated_at IS 'Last updated time of a event type.';

CREATE TABLE event (
  id TEXT,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  type TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY(id),
  FOREIGN KEY(type) REFERENCES event_type(name)
);
COMMENT ON COLUMN event.id IS 'Unique identifier for an event (e.g., event_01H7JNPD7J67AA5AD87Q4SZDF9).';
COMMENT ON COLUMN event.created_at IS 'Creation time of a event.';
COMMENT ON COLUMN event.updated_at IS 'Last updated time of a event.';

CREATE TABLE form_field_type (
  type TEXT,
  description TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY(type)
);

CREATE TABLE registration (
  id TEXT,
  event_id TEXT NOT NULL,
  name TEXT NOT NULL,
  close_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY(id),
  UNIQUE(event_id, name),
  FOREIGN KEY(event_id) REFERENCES event(id)
);

CREATE TABLE registration_form_field (
  registration_id TEXT NOT NULL,
  name TEXT NOT NULL,
  label TEXT NOT NULL,
  description TEXT,
  type TEXT NOT NULL,
  weight SERIAL NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY(registration_id, name),
  UNIQUE(registration_id, weight),
  FOREIGN KEY(registration_id) REFERENCES registration(id),
  FOREIGN KEY(type) REFERENCES form_field_type(type)
);

CREATE TABLE registration_form_field_data (
  registration_id TEXT NOT NULL,
  name TEXT NOT NULL,
  user_id TEXT NOT NULL,
  data TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY(registration_id, name, user_id), 
  FOREIGN KEY(registration_id, name) REFERENCES registration_form_field(registration_id, name)
);

CREATE TABLE price (
  id TEXT,
  event_id TEXT NOT NULL,
  name TEXT NOT NULL,
  fee INTEGER NOT NULL,
  currency_code CHAR(3) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY(id),
  UNIQUE(event_id, name),
  FOREIGN KEY(event_id) REFERENCES event(id),
  FOREIGN KEY(currency_code) REFERENCES currency(code)
);
COMMENT ON COLUMN price.event_id IS 'The corresponding identifier of the event.';

CREATE TABLE "session" (
  id TEXT,
  event_id TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  expected_attendees INTEGER NOT NULL,
  start_at TIMESTAMPTZ NOT NULL,
  end_at TIMESTAMPTZ NOT NULL,
  actual_start_at TIMESTAMPTZ,
  actual_end_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY(id),
  UNIQUE(event_id, name),
  FOREIGN KEY(event_id) REFERENCES event(id)
);

CREATE TABLE attendance (
  session_id TEXT,
  user_id TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY(session_id, user_id),
  FOREIGN KEY(session_id) REFERENCES "session"(id)
);

-- migrate:down
DROP TABLE attendance;
DROP TABLE "session";
DROP TABLE price;
DROP TABLE registration_form_field_data;
DROP TABLE registration_form_field;
DROP TABLE registration;
DROP TABLE form_field_type;
DROP TABLE event;
DROP TABLE event_type;
DROP TABLE currency;

DROP TABLE user_ministry;
DROP TABLE user_connect_group;
DROP TABLE ministry_role;
DROP TABLE pastoral_role;
DROP TABLE "user";
DROP TABLE ministry;
DROP TABLE ministry_department;
DROP TABLE ministry_team;
DROP TABLE connect_group;
DROP TABLE satellite;
DROP DOMAIN address;
DROP TYPE _address;
DROP TYPE gender;
