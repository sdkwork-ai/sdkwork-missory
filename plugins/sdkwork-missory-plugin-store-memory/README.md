# sdkwork-missory-plugin-store-memory

In-memory implementation of the `sdkwork-missory-spi` store ports. Phase-1
persistence adapter: owner-scoped maps with snowflake-style id minting. The
phase-2 SQLx/PostgreSQL repository replaces this adapter behind the same ports.

Canonical specs: `../../../sdkwork-specs/APPLICATION_LAYERED_ARCHITECTURE_SPEC.md`.
