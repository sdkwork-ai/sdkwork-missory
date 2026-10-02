//! Authoritative PostgreSQL store for SDKWork Missory.
//!
//! `store.rs` implements the SPI ports; `host.rs` owns the database bootstrap
//! (manifest engine admission, PG15 gate, lifecycle init/migrate, snowflake
//! allocation); `bootstrap.rs` ties them into the env-driven entrypoints the
//! assembly and gateway consume.

pub mod bootstrap;
pub mod host;
/// Audited, compile-time-constant SELECT fragments shared by store queries.
pub mod selects;
pub mod store;

pub use bootstrap::{
    ensure_server_role_database_engine_from_env, missory_store_from_env,
    missory_store_from_env_with_migrate, MissoryStoreSelection,
};
pub use host::{
    assert_postgres_server_version, bootstrap_missory_database,
    bootstrap_missory_database_from_env, MissoryDatabaseHost,
};
pub use store::SqlxMissoryStore;
