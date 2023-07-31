-- Add up migration script here
CREATE TABLE gender (
  gender TEXT PRIMARY KEY
);
INSERT INTO gender (gender) VALUES
  ('male'),
  ('female');

CREATE TABLE "user" (
  id SERIAL,
  email TEXT NOT NULL,
  username TEXT NOT NULL,
  given_name TEXT NOT NULL,
  family_name TEXT NOT NULL,
  name TEXT NOT NULL,
  gender TEXT NOT NULL,
  ic_number TEXT NOT NULL,
  phone_number TEXT NOT NULL,
  nickname TEXT,
  picture TEXT,
  cg_id INTEGER,
  PRIMARY KEY (id),
  UNIQUE (email),
  FOREIGN KEY (gender) REFERENCES gender(gender) ON UPDATE CASCADE,
  FOREIGN KEY (cg_id) REFERENCES cg(id) ON UPDATE CASCADE ON DELETE SET NULL
);

-- To start the id from 80
ALTER SEQUENCE user_id_seq RESTART WITH 100; 

CREATE TABLE cg (
  id SERIAL,
  name TEXT NOT NULL,
  PRIMARY KEY (id),
  UNIQUE (name)
);

