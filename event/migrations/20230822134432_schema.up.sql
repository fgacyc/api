-- Add up migration script here
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
