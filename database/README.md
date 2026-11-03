# missory database module

Authoritative PostgreSQL module for sdkwork-missory (`databaseRole: authoritative-server`).
Baseline DDL: `ddl/baseline/postgres/0001_missory_baseline.sql` (baseline-plus-migrations);
`migrations/postgres/` is reserved for post-GA changes. Table prefix: `missory_`.

Lifecycle commands run through the sdkwork-database CLI (root `pnpm db:*` scripts).
Runtime store: `sdkwork-communication-missory-repository-sqlx` implementing the
`sdkwork-missory-spi` ports.

## Initialization state

The module uses `baselineStrategy: baseline-plus-migrations`:

1. **Baseline** — `ddl/baseline/postgres/0001_missory_baseline.sql` is the authoritative
   greenfield DDL snapshot.
2. **Migrations** — `migrations/postgres/` carries ordered, idempotent post-baseline
   changes for already-initialized databases. Applied migrations are never edited.
3. **Drift** — run `pnpm db:drift:check` before release.

## Commands

```bash
pnpm db:validate            # framework standard check
pnpm db:plan                # show pending lifecycle work
pnpm db:init                # create schema and apply the baseline
pnpm db:migrate             # apply migrations
pnpm db:seed                # apply the default seed profile and locale
pnpm db:status              # applied revisions and state
pnpm db:drift:check         # compare live schema with the contract
```
