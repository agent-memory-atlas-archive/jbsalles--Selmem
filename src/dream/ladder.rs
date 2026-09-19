//! Fourth night pass: motif → belief → trait. Latent traces do not mint.

use std::collections::HashMap;

use crate::core::model::{now_secs, new_id, AxiomLayer, Channel, IdentityAxiom, TraceStatus};
use crate::core::store::MemoryStore;
use crate::recall::narrator::Narrator;

pub fn run(store: &mut MemoryStore, narrator: &dyn Narrator) -> Vec<IdentityAxiom> {
    let mut axioms = extract_axioms(store, narrator);
    axioms.extend(promote_traits(store, narrator));
    axioms
}

fn extract_axioms(store: &mut MemoryStore, narrator: &dyn Narrator) -> Vec<IdentityAxiom> {
    let mut evidence: HashMap<String, Vec<String>> = HashMap::new();
    let mut living: HashMap<String, Vec<String>> = HashMap::new();
    for t in store.traces.values() {
        if t.channel == Channel::World {
            continue;
        }
        let Some(s) = t.schema.as_ref() else { continue };
        match t.status {
            // Merged siblings stay Myth; they still count as episodes.
            TraceStatus::Active | TraceStatus::Cold => {
                evidence.entry(s.clone()).or_default().push(t.id.clone());
                living.entry(s.clone()).or_default().push(t.id.clone());
            }
            TraceStatus::Myth => {
                evidence.entry(s.clone()).or_default().push(t.id.clone());
            }
            // Latent must not mint a belief.
            _ => {}
        }
    }
    let existing: Vec<String> = store
        .living_axioms()
        .into_iter()
        .map(|a| a.statement.clone())
        .collect();

    let mut created = Vec::new();
    for (schema, ids) in evidence {
        if ids.len() < 2 {
            continue;
        }
        let Some(live_ids) = living.get(&schema) else {
            continue;
        };
        let traces: Vec<&crate::core::model::MemoryTrace> = ids
            .iter()
            .filter_map(|id| store.traces.get(id))
            .collect();
        let _ = live_ids;
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
        let n = ids.len() as f32;
        let mean_v = traces.iter().map(|t| t.valence).sum::<f32>() / n.max(1.0);
        let mean_a = traces.iter().map(|t| t.arousal).sum::<f32>() / n.max(1.0);
        let layer = if ids.len() >= 3 {
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
            let can = store
                .axioms
                .get(&old)
                .map(|prev| match (prev.layer, layer) {
                    (AxiomLayer::Trait, AxiomLayer::Motif) => false,
                    (AxiomLayer::Belief, AxiomLayer::Motif) => prev.strength < 0.36,
                    _ => true,
                })
                .unwrap_or(false);
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
    for (bucket, label) in [(pos, "trust"), (neg, "withdrawal")] {
        if bucket.len() < 2 {
            continue;
        }
        if store
            .living_axioms()
            .iter()
            .any(|a| a.layer == AxiomLayer::Trait && a.schema.as_deref() == Some(label))
        {
            continue;
        }
        let traces: Vec<&crate::core::model::MemoryTrace> = bucket
            .iter()
            .flat_map(|a| a.support_trace_ids.iter())
            .filter_map(|id| store.traces.get(id))
            .collect();
        let statement = narrator.distill_axiom(&traces).unwrap_or_else(|| {
            if label == "trust" {
                "I attach slowly, but I stay.".into()
            } else {
                "I pull away when someone vanishes without warning.".into()
            }
        });
        let mean_v = bucket.iter().map(|a| a.valence).sum::<f32>() / bucket.len() as f32;
        let axiom = IdentityAxiom {
            id: new_id("ax"),
            statement,
            support_trace_ids: bucket
                .iter()
                .flat_map(|a| a.support_trace_ids.clone())
                .collect(),
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
