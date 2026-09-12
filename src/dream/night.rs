//! One "night": decay detail, maybe rewrite, merge close episodes,
//! mint motifs / beliefs / traits, mark latent traces.
//! No LLM is required. A narrator, if present, only rewrites gists.

use std::collections::HashMap;

use crate::dream::drift::{sculpt, weather};
use crate::encode::embed::Embedder;
use crate::core::model::{
    now_secs, new_id, AxiomLayer, Channel, DriftEvent, DriftKind, IdentityAxiom, TraceStatus,
};
use crate::recall::narrator::Narrator;
use crate::core::profile::EntityProfile;
use crate::encode::scoring::{lexical_similarity, refresh_access};
use crate::dream::singularite;
use crate::core::store::MemoryStore;

pub struct DreamReport {
    pub faded: u32,
    pub cold: u32,
    pub myth: u32,
    pub merged: u32,
    pub extinguished: u32,
    pub weathered: u32,
    pub rewritten: u32,
    pub sculpted: Vec<DriftEvent>,
    pub axioms: Vec<IdentityAxiom>,
}

pub fn dream(
    store: &mut MemoryStore,
    profile: &EntityProfile,
    narrator: &dyn Narrator,
    embedder: &dyn Embedder,
) -> DreamReport {
    let mut faded = 0;
    let mut cold = 0;
    let mut myth = 0;
    let mut sculpted = Vec::new();
    let mut extinguished = 0;
    let mut weathered = 0;
    let ids = store.active_ids();
    singularite::apply_anchors(store);

    for id in &ids {
        {
            let trace = store.traces.get_mut(id).unwrap();
            if trace.channel == Channel::World {
                // Operational facts do not cool, mythologize, or go latent.
                if trace.status != TraceStatus::Sealed {
                    trace.status = TraceStatus::Active;
                }
                trace.access = 1.0;
                continue;
            }
        }
        let event = {
            let trace = store.traces.get_mut(id).unwrap();
            refresh_access(trace, profile);
            if weather(trace, profile).is_some() {
                weathered += 1;
            }
            let unused = match (trace.last_recalled_at, trace.last_consolidated_at) {
                (None, _) => true,
                (Some(r), Some(c)) => r <= c,
                (Some(_), None) => false,
            };
            let ev = if unused && extinguish(trace, profile) {
                extinguished += 1;
                None
            } else {
                sculpt(trace, profile)
            };
            if trace.last_consolidated_at.is_none() {
                trace.last_consolidated_at = Some(trace.created_at);
            } else {
                trace.last_consolidated_at = Some(now_secs());
            }
            ev
        };
        if let Some(ev) = event {
            sculpted.push(ev);
        }
        let trace = store.traces.get_mut(id).unwrap();
        if trace.permanence >= 0.8 {
            continue;
        }
        if trace.channel != Channel::World
            && trace.status != TraceStatus::Sealed
            && trace.fidelity < 0.34
            && trace.access < 0.26
            && (trace.valence.abs() > 0.25 || trace.disgust > 0.22 || trace.schema.is_some())
        {
            if trace.status != TraceStatus::Latent {
                trace.status = TraceStatus::Latent;
            }
        } else if trace.access < profile.myth_access && trace.status != TraceStatus::Myth {
            trace.status = TraceStatus::Myth;
            myth += 1;
        } else if trace.access < profile.cold_access && trace.status == TraceStatus::Active {
            trace.status = TraceStatus::Cold;
            cold += 1;
        } else if trace.access < profile.myth_access / 2.0 {
            faded += 1;
        }
        maybe_revive_latent(trace);
    }

    let rewritten = rewrite_pass(store, profile, narrator, embedder);
    let merged = merge_close(store, profile);
    let mut axioms = extract_axioms(store, narrator);
    axioms.extend(promote_traits(store, narrator));
    singularite::apply_anchors(store);
    DreamReport {
        faded,
        cold,
        myth,
        merged,
        extinguished,
        weathered,
        rewritten,
        sculpted,
        axioms,
    }
}

