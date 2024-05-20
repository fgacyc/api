SHELL := /usr/bin/env bash
DB_MIGRATIONS_DIR="./db/migrations"
DB_SCHEMA_FILE="./db/schema.sql"
DBMATE_MIGRATIONS_TABLE="migrations.schema_migrations"

include .env
export

# Connect to the db using pgcli
pgcli:
	pgcli $(DATABASE_URL)

# Create a new migration
migrate-new:
	dbmate -d $(DB_MIGRATIONS_DIR) -s $(DB_SCHEMA_FILE) --migrations-table $(DBMATE_MIGRATIONS_TABLE) new $(NAME)

# Apply latest migration
migrate-up:
	dbmate -d $(DB_MIGRATIONS_DIR) -s $(DB_SCHEMA_FILE) --migrations-table $(DBMATE_MIGRATIONS_TABLE) up

# Rollback latest migration
migrate-down:
	dbmate -d $(DB_MIGRATIONS_DIR) -s $(DB_SCHEMA_FILE) --migrations-table $(DBMATE_MIGRATIONS_TABLE) down

# Check migration status
migrate-status:
	dbmate -d $(DB_MIGRATIONS_DIR) -s $(DB_SCHEMA_FILE) --migrations-table $(DBMATE_MIGRATIONS_TABLE) status

# Exports the current database schema
dump:
	dbmate -d $(DB_MIGRATIONS_DIR) -s $(DB_SCHEMA_FILE) --migrations-table $(DBMATE_MIGRATIONS_TABLE) dump

