# API

*API Server for FGACYC's Database*

This project aims to build an API layer to interact with FGACYC's underlying PostgreSQL database. It initially started 
off as a REST API implemented in Rust but later due to maintainability issues, we switched over to a more autonomous 
solution with the [pg_graphql](https://github.com/supabase/pg_graphql) extension. 

What this enabled us to do is that we do not need to manually write API endpoints for each table we add onto the database, 
it would automatically instropect our SQL database and generate relevant GraphQL schema for it on-the-fly.

At the moment, you can find the hosted GraphQL API at:

- [GraphIQL Playground](https://graphql-playground.fgacyc.com)
- [GraphIQL Playground (Dev)](https://graphql-playground.development.fgacyc.com)

Note that to access the GraphQL API, an `Authorization: Bearer <access_token>` header is needed, the `access_token` here 
can be generated after an **Auth0** login. The playground already has **Auth0** integrated and after you logged in, it 
will just work directly.

![GraphIQL Example](images/graphiql_example.jpg)

The figure above shows an example query to get the user's information from the database. You can discover more *collections* 
in the playground and note that pagination is natively implemented if required.

To access the GraphQL API directly, following is the endpoints where you can use with client libraries in JavaScript / Dart:

- [GraphQL API](https://graphql.fgacyc.com/rpc/graphql)
- [GraphQL API (Dev)](https://graphql.development.fgacyc.com/rpc/graphql)

Do not hesitate to reach out or open an issue in this repo if you encounter blockers implementing the GraphQL API in your 
application.

---

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
create role fgacyc.official noinherit;
grant anon to fgacyc.official;
```

### 2. Hosting the server

Make sure you have the correct `.env`, then run `docker compose up -d`. Refer to `.env.example` for the env you need to have.

Also, remember to update the url in `docker/index.html` in case the url changes

### 3. Authentication

Refer to [PostgREST Auth](https://docs.postgrest.org/en/v12/references/auth.html) to see how authentication works through 
Json Web Token (JWT). 

In order for that to work, we needed to add a custom claim `role: "fgacyc.official"` to every access token through Auth0 
Actions. The following snippet is what we have now in production:

```js
const event = `post-login`;
const action = `Set custom claims`;
const logInfo = (...args) => console.log(`${event}(${action}): `, ...args);
const logError = (...args) => console.error(`${event}(${action}): `, ...args);

/**
 * Handler that will be called during the execution of a PostLogin flow.
 *
 * @param {Event} event - Details about the user and the context in which they are logging in.
 * @param {PostLoginAPI} api - Interface whose methods can be used to change the behavior of the login.
 */
exports.onExecutePostLogin = async (event, api) => {
  logInfo(
    `${event.user.user_id} (${event.user.email}) from ${event.client.name}(${event.client.client_id})`,
  );

  // Add email into the user claims.
  api.accessToken.setCustomClaim("email", event.user.email);
  api.accessToken.setCustomClaim("role", event.user.app_metadata && 'role' in event.user.app_metadata ? event.user.app_metadata.role : 'fgacyc.official')
  logInfo(`Added 'email' claim onto ${event.user.email}'s access token.`);
  logInfo(`Added 'role' claim onto ${event.user.email}'s access token.`)
};

/**
 * Handler that will be invoked when this action is resuming after an external redirect. If your
 * onExecutePostLogin function does not perform a redirect, this function can be safely ignored.
 *
 * @param {Event} event - Details about the user and the context in which they are logging in.
 * @param {PostLoginAPI} api - Interface whose methods can be used to change the behavior of the login.
 */
// exports.onContinuePostLogin = async (event, api) => {
// };
```
