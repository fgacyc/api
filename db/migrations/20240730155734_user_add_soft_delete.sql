-- migrate:up
ALTER TABLE "user" ADD COLUMN deleted BOOLEAN NOT NULL DEFAULT FALSE;

-- migrate:down
ALTER TABLE "user" DROP COLUMN deleted;

