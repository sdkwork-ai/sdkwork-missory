//! Resolved request identity shared by the route, service, and store layers.

/// Authenticated owner identity resolved before handlers run.
///
/// The tenant/user ids are never accepted from create/update bodies
/// (`API_SPEC.md` section 15.2); they are resolved from the request context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissoryRequestContext {
    /// Tenant the request operates in.
    pub tenant_id: u64,
    /// Organization scope inside the tenant (`0` for personal workspaces).
    pub organization_id: u64,
    /// Authenticated owner user id. All Missory data is scoped to this owner.
    pub user_id: u64,
}

impl MissoryRequestContext {
    /// Builds a context from explicit ids (used by adapters and tests).
    pub fn new(tenant_id: u64, organization_id: u64, user_id: u64) -> Self {
        Self {
            tenant_id,
            organization_id,
            user_id,
        }
    }
}
