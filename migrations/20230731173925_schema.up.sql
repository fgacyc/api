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
  nickname TEXT,
  picture TEXT,
  ic_number TEXT,
  phone_number TEXT,
  PRIMARY KEY (id),
  UNIQUE (email),
  FOREIGN KEY (gender) REFERENCES gender(gender) ON UPDATE CASCADE
);

-- To start the id from 80
ALTER SEQUENCE user_id_seq RESTART WITH 100; 

