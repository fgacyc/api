-- migrate:up
CREATE TABLE user_relationship (
  source_user_id TEXT,
  destination_user_id TEXT,
  relationship TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (source_user_id, destination_user_id),
  FOREIGN KEY (source_user_id) REFERENCES "user"(id),
  FOREIGN KEY (destination_user_id) REFERENCES "user"(id),
  CONSTRAINT source_destination_check CHECK (source_user_id != destination_user_id)
);

-- migrate:down
DROP TABLE user_relationship;