fn rewrite_pass(
    store: &mut MemoryStore,
    profile: &EntityProfile,
    narrator: &dyn Narrator,
    embedder: &dyn Embedder,
) -> u32 {
    let ids = store.active_ids();
    let mut rewritten = 0u32;
    let mut budget = 6u32;
    for id in ids {
        if budget == 0 {
            break;
        }
        let (channel, anchor, schema, embedding, status) = {
            let Some(t) = store.traces.get(&id) else { continue };
            (
                t.channel,
                t.anchor,
                t.schema.clone(),
                t.embedding.clone(),
                t.status,
            )
        };
        if channel == Channel::World || anchor >= 0.88 || status == TraceStatus::Latent {
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
        if crate::recall::ground::overlap_with_core(&text, &core) < profile.ground_min_overlap {
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

fn extinguish(trace: &mut crate::core::model::MemoryTrace, profile: &EntityProfile) -> bool {
    if trace.channel == Channel::World || trace.disgust < 0.08 {
        return false;
    }
    let unused = match (trace.last_recalled_at, trace.last_consolidated_at) {
        (None, _) => true,
        (Some(r), Some(c)) => r <= c,
        (Some(_), None) => false,
    };
    if !unused {
        return false;
    }
    let before = trace.disgust;
    trace.disgust = (trace.disgust * (1.0 - profile.extinction_rate)).max(0.0);
    if (before - trace.disgust).abs() > 0.001 {
        trace.drifts.push(DriftEvent {
            kind: DriftKind::Fade,
            at: now_secs(),
            note: "extinction lente du dégoût".into(),
            fidelity_delta: 0.0,
            valence_delta: 0.0,
            disgust_delta: trace.disgust - before,
        });
        true
    } else {
        false
    }
}

/// The charge was lived again. The original scene does not return:
/// only the core, as a cold blur.
fn maybe_revive_latent(trace: &mut crate::core::model::MemoryTrace) {
    if trace.status != TraceStatus::Latent {
        return;
    }
    if trace.rehearsals < 2 {
        return;
    }
    if !trace.core.is_empty() {
        trace.gist = trace.core.clone();
    }
    trace.status = TraceStatus::Cold;
    trace.fidelity = trace.fidelity.max(0.40).min(0.55);
    trace.access = trace.access.max(0.22);
    trace.detach_strikes = 0;
}

fn merge_weight(trace: &crate::core::model::MemoryTrace) -> (i32, i32, i32, String) {
    // Higher numeric key wins. Text is a content tie-break so two clones
    // that lived the same hours keep the same survivor, not the smaller id.
    let status_penalty = match trace.status {
        TraceStatus::Active => 0,
        TraceStatus::Cold => -1,
        TraceStatus::Latent => -3,
        TraceStatus::Myth => -8,
        TraceStatus::Sealed => -20,
    };
    let text = if trace.core.is_empty() {
        trace.gist.clone()
    } else {
        trace.core.clone()
    };
    (
        (trace.anchor * 1000.0) as i32 + status_penalty * 1000,
        (trace.permanence * 1000.0) as i32,
        (trace.fidelity * 1000.0) as i32,
        text,
    )
}

fn fuse_core(keeper: &str, absorbed: &str) -> String {
    let keeper = keeper.trim();
    let absorbed = absorbed.trim();
    if absorbed.is_empty() || keeper.contains(absorbed) {
        return keeper.to_string();
    }
    if keeper.is_empty() || absorbed.contains(keeper) {
        return absorbed.to_string();
    }
    // Keeper remains the semantic reference; distinctive words from the
    // absorbed episode stay available for later grounding.
    let extra: Vec<&str> = absorbed
        .split_whitespace()
        .filter(|w| {
            let w = w.trim_matches(|c: char| !c.is_alphanumeric());
            w.chars().count() > 2 && !keeper.to_lowercase().contains(&w.to_lowercase())
        })
        .take(4)
        .collect();
    if extra.is_empty() {
        keeper.to_string()
    } else {
        format!("{keeper} {}", extra.join(" "))
            .chars()
            .take(180)
            .collect()
    }
}

fn fuse_gist(a: &str, b: &str) -> String {
    let left = a.split(" / ").next().unwrap_or(a).trim();
    let right = b.split(" / ").next().unwrap_or(b).trim();
    if left == right {
        format!("{left}, devenu un mythe.")
    } else {
        format!("{left} / {right}")
    }
}

fn merge_close(store: &mut MemoryStore, profile: &EntityProfile) -> u32 {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for t in store.traces.values() {
        if t.channel != Channel::Selfhood || t.status == TraceStatus::Sealed {
            continue;
        }
        if let Some(s) = &t.schema {
            groups.entry(s.clone()).or_default().push(t.id.clone());
        }
    }

    let mut merged = 0u32;
    for (_schema, mut ids) in groups {
        if ids.len() < 2 {
            continue;
        }
        ids.sort();
        let Some(keep) = ids
            .iter()
            .filter_map(|id| store.traces.get(id).map(|t| (id.clone(), merge_weight(t))))
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(id, _)| id)
        else {
            continue;
        };
        for other in ids.iter().filter(|id| *id != &keep) {
            let similar = {
                let a = store.traces.get(&keep);
                let b = store.traces.get(other);
                match (a, b) {
                    (Some(a), Some(b)) => {
                        if b.status == TraceStatus::Myth || (a.anchor >= 0.8 && b.anchor >= 0.8) {
                            false
                        } else if !a.embedding.is_empty() && !b.embedding.is_empty() {
                            crate::encode::embed::cosine(&a.embedding, &b.embedding)
                                >= profile.merge_similarity
                        } else {
                            lexical_similarity(&a.gist, &b.gist) >= profile.merge_similarity
                        }
                    }
                    _ => false,
                }
            };
            if !similar {
                continue;
            }
            let other_clone = store.traces.get(other).cloned();
            let Some(src) = other_clone else { continue };
            if let Some(dst) = store.traces.get_mut(&keep) {
                dst.gist = fuse_gist(&dst.gist, &src.gist);
                dst.core = fuse_core(&dst.core, &src.core);
                dst.valence = (dst.valence + src.valence) / 2.0;
                dst.disgust = dst.disgust.max(src.disgust);
                dst.arousal = dst.arousal.max(src.arousal);
                dst.anchor = dst.anchor.max(src.anchor);
                dst.permanence = dst.permanence.max(src.permanence);
                dst.fidelity = (dst.fidelity.min(src.fidelity) * 0.85).max(0.15);
                dst.self_relevance = dst.self_relevance.max(src.self_relevance);
                for c in src.cues {
                    if !dst.cues.iter().any(|x| x == &c) {
                        dst.cues.push(c);
                    }
                }
                dst.drifts.push(DriftEvent {
                    kind: DriftKind::Merge,
                    at: now_secs(),
                    note: format!("fusion de {}", src.id),
                    fidelity_delta: -0.05,
                    valence_delta: 0.0,
                    disgust_delta: 0.0,
                });
                dst.clamp();
            }
            if let Some(src_mut) = store.traces.get_mut(other) {
                src_mut.status = TraceStatus::Myth;
            }
            store.link(&keep, other);
            merged += 1;
        }
    }
    merged
}

fn extract_axioms(store: &mut MemoryStore, narrator: &dyn Narrator) -> Vec<IdentityAxiom> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for t in store.traces.values() {
        // Forgotten scenes still color encoding. They must not mint new beliefs.
        if matches!(t.status, TraceStatus::Active | TraceStatus::Cold) {
            if let Some(s) = &t.schema {
                groups.entry(s.clone()).or_default().push(t.id.clone());
            }
        }
    }
    let existing: Vec<String> = store
        .living_axioms()
        .into_iter()
        .map(|a| a.statement.clone())
        .collect();

    let mut created = Vec::new();
    for (schema, ids) in groups {
        if ids.len() < 2 {
            continue;
        }
        let traces: Vec<&crate::core::model::MemoryTrace> = ids
            .iter()
            .filter_map(|id| store.traces.get(id))
            .collect();
        let Some(statement) = narrator.distill_axiom(&traces) else {
            continue;
        };
        if existing.iter().any(|s| s == &statement) {
            continue;
        }
        let prev_id = store
            .living_axioms()
            .into_iter()
            .find(|a| a.schema.as_deref() == Some(schema.as_str()))
            .map(|a| a.id.clone());
        let n = traces.len() as f32;
        let mean_v = traces.iter().map(|t| t.valence).sum::<f32>() / n;
        let mean_a = traces.iter().map(|t| t.arousal).sum::<f32>() / n;
        let layer = if traces.len() >= 3 {
            AxiomLayer::Belief
        } else {
            AxiomLayer::Motif
        };
        let strength = match layer {
            AxiomLayer::Belief => (0.28 * n + 0.22 * mean_a).min(1.0),
            AxiomLayer::Motif => (0.18 * n + 0.15 * mean_a).min(0.55),
            AxiomLayer::Trait => 0.7,
        };
        let axiom = IdentityAxiom {
            id: new_id("ax"),
            statement,
            support_trace_ids: ids,
            valence: mean_v,
            strength,
            created_at: now_secs(),
            superseded_by: None,
            schema: Some(schema),
            layer,
        };
        if let Some(old) = prev_id {
            let can = store.axioms.get(&old).map(|prev| {
                match (prev.layer, layer) {
                    (AxiomLayer::Trait, AxiomLayer::Motif) => false,
                    (AxiomLayer::Belief, AxiomLayer::Motif) => prev.strength < 0.36,
                    _ => true,
                }
            }).unwrap_or(false);
            if can {
                if let Some(prev) = store.axioms.get_mut(&old) {
                    prev.superseded_by = Some(axiom.id.clone());
                }
            }
        }
        store.add_axiom(axiom.clone());
        created.push(axiom);
    }
    created
}

fn promote_traits(store: &mut MemoryStore, narrator: &dyn Narrator) -> Vec<IdentityAxiom> {
    let beliefs: Vec<IdentityAxiom> = store
        .living_axioms()
        .into_iter()
        .filter(|a| a.layer == AxiomLayer::Belief && a.strength >= 0.45)
        .cloned()
        .collect();
    if beliefs.len() < 2 {
        return Vec::new();
    }
    let pos: Vec<_> = beliefs.iter().filter(|a| a.valence > 0.2).collect();
    let neg: Vec<_> = beliefs.iter().filter(|a| a.valence < -0.2).collect();
    let mut out = Vec::new();
    for (bucket, label) in [(pos, "confiance"), (neg, "retrait")] {
        if bucket.len() < 2 {
            continue;
        }
        if store.living_axioms().iter().any(|a| a.layer == AxiomLayer::Trait && a.schema.as_deref() == Some(label)) {
            continue;
        }
        let traces: Vec<&crate::core::model::MemoryTrace> = bucket
            .iter()
            .flat_map(|a| a.support_trace_ids.iter())
            .filter_map(|id| store.traces.get(id))
            .collect();
        let statement = narrator.distill_axiom(&traces).unwrap_or_else(|| {
            if label == "confiance" {
                "Je m'attache lentement, mais je reste.".into()
            } else {
                "Je me retire quand on disparaît sans prévenir.".into()
            }
        });
        let mean_v = bucket.iter().map(|a| a.valence).sum::<f32>() / bucket.len() as f32;
        let axiom = IdentityAxiom {
            id: new_id("ax"),
            statement,
            support_trace_ids: bucket.iter().flat_map(|a| a.support_trace_ids.clone()).collect(),
            valence: mean_v,
            strength: 0.72,
            created_at: now_secs(),
            superseded_by: None,
            schema: Some(label.into()),
            layer: AxiomLayer::Trait,
        };
        store.add_axiom(axiom.clone());
        out.push(axiom);
    }
    out
}
