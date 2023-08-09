# FGACYC Identity

This API service is used to manage every FGACYC user's identity. Several features are provided by this service, such as:

- Users registration (Only registration because login will be done through Auth0)
- Users and connect groups management
- Users and ministry management

## Database

At the moment, we are using [Neon](https://neon.tech/)'s serverless postgres database service. The database is hosted on AWS and the schema is managed using [sqlx cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli).

However, the term `database` does not really refer to a single database. Instead, it refers to a `cluster` because although we have a single cluster, we may have multiple SQL `databases` eg. `identity` and `attendance`. Refers to the migration scripts in [migrations](migrations).

### Entity Relation Diagram (ERD)

The following diagram depicts the relationship between the entities within the database. The diagram is generated using [Mermaid](https://mermaid-js.github.io/mermaid/#/).

### Identity Database

There are only 3 entities here:

- **User** - The user entity that is used for authentication and authorization.
- **Bot** - The bot entity that represents a trading bot.
- **Secret** - The secret entity that represents a secret eg. API Keys that is used by one or multiple bots.

