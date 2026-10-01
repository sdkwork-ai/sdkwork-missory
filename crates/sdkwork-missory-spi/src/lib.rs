//! Store ports and owner-scoped persistence boundary for SDKWork Missory.
//!
//! Record and query types are reused from `sdkwork-missory-contract` (a pure
//! serde crate) so the service never maps between duplicate shapes. Persistence
//! adapters implement [`ports::MissoryStore`].

pub mod error;
pub mod ports;

pub use error::{MissoryStoreError, MissoryStoreResult};
pub use ports::{
    MissoryScope, MissoryStore, ReminderStateRecord, ReminderStateStatus, SocialTextModel,
    SocialTextPrompt,
};
