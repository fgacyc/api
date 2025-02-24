-- migrate:up
ALTER TABLE connect_group ADD COLUMN image_url TEXT;

-- migrate:down
ALTER TABLE connect_group DROP COLUMN image_url;

