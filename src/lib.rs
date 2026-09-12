pub mod benchmark;
pub mod config;
pub mod core;
pub mod dream;
pub mod encode;
pub mod experiment;
pub mod lexicon;
pub mod net;
pub mod persist;
pub mod recall;

mod engine;

pub use core::model::{
    ArchiveRecord, AxiomLayer, Channel, DriftEvent, DriftKind, IdentityAxiom, MemoryTrace, Mood,
    RecalledMemory, TraceStatus,
};
pub use core::talk::{TalkTurn, WorkingTalk};
pub use core::profile::{EntityProfile, Voice};
pub use core::store::MemoryStore;
pub use dream::{detail_retention, fingerprint, seed_anchor, stability_days, DreamReport, Fingerprint};
pub use dream::singularite::distance as singularity_distance;
pub use encode::embed::{cosine, Embedder, HashEmbedder, HttpEmbedder};
pub use encode::{EncodeDecision, EncodeInput};
pub use engine::SelectiveMemory;
pub use benchmark::{
    h2_holds, run_v01, run_v01_n, v01_script, Arm, Campaign, Condition, PairReport, V01Script,
};
pub use experiment::{run_neutral, run_neutral_llm, run_salient, run_salient_llm, run_salient_without_sleep, run_erasure, run_split_lives, script, split_script, BifurcationReport, ErasureReport, LlmSpec};
pub use config::Config;
pub use net::api;
pub use recall::{HttpNarrator, Narrator, RuleNarrator, SpeakOnlyHttp};
pub use HttpNarrator as LLMNarrator;
