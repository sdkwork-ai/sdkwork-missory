//! Composition root for the Missory HTTP plane.
//!
//! Builds the complete assembly contribution (router + route manifest + OpenAPI
//! authority + permission catalog + context injectors + readiness check). The
//! standalone gateway consumes these builders; gateways must never hand-merge
//! route crates (API_ASSEMBLY_SPEC §10).

use std::sync::Arc;

use axum::Router;
use sdkwork_communication_missory_repository_sqlx::{
    missory_store_from_env, MissoryStoreSelection,
};
use sdkwork_communication_missory_service::MissoryService;
use sdkwork_routes_missory_app_api as missory_routes;

/// Materialized route manifest shipped with the assembly.
pub const ROUTE_MANIFEST_JSON: &str = include_str!(
    "../../../sdks/_route-manifests/app-api/sdkwork-routes-missory-app-api.route-manifest.json"
);

/// Authored OpenAPI authority shipped with the assembly.
pub const OPENAPI_JSON: &str =
    include_str!("../../../apis/app-api/communication/missory-app-api.openapi.json");

/// Permission codes owned by the Missory app-api surface (phase 1: owner-scoped
/// personal data only; IAM permission tiers land with the web-framework adapter).
pub const PERMISSION_CATALOG: &[&str] = &["missory.personal-data.owner"];

/// Context injectors contributing the request identity before the IAM adapter.
pub const DOMAIN_CONTEXT_INJECTORS: &[&str] = &["gateway-header-context-injector"];

/// Readiness probe for the assembled plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissoryReadinessCheck {
    /// Component reported by the probe.
    pub component: &'static str,
}

impl MissoryReadinessCheck {
    /// Phase-1 readiness: the in-memory store is always ready once assembled.
    pub fn is_ready(&self) -> bool {
        true
    }
}

/// Complete API assembly contribution for sdkwork-missory.
pub struct ApiAssembly {
    /// Router composing every owned route crate.
    pub router: Router,
    /// Route inventory manifest (materialized, never hand-written).
    pub route_manifest: &'static str,
    /// OpenAPI authority document.
    pub openapi: &'static str,
    /// Owned permission codes.
    pub permission_catalog: &'static [&'static str],
    /// Domain context injectors registered by the host.
    pub domain_context_injectors: &'static [&'static str],
    /// Readiness probe.
    pub readiness_check: MissoryReadinessCheck,
}

impl ApiAssembly {
    /// Builds the assembly around an existing service.
    pub fn new(service: Arc<MissoryService>) -> Self {
        Self {
            router: assemble_api_router(service),
            route_manifest: ROUTE_MANIFEST_JSON,
            openapi: OPENAPI_JSON,
            permission_catalog: PERMISSION_CATALOG,
            domain_context_injectors: DOMAIN_CONTEXT_INJECTORS,
            readiness_check: MissoryReadinessCheck {
                component: "sdkwork-missory",
            },
        }
    }
}

/// Composes the application router from the owned route crates.
///
/// Infrastructure health routes (`/healthz`, `/livez`, `/readyz`, `/metrics`)
/// are mounted through the shared web-framework service router — gateways must
/// not fork local health handlers (HEALTH_CHECK_SPEC §11). Phase 1 reports
/// always-ready because the in-memory store has no external dependencies; the
/// SQLx adapter swaps in a real readiness probe.
pub fn assemble_api_router(service: Arc<MissoryService>) -> Router {
    let business_router = assemble_business_router(service);
    sdkwork_web_bootstrap::service_router(
        business_router,
        sdkwork_web_bootstrap::ServiceRouterConfig::default().with_always_ready(),
    )
}

/// Composes the business-only router (no infrastructure health routes).
///
/// The standalone gateway uses this so its request-context middleware applies
/// to business routes only — `/healthz` and `/readyz` stay unauthenticated
/// (HEALTH_CHECK_SPEC §11).
pub fn assemble_business_router(service: Arc<MissoryService>) -> Router {
    missory_routes::gateway_mount(service)
}

/// Composes the application router from environment configuration.
///
/// The store is selected fail-closed (`missory_store_from_env`): production-like
/// environments require PostgreSQL and refuse the in-memory adapter.
pub async fn assemble_api_router_from_env() -> Result<Router, String> {
    let (store, selection) = missory_store_from_env().await?;
    tracing::info!(?selection, "missory store selected");
    let service = missory_service_from_store(store);
    Ok(assemble_api_router(service))
}

/// Composes the business-only router from environment configuration.
pub async fn assemble_business_router_from_env() -> Result<Router, String> {
    let (store, selection) = missory_store_from_env().await?;
    tracing::info!(?selection, "missory store selected");
    let service = missory_service_from_store(store);
    Ok(assemble_business_router(service))
}

fn missory_service_from_store(
    store: Arc<dyn sdkwork_missory_spi::MissoryStore>,
) -> Arc<MissoryService> {
    // Phase 2: resolve a configured SocialTextModel provider here.
    Arc::new(MissoryService::new(store, None))
}

/// `db-migrate` argv mode: server-role engine admission, then a forced
/// lifecycle migration against the configured PostgreSQL profile.
/// Returns the process exit code (0 on success).
pub async fn run_database_migrate_only() -> u8 {
    if let Err(error) =
        sdkwork_communication_missory_repository_sqlx::ensure_server_role_database_engine_from_env()
    {
        tracing::error!("{error}");
        return 2;
    }
    let result =
        sdkwork_communication_missory_repository_sqlx::missory_store_from_env_with_migrate(true)
            .await;
    match result {
        Ok((_, MissoryStoreSelection::Postgres)) => {
            tracing::info!("missory database migration completed");
            0
        }
        Ok((_, selection)) => {
            tracing::error!("db-migrate requires the PostgreSQL store (selected {selection:?})");
            2
        }
        Err(error) => {
            tracing::error!("db-migrate failed: {error}");
            2
        }
    }
}

/// Assembles the complete contribution from environment configuration.
pub async fn assemble_api_assembly_from_env() -> Result<ApiAssembly, String> {
    let (store, selection) = missory_store_from_env().await?;
    tracing::info!(?selection, "missory store selected");
    Ok(ApiAssembly::new(missory_service_from_store(store)))
}
