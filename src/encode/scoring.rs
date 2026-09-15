use crate::core::model::{now_secs, MemoryTrace, Mood};
use crate::core::profile::EntityProfile;

pub fn token_set(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for ch in text.chars() {
        if ch.is_alphanumeric() {
            cur.extend(ch.to_lowercase());
        } else if !cur.is_empty() {
            if cur.chars().count() > 2 {
                out.push(std::mem::take(&mut cur));
            } else {
                cur.clear();
            }
        }
    }
    if cur.chars().count() > 2 {
        out.push(cur);
    }
    out.sort();
    out.dedup();
    out
}

pub fn lexical_similarity(a: &str, b: &str) -> f32 {
    let sa = token_set(a);
    let sb = token_set(b);
    if sa.is_empty() || sb.is_empty() {
        return 0.0;
    }
    let inter = sa.iter().filter(|t| sb.binary_search(t).is_ok()).count();
    let union = sa.len() + sb.len() - inter;
    if union == 0 {
        0.0
    } else {
        inter as f32 / union as f32
    }
}

pub fn novelty(text: &str, gists: &[String]) -> f32 {
    if gists.is_empty() {
        return 1.0;
    }
    let nearest = gists
        .iter()
        .map(|g| lexical_similarity(text, g))
        .fold(0.0_f32, f32::max);
    1.0 - nearest
}

pub fn encode_score(
    profile: &EntityProfile,
    arousal: f32,
    novelty_s: f32,
    self_relevance: f32,
    utility: f32,
    goal_align: f32,
    redundancy: f32,
) -> f32 {
    profile.w_arousal * arousal
        + profile.w_novelty * novelty_s
        + profile.w_self * self_relevance
        + profile.w_utility * utility
        + profile.w_goal * goal_align
        - profile.w_redundancy * redundancy
}

pub fn refresh_access(trace: &mut MemoryTrace, profile: &EntityProfile) -> f32 {
    let origin = trace.last_recalled_at.unwrap_or(trace.created_at);
    let age_days = (now_secs().saturating_sub(origin) as f32) / 86_400.0;
    // Low salience at the gate → faster access decay. High salience holds.
    let forget = 0.45 + 1.55 * (1.0 - trace.salience_at_encode.clamp(0.0, 1.0));
    let lam = profile.decay_lambda
        * forget
        * (1.0 - 0.7 * trace.arousal)
        * (1.0 - trace.permanence)
        * (1.0 - 0.85 * trace.anchor);
    let decay = (-lam * age_days).exp();
    let rehearsal = 1.0 + profile.rehearsal_boost * (1.0 + trace.rehearsals as f32).ln();
    trace.access = (trace.fidelity * decay * rehearsal).clamp(0.0, 1.0);
    trace.access
}

pub fn affective_congruence(trace: &MemoryTrace, mood: &Mood) -> f32 {
    let v = 1.0 - (trace.valence - mood.valence).abs() / 2.0;
    let d = 1.0 - (trace.disgust - mood.disgust).abs();
    0.6 * v + 0.4 * d
}

pub fn recall_score_emb(
    trace: &mut MemoryTrace,
    query: &str,
    query_emb: Option<&[f32]>,
    mood: &Mood,
    profile: &EntityProfile,
) -> f32 {
    refresh_access(trace, profile);
    let mut sim = lexical_similarity(query, &trace.gist);
    for cue in &trace.cues {
        sim = sim.max(lexical_similarity(query, cue));
    }
    if let Some(q) = query_emb {
        if !trace.embedding.is_empty() {
            sim = sim.max(crate::encode::embed::cosine(q, &trace.embedding));
        }
    }
    let cong = affective_congruence(trace, mood);
    (0.50 * sim + 0.25 * trace.access + 0.25 * cong) * (0.6 + 0.4 * trace.self_relevance)
}
