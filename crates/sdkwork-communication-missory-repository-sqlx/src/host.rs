//! Database host: manifest engine admission, PG15 gate, lifecycle orchestration.
//!
//! Mirrors the fleet `*-database-host` pattern: `DatabaseConfig::from_env` →
//! `create_pool_from_config` → manifest engine admission → PG15 gate →
//! `LifecycleOrchestrator` init + migrate (gated by `SDKWORK_DATABASE_AUTO_MIGRATE`).

use std::sync::Arc;

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_lifecycle::{lifecycle_options_from_env, LifecycleOrchestrator};
use sdkwork_database_spi::DefaultDatabaseModule;
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool};

/// Bootstrapped database host: pool + resolved module.
pub struct MissoryDatabaseHost {
    pool: DatabasePool,
    module: Arc<DefaultDatabaseModule>,
}

impl MissoryDatabaseHost {
    /// Process-shared pool for the missory module.
    pub fn pool(&self) -> &DatabasePool {
        &self.pool
    }

    /// Resolved database module (manifest-backed).
    pub fn module(&self) -> Arc<DefaultDatabaseModule> {
        self.module.clone()
    }
}

fn resolve_app_root() -> String {
    std::env::var("SDKWORK_MISSORY_APP_ROOT").unwrap_or_else(|_| ".".to_owned())
}

/// Bootstraps the database host from `SDKWORK_DATABASE_*` environment.
pub async fn bootstrap_missory_database_from_env() -> Result<MissoryDatabaseHost, String> {
    let config =
        DatabaseConfig::from_env("MISSORY").map_err(|error| format!("database config: {error}"))?;
    let pool = create_pool_from_config(config)
        .await
        .map_err(|error| format!("create missory database pool failed: {error}"))?;
    bootstrap_missory_database(pool).await
}

/// Bootstraps the database host over an existing pool.
pub async fn bootstrap_missory_database(pool: DatabasePool) -> Result<MissoryDatabaseHost, String> {
    let app_root = resolve_app_root();
    let module = Arc::new(
        DefaultDatabaseModule::from_app_root(&app_root)
            .map_err(|error| format!("read missory database module failed: {error}"))?,
    );
    let manifest = module.manifest();
    if !manifest.engines.contains(&pool.engine().to_string()) {
        return Err(format!(
            "missory manifest declares engines {:?} but the resolved pool engine is {}",
            manifest.engines,
            pool.engine()
        ));
    }
    // With the default feature set the pool is PostgreSQL-only; the sqlite
    // feature is a desktop/test opt-in and never reaches a server role.
    #[allow(irrefutable_let_patterns)]
    if let DatabasePool::Postgres(pg, _context) = &pool {
        assert_postgres_server_version(pg).await?;
    }
    let options = lifecycle_options_from_env("MISSORY", manifest);
    let orchestrator =
        LifecycleOrchestrator::new(pool.clone(), module.clone()).with_applied_by("sdkwork-missory");
    orchestrator
        .init()
        .await
        .map_err(|error| format!("missory database init failed: {error}"))?;
    if options.auto_migrate {
        orchestrator
            .migrate()
            .await
            .map_err(|error| format!("missory database migrate failed: {error}"))?;
    }
    Ok(MissoryDatabaseHost { pool, module })
}

/// Enforces the PostgreSQL 15+ server baseline (`DATABASE_SPEC.md` section 5.2).
///
/// `SHOW server_version_num` returns TEXT on some PostgreSQL-compatible
/// servers, so the value is read as a string and parsed.
pub async fn assert_postgres_server_version(pool: &sqlx::PgPool) -> Result<(), String> {
    let raw: String = sqlx::query_scalar("SHOW server_version_num")
        .fetch_one(pool)
        .await
        .map_err(|error| format!("read postgres server version failed: {error}"))?;
    let version: i64 = raw
        .trim()
        .parse()
        .map_err(|error| format!("parse postgres server_version_num '{raw}' failed: {error}"))?;
    if version < 150_000 {
        return Err(format!(
            "missory requires PostgreSQL 15+ (server_version_num {version} < 150000)"
        ));
    }
    Ok(())
}
