# sdkwork-missory-spi

Store ports (persistence boundary) for SDKWork Missory. Persistence adapters
(currently the in-memory store plugin, later a SQLx/PostgreSQL repository) implement
`MissoryStore` against the owner-scoped record types defined here.

Canonical specs: `../../../sdkwork-specs/APPLICATION_LAYERED_ARCHITECTURE_SPEC.md`,
`../../../sdkwork-specs/RUST_CODE_SPEC.md`.
