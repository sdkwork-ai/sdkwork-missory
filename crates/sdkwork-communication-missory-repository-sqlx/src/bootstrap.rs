//! Env-driven store selection and bootstrap entrypoints.
//!
//! Selection rules (fail-closed per `ENVIRONMENT_SPEC.md` section 7.2):
//! - `SDKWORK_MISSORY_STORE=memory` → in-memory store, **development/test only**;
//! - `SDKWORK_MISSORY_STORE=postgres` or `SDKWORK_DATABASE_URL`/`SDKWORK_DATABASE_ENGINE`
//!   present → PostgreSQL store via the sqlx repository;
//! - neither set: production-like environments (staging/demo/production) **refuse to
//!   start**; development/test fall back to the in-memory store.

use std::sync::Arc;

use sdkwork_database_config::DatabaseConfig;
use sdkwork_missory_spi::MissoryStore;

use crate::host::{bootstrap_missory_database_from_env, MissoryDatabaseHost};
use crate::store::SqlxMissoryStore;

/// Which store implementation the process selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissoryStoreSelection {
    /// In-memory adapter (development/test only).
    Memory,
    /// Authoritative PostgreSQL adapter.
    Postgres,
}

fn environment() -> String {
    std::env::var("SDKWORK_MISSORY_ENVIRONMENT")
        .unwrap_or_else(|_| std::env::var("SDKWORK_ENVIRONMENT").unwrap_or_default())
        .to_lowercase()
}

fn is_production_like(environment: &str) -> bool {
    matches!(environment, "staging" | "demo" | "production" | "prod")
}

fn is_development_like(environment: &str) -> bool {
    matches!(environment, "" | "development" | "dev" | "test" | "testing")
}

fn database_env_declared() -> bool {
    std::env::var("SDKWORK_DATABASE_URL").is_ok()
        || std::env::var("SDKWORK_DATABASE_ENGINE").is_ok()
}

/// Resolves which store the process must use; production-like environments
/// without an explicit database configuration are rejected (fail-closed).
pub fn select_missory_store_kind() -> Result<MissoryStoreSelection, String> {
    let environment = environment();
    match std::env::var("SDKWORK_MISSORY_STORE").as_deref() {
        Ok("memory") => {
            if is_production_like(&environment) {
                return Err(
                    "SDKWORK_MISSORY_STORE=memory is not allowed in a production-like environment; \
                     configure PostgreSQL via SDKWORK_DATABASE_*"
                        .to_owned(),
                );
            }
            Ok(MissoryStoreSelection::Memory)
        }
        Ok(value @ ("postgres" | "postgresql")) => {
            let _ = value;
            Ok(MissoryStoreSelection::Postgres)
        }
        Ok(other) => Err(format!(
            "SDKWORK_MISSORY_STORE must be memory or postgres (got {other})"
        )),
        Err(_) => {
            if database_env_declared() {
                return Ok(MissoryStoreSelection::Postgres);
            }
            if is_development_like(&environment) {
                Ok(MissoryStoreSelection::Memory)
            } else {
                Err(format!(
                    "refusing to start: production-like environment ({environment}) requires \
                     PostgreSQL; set SDKWORK_DATABASE_URL (or SDKWORK_DATABASE_ENGINE) or pin \
                     SDKWORK_MISSORY_STORE explicitly"
                ))
            }
        }
    }
}

/// Server-role engine admission (used by the `db-migrate` argv mode): the
/// SQLite engine is rejected for every non-test deployment mode.
pub fn ensure_server_role_database_engine_from_env() -> Result<(), String> {
    let config =
        DatabaseConfig::from_env("MISSORY").map_err(|error| format!("database config: {error}"))?;
    match config.engine {
        sdkwork_database_config::DatabaseEngine::Postgres => Ok(()),
        sdkwork_database_config::DatabaseEngine::Sqlite => Err(
            "authoritative-server missory rejects the SQLite engine: PostgreSQL is required for \
             every server role; set SDKWORK_DATABASE_URL to the workspace PostgreSQL profile"
                .to_owned(),
        ),
    }
}

/// Builds the missory store from the environment (auto-migrate off).
pub async fn missory_store_from_env(
) -> Result<(Arc<dyn MissoryStore>, MissoryStoreSelection), String> {
    missory_store_from_env_with_migrate(false).await
}

/// Builds the missory store from the environment, optionally running the
/// lifecycle migration after the manifest engine admission.
pub async fn missory_store_from_env_with_migrate(
    auto_migrate: bool,
) -> Result<(Arc<dyn MissoryStore>, MissoryStoreSelection), String> {
    match select_missory_store_kind()? {
        MissoryStoreSelection::Memory => {
            let store = sdkwork_missory_plugin_store_memory::InMemoryMissoryStore::from_env();
            Ok((Arc::new(store), MissoryStoreSelection::Memory))
        }
        MissoryStoreSelection::Postgres => {
            let host = open_postgres_host(auto_migrate).await?;
            let pool = host.pool().as_postgres().cloned().ok_or_else(|| {
                "missory postgres store requires a PostgreSQL pool (engine admission passed but \
                 pool is not postgres)"
                    .to_owned()
            })?;
            let (generator, _lease) =
                sdkwork_database_id::SnowflakeNodeAllocator::allocate_process_generator(
                    host.pool(),
                    &sdkwork_database_id::NodeAllocatorConfig::from_service_name("missory-service"),
                )
                .await
                .map_err(|error| format!("allocate snowflake node failed: {error}"))?;
            let store = SqlxMissoryStore::new(Arc::new(pool), generator);
            Ok((Arc::new(store), MissoryStoreSelection::Postgres))
        }
    }
}

async fn open_postgres_host(auto_migrate: bool) -> Result<MissoryDatabaseHost, String> {
    let previous = std::env::var_os("SDKWORK_DATABASE_AUTO_MIGRATE");
    if auto_migrate {
        std::env::set_var("SDKWORK_DATABASE_AUTO_MIGRATE", "true");
    }
    let result = bootstrap_missory_database_from_env().await;
    match previous {
        Some(value) => std::env::set_var("SDKWORK_DATABASE_AUTO_MIGRATE", value),
        None => std::env::remove_var("SDKWORK_DATABASE_AUTO_MIGRATE"),
    }
    result
}
