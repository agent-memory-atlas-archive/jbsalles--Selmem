use crate::dream::drift::apply_reconsolidation;
use crate::encode::embed::Embedder;
use crate::core::model::{now_secs, Channel, Mood, RecalledMemory, TraceStatus};
use crate::recall::narrator::Narrator;
use crate::core::profile::EntityProfile;
use crate::encode::scoring::recall_score_emb;
use crate::core::store::MemoryStore;

pub fn recall(
    store: &mut MemoryStore,
    profile: &EntityProfile,
    narrator: &dyn Narrator,
    embedder: &dyn Embedder,
    query: &str,
    mood: &Mood,
) -> Vec<RecalledMemory> {
    let qemb = embedder.embed(query);
    let mut scored: Vec<(String, f32)> = store
        .active_ids()
        .into_iter()
        .filter_map(|id| {
            let trace = store.traces.get_mut(&id)?;
            let s = recall_score_emb(trace, query, Some(&qemb), mood, profile);
            Some((id, s))
        })
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut chosen = Vec::new();
    for (id, score) in scored {
        if chosen.len() >= profile.max_recall {
            break;
        }
        let status = store.traces.get(&id).map(|t| t.status);
        if matches!(status, Some(TraceStatus::Cold)) && score < 0.12 {
            continue;
        }
        if score < 0.08 {
            continue;
        }
        chosen.push(id);
    }

    let mut out = Vec::new();
    for id in chosen {
        let (channel, gist, schema) = {
            let t = store.traces.get(&id).unwrap();
            (t.channel, t.gist.clone(), t.schema.clone())
        };
        let (narrative, disclaimer, fidelity) = if channel == Channel::World {
            (
                gist,
                "fait opérationnel, non déformé".to_string(),
                store.traces[&id].fidelity,
            )
        } else {
            let generated = {
                let t = store.traces.get(&id).unwrap();
                narrator.reconstruct(t, mood, query)
            };
            let core = store.traces.get(&id).unwrap().core.clone();
            let rewrite = {
                let t = store.traces.get(&id).unwrap();
                if crate::recall::ground::will_correct(t, profile, &generated, &core) {
                    Some(narrator.recontextualize(t, &core, profile))
                } else {
                    None
                }
            };
            let check = {
                let t = store.traces.get_mut(&id).unwrap();
                crate::recall::ground::note(t, profile, &generated, &core, rewrite)
            };
            if !check.corrected {
                if let Some(t) = store.traces.get_mut(&id) {
                    apply_reconsolidation(t, &check.text, profile, mood.valence);
                }
            }
            let t = store.traces.get(&id).unwrap();
            let disclaimer = if check.corrected {
                "reprise vers le core".to_string()
            } else {
                format!("récit vécu (fidélité {:.2})", t.fidelity)
            };
            (check.text, disclaimer, t.fidelity)
        };
        if let Some(t) = store.traces.get_mut(&id) {
            t.rehearsals += 1;
            t.last_recalled_at = Some(now_secs());
        }
        out.push(RecalledMemory {
            trace_id: id,
            narrative,
            fidelity,
            schema,
            channel,
            disclaimer,
        });
    }
    out
}
