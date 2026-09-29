//! Second night pass: neighbor retell. A miss vs core is pulled, not written.

use crate::core::model::{
    now_secs, Attribution, Channel, DriftEvent, DriftKind, MemoryTrace, TraceStatus,
};
use crate::core::profile::EntityProfile;
use crate::core::store::MemoryStore;
use crate::encode::embed::Embedder;
use crate::recall::narrator::Narrator;

pub fn run(
    store: &mut MemoryStore,
    profile: &EntityProfile,
    narrator: &dyn Narrator,
    embedder: &dyn Embedder,
    ground: bool,
) -> u32 {
    let ids = store.active_ids();
    let mut rewritten = 0u32;
    let mut budget = 6u32;
    for id in ids {
        if budget == 0 {
            break;
        }
        let (channel, _anchor, schema, embedding, status) = {
            let Some(t) = store.traces.get(&id) else { continue };
            (
                t.channel,
                t.anchor,
                t.schema.clone(),
                t.embedding.clone(),
                t.status,
            )
        };
        if channel.verbatim() || status == TraceStatus::Latent {
            continue;
        }
        if skip_rewrite(store, &id) {
            continue;
        }
        let neighbors: Vec<crate::core::model::MemoryTrace> = store
            .traces
            .values()
            .filter(|o| o.id != id && o.channel == Channel::Selfhood)
            .filter(|o| {
                if let (Some(a), Some(b)) = (schema.as_ref(), o.schema.as_ref()) {
                    if a == b {
                        return true;
                    }
                }
                !embedding.is_empty()
                    && !o.embedding.is_empty()
                    && crate::encode::embed::cosine(&embedding, &o.embedding) >= profile.merge_similarity
            })
            .cloned()
            .take(3)
            .collect();
        let neighbor_refs: Vec<&crate::core::model::MemoryTrace> = neighbors.iter().collect();
        let Some(t) = store.traces.get(&id) else { continue };
        let Some(text) = narrator.rewrite(t, &neighbor_refs, profile) else { continue };
        if text.trim().is_empty() || text == t.gist {
            continue;
        }
        let core = t.core.clone();
        if ground
            && crate::recall::ground::is_grounding_miss(&text, &core, profile.ground_min_overlap)
        {
            let rewrite = narrator.recontextualize(t, &core, profile);
            if let Some(tr) = store.traces.get_mut(&id) {
                crate::recall::ground::apply_grounding(tr, profile, &text, &core, Some(rewrite));
            }
            continue;
        }
        if let Some(t) = store.traces.get_mut(&id) {
            t.gist = text.chars().take(280).collect();
            t.embedding = embedder.embed(&t.gist);
            t.drifts.push(DriftEvent {
                kind: DriftKind::Rewrite,
                at: now_secs(),
                note: "consolidation narrative".into(),
                fidelity_delta: -0.02 * (1.0 - t.anchor),
                valence_delta: 0.0,
                disgust_delta: 0.0,
            });
            t.fidelity = (t.fidelity - 0.02 * (1.0 - t.anchor)).max(0.15);
            t.clamp();
            rewritten += 1;
            budget -= 1;
        }
    }
    rewritten
}

pub const CONFLICT_CONGRUENCE: f32 = 0.40;

/// Internal hour that cannot sit in living identity without a rewrite.
pub fn is_conflict(store: &MemoryStore, trace: &MemoryTrace) -> bool {
    if trace.attribution != Attribution::Internal {
        return false;
    }
    crate::encode::measure_congruence(store, trace.schema.as_deref(), trace.valence)
        < CONFLICT_CONGRUENCE
}

/// Weather/sculpt may still drop detail. `true` = keep the current gist wording.
pub fn hold_gist_text(trace: &MemoryTrace, conflict: bool) -> bool {
    match trace.attribution {
        Attribution::External => false,
        Attribution::Internal => !conflict,
        Attribution::None => {
            trace.self_relevance >= 0.80 && trace.valence.abs() >= 0.40
        }
    }
}

pub fn skip_rewrite(store: &MemoryStore, id: &str) -> bool {
    let Some(t) = store.traces.get(id) else {
        return true;
    };
    if t.channel.verbatim() || t.status == TraceStatus::Latent {
        return true;
    }
    match t.attribution {
        Attribution::External => true,
        Attribution::Internal => !is_conflict(store, t),
        Attribution::None => skip_rewrite_legacy(store, t),
    }
}

fn skip_rewrite_legacy(store: &MemoryStore, t: &MemoryTrace) -> bool {
    let charged = t.self_relevance >= 0.80 && t.valence.abs() >= 0.40;
    if charged || t.permanence >= 0.92 || t.anchor >= 0.80 {
        return true;
    }
    if !store.living_axiom_ids_for(&t.id).is_empty() {
        return true;
    }
    let Some(schema) = t.schema.as_deref() else {
        return false;
    };
    let charged_n = store
        .traces
        .values()
        .filter(|o| {
            o.schema.as_deref() == Some(schema)
                && o.channel == Channel::Selfhood
                && o.self_relevance >= 0.80
                && o.valence.abs() >= 0.40
        })
        .count();
    charged_n >= 2
}
