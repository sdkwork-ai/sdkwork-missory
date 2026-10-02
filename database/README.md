# missory database module

Authoritative PostgreSQL module for sdkwork-missory (`databaseRole: authoritative-server`).
Baseline DDL: `ddl/baseline/postgres/0001_missory_baseline.sql` (baseline-plus-migrations);
`migrations/postgres/` is reserved for post-GA changes. Table prefix: `missory_`.

Lifecycle commands run through the sdkwork-database CLI (root `pnpm db:*` scripts).
Runtime store: `sdkwork-communication-missory-repository-sqlx` implementing the
`sdkwork-missory-spi` ports.
