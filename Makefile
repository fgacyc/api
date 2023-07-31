include .env
export

pgcli: # Connect to the db using pgcli
	pgcli $(DATABASE_URL)
