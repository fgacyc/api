## Setup the `pg_graphql` server

### 1. Install `pg_graphql` onto the postgres instance

```sql
create extension pg_graphql;

create function public.graphql(
    "operationName" text default null,
    query text default null,
    variables jsonb default null
)
    returns jsonb
    language sql
as $$
    select graphql.resolve(query, variables);
$$;

create role anon noinherit;
create role authenticator noinherit;
grant anon to authenticator;
```

### 2. Hosting the server

Make sure you have the correct `.env`, then run `docker compose up -d`. Refer to `.env.example` for the env you need to have.

Also, remember to update the url in `docker/index.html` in case the url changes
