pub mod core;
pub mod dream;
pub mod encode;
pub mod net;
pub mod persist;
pub mod recall;

mod engine;

pub use core::model::{
    ArchiveRecord, AxiomLayer, Channel, DriftEvent, DriftKind, IdentityAxiom, MemoryTrace, Mood,
    RecalledMemory, TraceStatus,
};
pub use core::profile::EntityProfile;
pub use core::store::MemoryStore;
pub use dream::{detail_retention, fingerprint, seed_anchor, stability_days, DreamReport, Fingerprint};
pub use dream::singularite::distance as singularity_distance;
pub use encode::embed::{cosine, Embedder, HashEmbedder, HttpEmbedder};
pub use encode::{EncodeDecision, EncodeInput};
pub use engine::SelectiveMemory;
pub use net::api;
pub use recall::{HttpNarrator, Narrator, RuleNarrator};
pub use HttpNarrator as LLMNarrator;
