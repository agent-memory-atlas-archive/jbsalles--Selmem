use crate::core::model::{MemoryTrace, Mood};

#[derive(Clone, Debug)]
pub struct Interpretation {
    pub valence: f32,
    pub arousal: f32,
    pub disgust: f32,
    pub schema: Option<String>,
    pub self_relevance: f32,
}

pub trait Narrator: Send + Sync {
    fn reconstruct(&self, trace: &MemoryTrace, mood: &Mood, query: &str) -> String;
    fn distill_axiom(&self, traces: &[&MemoryTrace]) -> Option<String>;
    /// Live event only — never an archive. Default: none (caller uses lexicon + identity).
    fn interpret(&self, event: &str, mood: &Mood, axioms: &[String]) -> Option<Interpretation> {
        let _ = (event, mood, axioms);
        None
    }
    /// Réécriture de consolidation : garder la charge, accentuer, jeter le superflu.
    fn rewrite(&self, trace: &MemoryTrace, neighbors: &[&MemoryTrace]) -> Option<String> {
        let _ = neighbors;
        if trace.core.is_empty() {
            Some(trace.gist.clone())
        } else if trace.fidelity < 0.5 {
            Some(trace.core.clone())
        } else {
            Some(trace.gist.clone())
        }
    }
    fn reply(&self, user: &str, memories: &[String], axioms: &[String], mood: &Mood) -> String {
        let mut out = String::new();
        if let Some(ax) = axioms.first() {
            out.push_str(ax);
            out.push(' ');
        }
        if let Some(m) = memories.first() {
            out.push_str("Cela me revient : ");
            out.push_str(m);
            out.push(' ');
        }
        if out.is_empty() {
            out.push_str("Je t'écoute. ");
        }
        out.push_str("(");
        out.push_str(user);
        out.push_str(")");
        let _ = mood;
        out
    }
}

#[derive(Default)]
pub struct RuleNarrator;

impl Narrator for RuleNarrator {
    fn reconstruct(&self, trace: &MemoryTrace, mood: &Mood, _query: &str) -> String {
        let holes = if trace.fidelity < 0.7 {
            "certains détails se dérobent"
        } else {
            "le souvenir est encore net"
        };
        let mood_tint = if mood.valence > 0.2 {
            "une chaleur persistante"
        } else if mood.valence < -0.2 {
            "une ombre courte"
        } else {
            "un calme plat"
        };
        let color = if trace.disgust > 0.45 {
            "le corps se rappelle d'abord le retrait"
        } else if trace.valence > 0.25 {
            "ce qui reste a pris plus d'éclat que l'instant"
        } else {
            "rien n'est tout à fait à sa place d'origine"
        };
        let schema = match &trace.schema {
            Some(s) => format!(" Schéma {s}."),
            None => String::new(),
        };
        format!(
            "{} — {}. Aujourd'hui le souvenir arrive avec {mood_tint} ; {holes}.{schema}",
            if trace.fidelity < 0.42 && !trace.core.is_empty() {
                trace.core.trim_end_matches('.')
            } else {
                trace.gist.trim_end_matches('.')
            },
            color
        )
    }

    fn distill_axiom(&self, traces: &[&MemoryTrace]) -> Option<String> {
        if traces.len() < 2 {
            return None;
        }
        let mut counts: Vec<(String, usize)> = Vec::new();
        for t in traces {
            if let Some(s) = &t.schema {
                if let Some(slot) = counts.iter_mut().find(|(k, _)| k == s) {
                    slot.1 += 1;
                } else {
                    counts.push((s.clone(), 1));
                }
            }
        }
        let dominant = counts.into_iter().max_by_key(|(_, n)| *n)?.0;
        let mean_v: f32 = traces.iter().map(|t| t.valence).sum::<f32>() / traces.len() as f32;
        Some(if mean_v < -0.2 {
            format!("Je me retire de ce qui ressemble à : {dominant}.")
        } else if mean_v > 0.2 {
            format!("Je garde précieusement ce qui ressemble à : {dominant}.")
        } else {
            format!("Je reconnais un motif récurrent : {dominant}.")
        })
    }
}


