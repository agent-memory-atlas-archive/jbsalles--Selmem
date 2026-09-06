use crate::core::model::{AxiomLayer, Mood};
use crate::core::store::MemoryStore;
use crate::encode::EncodeInput;

/// Living identity colors a new event before the salience gate.
pub fn paint(store: &MemoryStore, mood: &Mood, input: &mut EncodeInput<'_>) {
    let axioms = store.living_axioms();
    if axioms.is_empty() {
        input.valence = (input.valence + 0.15 * mood.valence).clamp(-1.0, 1.0);
        input.arousal = (input.arousal + 0.08 * mood.arousal).clamp(0.0, 1.0);
        return;
    }

    let event = input.event.to_lowercase();
    let mut pull_v = 0.0;
    let mut pull_d = 0.0;
    let mut pull_s = 0.0;
    let mut schema_hit: Option<String> = None;
    let mut hits = 0.0;

    for a in axioms {
        let w = match a.layer {
            AxiomLayer::Trait => 1.0,
            AxiomLayer::Belief => 0.7,
            AxiomLayer::Motif => 0.35,
        } * a.strength.max(0.15);
        let stmt = a.statement.to_lowercase();
        let schema = a.schema.as_deref().unwrap_or("");
        let related = event_related(&event, schema, &stmt);
        if !related {
            continue;
        }
        hits += w;
        pull_v += w * a.valence;
        if a.valence < -0.2 {
            pull_d += w * 0.35;
        }
        pull_s += w;
        if schema_hit.is_none() && !schema.is_empty() {
            schema_hit = Some(schema.to_string());
        }
    }

    if hits > 0.0 {
        let v = pull_v / hits;
        input.valence = (0.65 * input.valence + 0.35 * v).clamp(-1.0, 1.0);
        input.disgust = (input.disgust + pull_d / hits.max(1.0)).clamp(0.0, 1.0);
        input.self_relevance = (input.self_relevance + 0.15 * pull_s.min(1.0)).clamp(0.0, 1.0);
        input.goal_align = (input.goal_align + 0.2 * pull_s.min(1.0)).clamp(0.0, 1.0);
        if input.schema.is_none() {
            input.schema = schema_hit;
        }
    }

    input.valence = (input.valence + 0.12 * mood.valence).clamp(-1.0, 1.0);
    input.arousal = (input.arousal + 0.08 * mood.arousal).clamp(0.0, 1.0);
}

fn event_related(event: &str, schema: &str, statement: &str) -> bool {
    if !schema.is_empty() && event.contains(&schema.to_lowercase()) {
        return true;
    }
    let keys = [
        "fidél", "loyal", "resté", "stayed", "abandon", "parti", "left", "trahi", "betray",
        "humili", "confian", "trust", "pluie", "rain", "aimer", "love", "peur", "fear",
        "honte", "shame",
    ];
    for k in keys {
        if (event.contains(k) || schema.contains(k)) && (statement.contains(k) || schema.contains(k))
        {
            return true;
        }
    }
    statement.split_whitespace().any(|w| w.len() > 5 && event.contains(w))
}
