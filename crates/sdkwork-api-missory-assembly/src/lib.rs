//! API assembly for sdkwork-missory.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.

pub mod bootstrap;
mod generated;

pub use bootstrap::{
    assemble_api_assembly_from_env, assemble_api_router, assemble_api_router_from_env, ApiAssembly,
    MissoryReadinessCheck, OPENAPI_YAML, PERMISSION_CATALOG, ROUTE_MANIFEST_JSON,
};

/// Number of route crates composed into this assembly.
pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
