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
    /// How many fact slices the gate saw. 1 = the event was small enough to keep whole.
    pub parts: usize,
    pub kept_n: usize,
}

pub fn encode(
    store: &mut MemoryStore,
    profile: &EntityProfile,
    input: EncodeInput<'_>,
    embedder: &dyn Embedder,
) -> EncodeDecision {
    encode_with_parts(store, profile, input, embedder, None)
}

pub fn encode_with_parts(
    store: &mut MemoryStore,
    profile: &EntityProfile,
    input: EncodeInput<'_>,
    embedder: &dyn Embedder,
    proposed: Option<&[String]>,
) -> EncodeDecision {
    let parts = split_event(input.event, proposed);
    if parts.is_empty() {
        return EncodeDecision {
            kept: false,
            score: 0.0,
            reason: "empty".into(),
            trace_id: None,
            archive_id: String::new(),
            parts: 0,
            kept_n: 0,
        };
    }
    if parts.len() == 1 {
        return encode_one(store, profile, input, embedder, None);
    }

    // Novelty is against the book as it stood before this paste. Sibling
    // slices of the same document must not knock each other under τ.
    let prior_emb: Vec<Vec<f32>> = store
        .traces
        .values()
        .filter(|t| !t.embedding.is_empty())
        .map(|t| t.embedding.clone())
        .collect();
    let prior_gists: Vec<String> = store.traces.values().map(|t| t.gist.clone()).collect();

    let mut kept_ids = Vec::new();
    let mut first_archive = String::new();
    let mut best = 0.0_f32;
    let mut last_reason = String::new();
    for part in &parts {
        let mut slice = EncodeInput::new(part);
        slice.source = input.source;
        slice.valence = input.valence;
        slice.arousal = input.arousal;
        slice.disgust = input.disgust;
        slice.self_relevance = input.self_relevance;
        slice.utility = input.utility;
        slice.goal_align = input.goal_align;
        slice.schema = input.schema.clone();
        slice.channel = input.channel;
        slice.permanence = input.permanence;
        let d = encode_one(
            store,
            profile,
            slice,
            embedder,
            Some((&prior_emb, &prior_gists)),
        );
        best = best.max(d.score);
        last_reason = d.reason;
        if d.kept {
            if first_archive.is_empty() {
                first_archive = d.archive_id;
            }
            if let Some(id) = d.trace_id {
                kept_ids.push(id);
            }
        }
    }
    EncodeDecision {
        kept: !kept_ids.is_empty(),
        score: best,
        reason: if kept_ids.is_empty() {
            last_reason
        } else {
            format!("encoded {}/{} parts", kept_ids.len(), parts.len())
        },
        trace_id: kept_ids.first().cloned(),
        archive_id: first_archive,
        parts: parts.len(),
        kept_n: kept_ids.len(),
    }
}

fn encode_one(
    store: &mut MemoryStore,
    profile: &EntityProfile,
    input: EncodeInput<'_>,
    embedder: &dyn Embedder,
    novelty_vs: Option<(&Vec<Vec<f32>>, &Vec<String>)>,
) -> EncodeDecision {
    let embedding = embedder.embed(input.event);
    let nov = if let Some((existing, gists)) = novelty_vs {
        if existing.is_empty() {
            novelty(input.event, gists)
        } else {
            novelty_emb(&embedding, existing)
        }
    } else {
        let existing: Vec<Vec<f32>> = store
            .traces
            .values()
            .filter(|t| !t.embedding.is_empty())
            .map(|t| t.embedding.clone())
            .collect();
        if existing.is_empty() {
            let gists: Vec<String> = store.traces.values().map(|t| t.gist.clone()).collect();
            novelty(input.event, &gists)
        } else {
            novelty_emb(&embedding, &existing)
        }
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
            parts: 1,
            kept_n: 0,
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
        parts: 1,
        kept_n: 1,
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

/// True when a paste is too long to compress as one hour.
pub fn needs_split(event: &str) -> bool {
    let text = event.trim();
    if text.is_empty() {
        return false;
    }
    let lines = text.lines().map(str::trim).filter(|l| !l.is_empty()).count();
    let words = text.split_whitespace().count();
    lines > 10 || words > 80
}

/// Semantic cut if `proposed` is a lossless partition of `event`.
/// Otherwise pack by lines, then by sentences / words.
pub fn split_event(event: &str, proposed: Option<&[String]>) -> Vec<String> {
    if let Some(p) = proposed {
        if let Some(parts) = lossless_parts(event, p) {
            return parts;
        }
    }
    segment_facts(event)
}

/// Each unit must be a contiguous excerpt of `source`, in order, covering
/// almost the whole text. Paraphrase is rejected.
pub fn lossless_parts(source: &str, proposed: &[String]) -> Option<Vec<String>> {
    let tokens = tokens_with_spans(source);
    if tokens.is_empty() {
        return None;
    }
    let mut ti = 0usize;
    let mut out = Vec::new();
    for p in proposed {
        let words: Vec<&str> = p.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }
        let mut found = None;
        let mut j = ti;
        while j + words.len() <= tokens.len() {
            let hit = tokens[j..j + words.len()]
                .iter()
                .zip(words.iter())
                .all(|(tok, w)| tok.2.eq_ignore_ascii_case(w));
            if hit {
                found = Some((j, j + words.len()));
                break;
            }
            j += 1;
        }
        let (a, b) = found?;
        if a < ti {
            return None;
        }
        let start = tokens[a].0;
        let end = tokens[b - 1].1;
        out.push(source[start..end].to_string());
        ti = b;
    }
    if out.len() < 2 {
        return None;
    }
    if tokens.len() - ti > 6 {
        return None;
    }
    Some(out)
}

fn tokens_with_spans(source: &str) -> Vec<(usize, usize, &str)> {
    let mut out = Vec::new();
    let mut start = None;
    for (i, ch) in source.char_indices() {
        if ch.is_whitespace() {
            if let Some(s) = start.take() {
                out.push((s, i, &source[s..i]));
            }
        } else if start.is_none() {
            start = Some(i);
        }
    }
    if let Some(s) = start {
        out.push((s, source.len(), &source[s..]));
    }
    out
}

/// Read a JSON string array out of a model reply. Anything else is ignored.
pub fn parse_segment_reply(raw: &str) -> Option<Vec<String>> {
    let start = raw.find('[')?;
    let bytes = raw[start..].as_bytes();
    let mut depth = 0i32;
    let mut end = None;
    let mut in_str = false;
    let mut esc = false;
    for (i, b) in bytes.iter().enumerate() {
        if in_str {
            if esc {
                esc = false;
            } else if *b == b'\\' {
                esc = true;
            } else if *b == b'"' {
                in_str = false;
            }
            continue;
        }
        match *b {
            b'"' => in_str = true,
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(i + 1);
                    break;
                }
            }
            _ => {}
        }
    }
    let slice = &raw[start..start + end?];
    let parts = parse_json_strings(slice);
    if parts.len() >= 2 {
        Some(parts)
    } else {
        None
    }
}

