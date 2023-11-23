-- Add up migration script here
ALTER TABLE "user" ADD "deleted_at" TIMESTAMPTZ DEFAULT NULL;