//! Missory domain service: business logic over the store ports.
//!
//! The service never touches HTTP types; routes depend only on
//! `sdkwork_missory_contract::ports::MissoryAppApi`.

pub mod assistant;
pub mod extract;
pub mod reminders;
pub mod service;

pub use service::{Clock, MissoryService, DEFAULT_LONG_UNCONTACTED_DAYS};

use std::sync::Arc;

use sdkwork_missory_spi::{MissoryStore, SocialTextModel};

/// Builds a [`MissoryService`] over an injected store (composition root helper).
/// Builds a [`MissoryService`] over an injected store (composition root helper).
pub fn missory_service(
    store: Arc<dyn MissoryStore>,
    model: Option<Arc<dyn SocialTextModel>>,
) -> Arc<MissoryService> {
    Arc::new(MissoryService::new(store, model))
}