fn parse_json_strings(s: &str) -> Vec<String> {
    let mut body = s.trim_start();
    if !body.starts_with('[') {
        return Vec::new();
    }
    body = body[1..].trim_start();
    let mut out = Vec::new();
    loop {
        body = body.trim_start();
        if body.is_empty() || body.starts_with(']') {
            break;
        }
        if body.starts_with(',') {
            body = body[1..].trim_start();
            continue;
        }
        if body.starts_with('"') {
            if let Some((v, n)) = crate::net::httpx::parse_json_string(body) {
                out.push(v);
                body = &body[n..];
                continue;
            }
        }
        break;
    }
    out
}

/// A short hour stays one fact. A long paste is packed into 5–10 line slices
/// so compress(28) does not throw away everything after the first paragraph.
pub fn segment_facts(event: &str) -> Vec<String> {
    let text = event.trim();
    if text.is_empty() {
        return Vec::new();
    }
    let lines: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    let words = text.split_whitespace().count();
    if lines.len() <= 10 && words <= 80 {
        return vec![text.to_string()];
    }
    if lines.len() <= 1 {
        return pack_sentences(text, 40, 70);
    }
    pack_lines(&lines, 5, 10)
}

fn pack_lines(lines: &[&str], min: usize, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur: Vec<&str> = Vec::new();
    let target = ((min + max) / 2).max(1);
    for line in lines {
        cur.push(*line);
        if cur.len() >= target {
            out.push(cur.join("\n"));
            cur.clear();
        }
    }
    if !cur.is_empty() {
        if let Some(last) = out.last_mut() {
            let last_n = last.lines().count();
            if last_n + cur.len() <= max {
                last.push('\n');
                last.push_str(&cur.join("\n"));
            } else {
                out.push(cur.join("\n"));
            }
        } else {
            out.push(cur.join("\n"));
        }
    }
    out
}

fn pack_sentences(text: &str, min_words: usize, max_words: usize) -> Vec<String> {
    let mut sentences: Vec<String> = Vec::new();
    let mut buf = String::new();
    for ch in text.chars() {
        buf.push(ch);
        if matches!(ch, '.' | '!' | '?' | '。' | '…') {
            let s = buf.trim();
            if !s.is_empty() {
                sentences.push(s.to_string());
            }
            buf.clear();
        }
    }
    let tail = buf.trim();
    if !tail.is_empty() {
        sentences.push(tail.to_string());
    }
    if sentences.len() <= 1 {
        return pack_words(text, max_words);
    }
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut n = 0usize;
    for s in sentences {
        let w = s.split_whitespace().count();
        if n > 0 && n + w > max_words {
            out.push(cur.trim().to_string());
            cur.clear();
            n = 0;
        }
        if !cur.is_empty() {
            cur.push(' ');
        }
        cur.push_str(&s);
        n += w;
        if n >= min_words && n >= max_words / 2 {
            out.push(cur.trim().to_string());
            cur.clear();
            n = 0;
        }
    }
    if !cur.trim().is_empty() {
        if let Some(last) = out.last_mut() {
            let last_n = last.split_whitespace().count();
            if last_n + n <= max_words {
                last.push(' ');
                last.push_str(cur.trim());
            } else {
                out.push(cur.trim().to_string());
            }
        } else {
            out.push(cur.trim().to_string());
        }
    }
    if out.is_empty() {
        vec![text.to_string()]
    } else {
        out
    }
}

fn pack_words(text: &str, max_words: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.len() <= max_words {
        return vec![text.to_string()];
    }
    words
        .chunks(max_words)
        .map(|c| c.join(" "))
        .collect()
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
