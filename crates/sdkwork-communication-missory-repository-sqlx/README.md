# sdkwork-communication-missory-repository-sqlx

Authoritative PostgreSQL store for SDKWork Missory: implements the
`sdkwork-missory-spi` store ports over sqlx (PostgreSQL only, per
`DATABASE_SPEC.md` section 7.2 — SQLite never serves a server role).

Owns the database bootstrap (`database/database.manifest.json` engine
admission, PG15 gate, lifecycle init/migrate) and the snowflake id generator
allocation. Tables use the `missory_` prefix declared in the module contract.

Canonical specs: `../../../sdkwork-specs/DATABASE_SPEC.md`,
`../../../sdkwork-specs/DATABASE_FRAMEWORK_SPEC.md`.
