//! Pick a few traces for a query, reconstruct them, then optionally
//! pull a drifted sentence back toward its semantic core.

use crate::core::model::{now_secs, Channel, Mood, RecalledMemory, TraceStatus};
use crate::core::profile::EntityProfile;
use crate::core::store::MemoryStore;
use crate::dream::drift::apply_reconsolidation;
use crate::encode::embed::Embedder;
use crate::encode::scoring::recall_score_emb;
use crate::recall::narrator::Narrator;

pub fn recall(
    store: &mut MemoryStore,
    profile: &EntityProfile,
    narrator: &dyn Narrator,
    embedder: &dyn Embedder,
    query: &str,
    mood: &Mood,
) -> Vec<RecalledMemory> {
    let query_embedding = embedder.embed(query);
    let mut ranked: Vec<(String, f32)> = store
        .active_ids()
        .into_iter()
        .filter_map(|trace_id| {
            let trace = store.traces.get_mut(&trace_id)?;
            let score = recall_score_emb(trace, query, Some(&query_embedding), mood, profile);
            Some((trace_id, score))
        })
        .collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut chosen_ids = Vec::new();
    for (trace_id, score) in ranked {
        if chosen_ids.len() >= profile.max_recall {
            break;
        }
        let status = store.traces.get(&trace_id).map(|trace| trace.status);
        // Latent traces have no scene to tell. Their charge is used at encode time only.
        if matches!(status, Some(TraceStatus::Latent)) {
            continue;
        }
        if matches!(status, Some(TraceStatus::Cold)) && score < 0.12 {
            continue;
        }
        if score < 0.08 {
            continue;
        }
        chosen_ids.push(trace_id);
    }

    let mut recalled = Vec::new();
    for trace_id in chosen_ids {
        let (channel, gist, schema) = {
            let trace = store.traces.get(&trace_id).unwrap();
            (trace.channel, trace.gist.clone(), trace.schema.clone())
        };

        let (narrative, disclaimer, fidelity) = if channel == Channel::World {
            (
                gist,
                "operational fact, not distorted".to_string(),
                store.traces[&trace_id].fidelity,
            )
        } else {
            let generated = {
                let trace = store.traces.get(&trace_id).unwrap();
                narrator.reconstruct(trace, mood, query)
            };
            let core = store.traces.get(&trace_id).unwrap().core.clone();
            let narrator_rewrite = {
                let trace = store.traces.get(&trace_id).unwrap();
                if crate::recall::ground::should_force_core_rewrite(trace, profile, &generated, &core)
                {
                    Some(narrator.recontextualize(trace, &core, profile))
                } else {
                    None
                }
            };
            let outcome = {
                let trace = store.traces.get_mut(&trace_id).unwrap();
                crate::recall::ground::apply_grounding(
                    trace,
                    profile,
                    &generated,
                    &core,
                    narrator_rewrite,
                )
            };
            if !outcome.pulled_toward_core {
                if let Some(trace) = store.traces.get_mut(&trace_id) {
                    apply_reconsolidation(trace, &outcome.spoken_text, profile, mood.valence);
                }
            }
            let trace = store.traces.get(&trace_id).unwrap();
            let disclaimer = if outcome.pulled_toward_core {
                "pulled back toward the core".to_string()
            } else {
                format!("lived account (fidelity {:.2})", trace.fidelity)
            };
            (outcome.spoken_text, disclaimer, trace.fidelity)
        };

        if let Some(trace) = store.traces.get_mut(&trace_id) {
            trace.rehearsals += 1;
            trace.last_recalled_at = Some(now_secs());
        }
        recalled.push(RecalledMemory {
            trace_id,
            narrative,
            fidelity,
            schema,
            channel,
            disclaimer,
        });
    }
    recalled
}
