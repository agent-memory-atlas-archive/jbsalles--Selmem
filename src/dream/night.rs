//! One night: the five passes, in this order.
//! No LLM is required. A narrator, if present, only rewrites gists.

use crate::core::model::{DriftEvent, IdentityAxiom, OrganCut, TraceStatus};
use crate::core::profile::EntityProfile;
use crate::core::store::MemoryStore;
use crate::dream::{ladder, merge, release, rewrite, singularite, weather};
use crate::encode::embed::Embedder;
use crate::recall::narrator::Narrator;

/// Scientific order. A swap changes the book; unit tests on traces will not see it.
pub const NIGHT_PASSES: &[&str] = &["weather", "rewrite", "merge", "ladder", "release"];

pub struct DreamReport {
    pub faded: u32,
    pub cold: u32,
    pub myth: u32,
    pub merged: u32,
    pub extinguished: u32,
    pub weathered: u32,
    pub rewritten: u32,
    pub released: u32,
    pub sculpted: Vec<DriftEvent>,
    pub axioms: Vec<IdentityAxiom>,
}

pub fn dream(
    store: &mut MemoryStore,
    profile: &EntityProfile,
    narrator: &dyn Narrator,
    embedder: &dyn Embedder,
) -> DreamReport {
    dream_cut(store, profile, narrator, embedder, OrganCut::full())
}

pub fn dream_cut(
    store: &mut MemoryStore,
    profile: &EntityProfile,
    narrator: &dyn Narrator,
    embedder: &dyn Embedder,
    cut: OrganCut,
) -> DreamReport {
    let previously_latent: std::collections::HashSet<String> = store
        .traces
        .values()
        .filter(|t| t.status == TraceStatus::Latent)
        .map(|t| t.id.clone())
        .collect();

    singularite::apply_anchors(store);
    let w = weather::run(store, profile);
    let rewritten = rewrite::run(store, profile, narrator, embedder, cut.ground);
    let merged = merge::run(store, profile);
    let axioms = if cut.ladder {
        ladder::run(store, narrator)
    } else {
        Vec::new()
    };
    let released = release::run(store, &previously_latent);
    singularite::apply_anchors(store);

    DreamReport {
        faded: w.faded,
        cold: w.cold,
        myth: w.myth,
        merged,
        extinguished: w.extinguished,
        weathered: w.weathered,
        rewritten,
        released,
        sculpted: w.sculpted,
        axioms,
    }
}
