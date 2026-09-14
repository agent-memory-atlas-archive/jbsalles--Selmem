use crate::encode::embed::{novelty_emb, Embedder};
use crate::core::model::{now_secs, new_id, ArchiveRecord, Channel, MemoryTrace, TraceStatus};
use crate::core::profile::EntityProfile;
use crate::encode::scoring::{encode_score, novelty};
use crate::core::store::MemoryStore;

pub struct EncodeInput<'a> {
    pub event: &'a str,
    pub source: &'a str,
    pub cues: Option<Vec<String>>,
    pub valence: f32,
    pub arousal: f32,
    pub disgust: f32,
    pub self_relevance: f32,
    pub utility: f32,
    pub goal_align: f32,
    pub schema: Option<String>,
    pub channel: Channel,
    pub permanence: f32,
}

impl<'a> EncodeInput<'a> {
    pub fn new(event: &'a str) -> Self {
        Self {
            event,
            source: "interaction",
            cues: None,
            valence: 0.0,
            arousal: 0.3,
            disgust: 0.0,
            self_relevance: 0.5,
            utility: 0.4,
            goal_align: 0.3,
            schema: None,
            channel: Channel::Selfhood,
            permanence: 0.0,
        }
    }
}

pub struct EncodeDecision {
    pub kept: bool,
    pub score: f32,
    pub reason: String,
    pub trace_id: Option<String>,
    pub archive_id: String,
}

pub fn encode(
    store: &mut MemoryStore,
    profile: &EntityProfile,
    input: EncodeInput<'_>,
    embedder: &dyn Embedder,
) -> EncodeDecision {
    let embedding = embedder.embed(input.event);
    let existing: Vec<Vec<f32>> = store
        .traces
        .values()
        .filter(|t| !t.embedding.is_empty())
        .map(|t| t.embedding.clone())
        .collect();
    let nov = if existing.is_empty() {
        let gists: Vec<String> = store.traces.values().map(|t| t.gist.clone()).collect();
        novelty(input.event, &gists)
    } else {
        novelty_emb(&embedding, &existing)
    };
    let score = encode_score(
        profile,
        input.arousal,
        nov,
        input.self_relevance,
        input.utility,
        input.goal_align,
        1.0 - nov,
    );

    let mut threshold = profile.encode_threshold;
    if input.channel == Channel::World {
        threshold *= 0.6;
    }
    if input.permanence >= 0.8 {
        threshold = threshold.min(0.2);
    }

    if score < threshold && input.permanence < 0.8 && input.channel != Channel::World {
        return EncodeDecision {
            kept: false,
            score,
            reason: format!("below threshold ({score:.2} < {threshold:.2})"),
            trace_id: None,
            archive_id: String::new(),
        };
    }

    let archive = ArchiveRecord {
        id: new_id("ar"),
        verbatim: input.event.to_string(),
        source: input.source.to_string(),
        created_at: now_secs(),
    };
    let archive_id = store.add_archive(archive);

    let mut trace = MemoryTrace {
        id: new_id("tr"),
        gist: compress(input.event, 28),
        core: compress(input.event, 12),
        cues: input.cues.unwrap_or_else(|| default_cues(input.event)),
        valence: input.valence,
        arousal: input.arousal,
        disgust: input.disgust,
        self_relevance: input.self_relevance,
        schema: input.schema,
        channel: input.channel,
        archive_id: Some(archive_id.clone()),
        created_at: now_secs(),
        last_recalled_at: None,
        last_consolidated_at: None,
        fidelity: 1.0,
        permanence: input.permanence,
        rehearsals: 0,
        access: 1.0,
        status: TraceStatus::Active,
        drifts: Vec::new(),
        salience_at_encode: score,
        embedding,
        anchor: crate::dream::singularite::seed_anchor(input.valence, input.arousal, input.disgust, input.permanence, input.self_relevance),
        detach_strikes: 0,
    };
    trace.clamp();
    associate(store, &trace);
    let tid = store.add_trace(trace);
    EncodeDecision {
        kept: true,
        score,
        reason: "encoded".into(),
        trace_id: Some(tid),
        archive_id,
    }
}

fn associate(store: &mut MemoryStore, trace: &MemoryTrace) {
    let others: Vec<(String, Option<String>, Vec<String>)> = store
        .traces
        .values()
        .filter(|o| o.id != trace.id)
        .map(|o| (o.id.clone(), o.schema.clone(), o.cues.clone()))
        .collect();
    for (oid, schema, cues) in others {
        if schema.is_some() && schema == trace.schema {
            store.link(&trace.id, &oid);
            continue;
        }
        let shared = trace.cues.iter().any(|c| {
            cues.iter().any(|o| o.eq_ignore_ascii_case(c))
        });
        if shared {
            store.link(&trace.id, &oid);
        }
    }
}

/// LLM may propose a core. Accept only if it still talks about this event.
pub fn accept_core(proposed: &str, event: &str) -> Option<String> {
    let p = proposed.trim();
    if p.is_empty() || p.chars().count() < 8 {
        return None;
    }
    let p: String = p.chars().take(500).collect();
    let ev: Vec<String> = event
        .split_whitespace()
        .map(|w| {
            w.trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase()
        })
        .filter(|w| w.chars().count() > 2)
        .collect();
    let pr: Vec<String> = p
        .split_whitespace()
        .map(|w| {
            w.trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase()
        })
        .filter(|w| w.chars().count() > 2)
        .collect();
    if pr.is_empty() {
        return None;
    }
    let hit = pr.iter().filter(|w| ev.iter().any(|e| e == *w)).count();
    if hit * 4 < pr.len() && hit < 2 {
        return None;
    }
    Some(p)
}

fn compress(event: &str, max_words: usize) -> String {
    let words: Vec<&str> = event.split_whitespace().collect();
    if words.len() <= max_words {
        event.trim().to_string()
    } else {
        format!("{}…", words[..max_words].join(" "))
    }
}

fn default_cues(event: &str) -> Vec<String> {
    event
        .split_whitespace()
        .map(|w| {
            w.trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase()
        })
        .filter(|w| w.chars().count() > 4)
        .take(8)
        .collect()
}
