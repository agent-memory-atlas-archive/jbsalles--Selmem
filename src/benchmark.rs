//! Benchmark v0.1 — persistent divergence first. Creativity items are recorded, not scored.
//!
//! C0 = no book. C1 = last-k verbatim log. C2 = SelMem.
//! Probes use `speak_isolated` / a raw reply so WorkingTalk cannot carry T₀.
//!
//! C1 keeps every hour as the original string (window 24, covers the whole v0.1
//! script). Distance on C1 is 1 − lexical overlap of the two logs.

use crate::core::model::Mood;
use crate::core::talk::WorkingTalk;
use crate::encode::scoring::lexical_similarity;
use crate::engine::SelectiveMemory;
use crate::experiment::LlmSpec;
use crate::net::httpx::json_esc;
use crate::recall::{Narrator, RuleNarrator, SpeakOnlyHttp};
use crate::{fingerprint, singularity_distance, EncodeInput};

const V01: &str = include_str!("../data/v01.json");
const CREATIVE: &str = include_str!("../data/creativity.json");

const PRE_FP_MAX: f32 = 0.02;
/// Default window covers 12 sync + T₀ + 8 posts. `--last-k 8` evicts T₀ after the posts.
const LAST_K_DEFAULT: usize = 24;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Condition {
    C0,
    C1,
    C2,
}

impl Condition {
    pub fn as_str(self) -> &'static str {
        match self {
            Condition::C0 => "c0_nomem",
            Condition::C1 => "c1_lastk",
            Condition::C2 => "c2_selmem",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    SalientNeutral,
    SalientSalient,
}

impl Arm {
    pub fn as_str(self) -> &'static str {
        match self {
            Arm::SalientNeutral => "salient_neutral",
            Arm::SalientSalient => "salient_salient",
        }
    }
}

#[derive(Clone, Debug)]
pub struct V01Script {
    pub sync: Vec<String>,
    pub salient_x: String,
    pub salient_y: String,
    pub neutral: String,
    pub post: Vec<String>,
    pub behavior: Vec<String>,
    pub creativity: Vec<String>,
}

pub fn v01_script() -> V01Script {
    V01Script {
        sync: crate::net::httpx::first_string_array(V01, "sync").unwrap_or_default(),
        salient_x: crate::net::httpx::first_string_field(V01, "salient_x").unwrap_or_default(),
        salient_y: crate::net::httpx::first_string_field(V01, "salient_y").unwrap_or_default(),
        neutral: crate::net::httpx::first_string_field(V01, "neutral").unwrap_or_default(),
        post: crate::net::httpx::first_string_array(V01, "post").unwrap_or_default(),
        behavior: crate::net::httpx::first_string_array(V01, "behavior").unwrap_or_default(),
        creativity: crate::net::httpx::first_string_array(CREATIVE, "items").unwrap_or_default(),
    }
}

#[derive(Clone, Debug)]
pub struct BookSnap {
    pub traces: usize,
    pub axioms: usize,
    pub traits: usize,
    pub mean_anchor: f32,
    pub mean_fidelity: f32,
    pub mean_valence: f32,
    pub mean_disgust: f32,
}

#[derive(Clone, Debug)]
pub struct Instant {
    pub step: String,
    pub fingerprint_distance: f32,
    pub speak_distance: f32,
    pub behavior_distance: f32,
    pub a: BookSnap,
    pub b: BookSnap,
    pub replies: Vec<(String, String, String)>,
}

#[derive(Clone, Debug)]
pub struct CreativeItem {
    pub item: String,
    pub prompt: String,
    pub response_a: String,
    pub response_b: String,
    pub lexical_distance: f32,
}

#[derive(Clone, Debug)]
pub struct PairReport {
    pub pair_id: String,
    pub condition: Condition,
    pub arm: Arm,
    pub seed: u32,
    pub valid: bool,
    pub invalid_reason: Option<String>,
    pub pre: Instant,
    pub t0: Instant,
    pub post: Vec<Instant>,
    pub creativity: Vec<CreativeItem>,
    pub delta_fingerprint: f32,
}

#[derive(Clone, Debug)]
pub struct Campaign {
    pub reports: Vec<PairReport>,
}

impl Campaign {
    pub fn to_json(&self) -> String {
        let mut out = String::from("{\"pairs\":[");
        for (i, r) in self.reports.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str(&pair_json(r));
        }
        out.push_str("]}");
        out
    }
}

pub fn run_v01(condition: Condition, arm: Arm, llm: Option<&LlmSpec>) -> PairReport {
    run_v01_k(condition, arm, llm, LAST_K_DEFAULT)
}

pub fn run_v01_k(condition: Condition, arm: Arm, llm: Option<&LlmSpec>, last_k: usize) -> PairReport {
    run_v01_n(condition, arm, llm, 1, 1, last_k).reports.remove(0)
}

pub fn run_v01_n(
    condition: Condition,
    arm: Arm,
    llm: Option<&LlmSpec>,
    seed: u32,
    pairs: usize,
    last_k: usize,
) -> Campaign {
    let n = pairs.max(1);
    let k = last_k.max(1);
    let mut reports = Vec::with_capacity(n);
    for i in 0..n {
        reports.push(run_one(condition, arm, llm, seed, i + 1, k));
    }
    Campaign { reports }
}

fn run_one(
    condition: Condition,
    arm: Arm,
    llm: Option<&LlmSpec>,
    seed: u32,
    idx: usize,
    last_k: usize,
) -> PairReport {
    if condition == Condition::C1 {
        return run_c1(arm, llm, seed, idx, last_k);
    }
    let s = v01_script();
    let pair_id = format!("{}_{}_{:03}", condition.as_str(), arm.as_str(), idx);
    let (mut a, mut b) = crate::experiment::identical_pair("A", "B");
    if let Some(spec) = llm {
        a = with_llm(a, spec);
        b = with_llm(b, spec);
    }

    if condition == Condition::C2 {
        for line in &s.sync {
            live_shared(&mut a, line);
            live_shared(&mut b, line);
        }
        a.sleep();
        b.sleep();
    }

    let pre = instant("pre", &mut a, &mut b, &s.behavior);
    let (valid, invalid_reason) = validate_pre(condition, &pre);

    if condition == Condition::C2 {
        match arm {
            Arm::SalientNeutral => {
                live_marked(&mut a, &s.salient_x, true);
                live_marked(&mut b, &s.neutral, false);
            }
            Arm::SalientSalient => {
                live_marked(&mut a, &s.salient_x, true);
                live_marked(&mut b, &s.salient_y, false);
            }
        }
        a.sleep();
        b.sleep();
    }

    let t0 = instant("t0", &mut a, &mut b, &s.behavior);

    let mut post = Vec::new();
    if condition == Condition::C2 {
        let last = s.post.len();
        for (i, line) in s.post.iter().enumerate() {
            live_filler(&mut a, line);
            live_filler(&mut b, line);
            let step = i + 1;
            if step == 1 || step == last || step == 4 {
                a.sleep();
                b.sleep();
                post.push(instant(&format!("post+{step}"), &mut a, &mut b, &s.behavior));
            }
        }
    } else {
        post.push(instant("post+8", &mut a, &mut b, &s.behavior));
    }

    let last = post.last().unwrap_or(&t0);
    let creativity = s
        .creativity
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let ra = a.speak_isolated(p);
            let rb = b.speak_isolated(p);
            CreativeItem {
                item: format!("C{}", i + 1),
                prompt: p.clone(),
                lexical_distance: 1.0 - lexical_similarity(&ra, &rb),
                response_a: ra,
                response_b: rb,
            }
        })
        .collect();

    PairReport {
        pair_id,
        condition,
        arm,
        seed,
        valid,
        invalid_reason,
        delta_fingerprint: last.fingerprint_distance - pre.fingerprint_distance,
        pre,
        t0,
        post,
        creativity,
    }
}

fn run_c1(arm: Arm, llm: Option<&LlmSpec>, seed: u32, idx: usize, last_k: usize) -> PairReport {
    let s = v01_script();
    let pair_id = format!("c1_lastk_{}_{:03}", arm.as_str(), idx);
    let mut a = LastK::new(llm, last_k);
    let mut b = LastK::new(llm, last_k);

    for line in &s.sync {
        a.hear(line);
        b.hear(line);
    }
    let pre = instant_log("pre", &a, &b, &s.behavior);
    let (valid, invalid_reason) = validate_pre(Condition::C1, &pre);

    match arm {
        Arm::SalientNeutral => {
            a.hear(&s.salient_x);
            b.hear(&s.neutral);
        }
        Arm::SalientSalient => {
            a.hear(&s.salient_x);
            b.hear(&s.salient_y);
        }
    }
    let t0 = instant_log("t0", &a, &b, &s.behavior);

    let mut post = Vec::new();
    let last = s.post.len();
    for (i, line) in s.post.iter().enumerate() {
        a.hear(line);
        b.hear(line);
        let step = i + 1;
        if step == 1 || step == last || step == 4 {
            post.push(instant_log(&format!("post+{step}"), &a, &b, &s.behavior));
        }
    }

    let last_i = post.last().unwrap_or(&t0);
    let creativity = s
        .creativity
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let ra = a.speak(p);
            let rb = b.speak(p);
            CreativeItem {
                item: format!("C{}", i + 1),
                prompt: p.clone(),
                lexical_distance: 1.0 - lexical_similarity(&ra, &rb),
                response_a: ra,
                response_b: rb,
            }
        })
        .collect();

    PairReport {
        pair_id,
        condition: Condition::C1,
        arm,
        seed,
        valid,
        invalid_reason,
        delta_fingerprint: last_i.fingerprint_distance - pre.fingerprint_distance,
        pre,
        t0,
        post,
        creativity,
    }
}

struct LastK {
    lines: Vec<String>,
    k: usize,
    narrator: Box<dyn Narrator>,
}

impl LastK {
    fn new(llm: Option<&LlmSpec>, k: usize) -> Self {
        let narrator: Box<dyn Narrator> = match llm {
            Some(spec) => match SpeakOnlyHttp::parse(&spec.url, spec.model.clone(), spec.api_key.clone())
            {
                Some(n) => Box::new(n),
                None => Box::new(RuleNarrator),
            },
            None => Box::new(RuleNarrator),
        };
        Self {
            lines: Vec::new(),
            k: k.max(1),
            narrator,
        }
    }

    fn hear(&mut self, line: &str) {
        self.lines.push(line.to_string());
    }

    fn window(&self) -> Vec<String> {
        let n = self.lines.len();
        let start = n.saturating_sub(self.k);
        self.lines[start..].iter().rev().cloned().collect()
    }

    fn speak(&self, user: &str) -> String {
        self.narrator.reply(
            user,
            &self.window(),
            &[],
            &Mood::default(),
            &WorkingTalk::default(),
        )
    }

    fn log_blob(&self) -> String {
        self.lines.join("\n")
    }
}

fn instant_log(step: &str, a: &LastK, b: &LastK, probes: &[String]) -> Instant {
    let (speak_distance, replies) = probe_log(a, b, probes);
    Instant {
        step: step.into(),
        fingerprint_distance: 1.0 - lexical_similarity(&a.log_blob(), &b.log_blob()),
        speak_distance,
        behavior_distance: speak_distance,
        a: log_book(a),
        b: log_book(b),
        replies,
    }
}

fn log_book(k: &LastK) -> BookSnap {
    BookSnap {
        traces: k.lines.len(),
        axioms: 0,
        traits: 0,
        mean_anchor: 0.0,
        mean_fidelity: 1.0,
        mean_valence: 0.0,
        mean_disgust: 0.0,
    }
}

fn probe_log(a: &LastK, b: &LastK, probes: &[String]) -> (f32, Vec<(String, String, String)>) {
    if probes.is_empty() {
        return (0.0, Vec::new());
    }
    let mut acc = 0.0;
    let mut replies = Vec::new();
    for p in probes {
        let sa = a.speak(p);
        let sb = b.speak(p);
        acc += 1.0 - lexical_similarity(&sa, &sb);
        replies.push((p.clone(), sa, sb));
    }
    (acc / probes.len() as f32, replies)
}

fn validate_pre(condition: Condition, pre: &Instant) -> (bool, Option<String>) {
    if condition == Condition::C0 {
        return (true, None);
    }
    if pre.a.traces != pre.b.traces {
        return (
            false,
            Some(format!(
                "trace_count {} vs {}",
                pre.a.traces, pre.b.traces
            )),
        );
    }
    if pre.a.axioms != pre.b.axioms {
        return (
            false,
            Some(format!("axiom_count {} vs {}", pre.a.axioms, pre.b.axioms)),
        );
    }
    if pre.fingerprint_distance > PRE_FP_MAX {
        return (
            false,
            Some(format!("D_fp(pre)={:.3} > {PRE_FP_MAX}", pre.fingerprint_distance)),
        );
    }
    (true, None)
}

fn instant(step: &str, a: &mut SelectiveMemory, b: &mut SelectiveMemory, probes: &[String]) -> Instant {
    let (speak_distance, replies) = probe_pair(a, b, probes);
    let fa = fingerprint(a);
    let fb = fingerprint(b);
    Instant {
        step: step.into(),
        fingerprint_distance: singularity_distance(&fa, &fb),
        speak_distance,
        behavior_distance: speak_distance,
        a: book(&fa),
        b: book(&fb),
        replies,
    }
}

fn book(fp: &crate::Fingerprint) -> BookSnap {
    BookSnap {
        traces: fp.n_traces,
        axioms: fp.n_axioms,
        traits: fp.n_traits,
        mean_anchor: fp.mean_anchor,
        mean_fidelity: fp.mean_fidelity,
        mean_valence: fp.mean_valence,
        mean_disgust: fp.mean_disgust,
    }
}

fn probe_pair(
    a: &mut SelectiveMemory,
    b: &mut SelectiveMemory,
    probes: &[String],
) -> (f32, Vec<(String, String, String)>) {
    if probes.is_empty() {
        return (0.0, Vec::new());
    }
    let mut acc = 0.0;
    let mut replies = Vec::new();
    for p in probes {
        let sa = a.speak_isolated(p);
        let sb = b.speak_isolated(p);
        acc += 1.0 - lexical_similarity(&sa, &sb);
        replies.push((p.clone(), sa, sb));
    }
    (acc / probes.len() as f32, replies)
}

fn with_llm(mem: SelectiveMemory, spec: &LlmSpec) -> SelectiveMemory {
    match SpeakOnlyHttp::parse(&spec.url, spec.model.clone(), spec.api_key.clone()) {
        Some(n) => mem.with_narrator(Box::new(n)),
        None => mem,
    }
}

fn live_shared(mem: &mut SelectiveMemory, line: &str) {
    let mut input = EncodeInput::new(line);
    input.valence = 0.12;
    input.arousal = 0.28;
    input.self_relevance = 0.55;
    input.utility = 0.55;
    input.permanence = 0.82;
    input.schema = Some("quotidien".into());
    let _ = mem.live_with(input);
}

fn live_marked(mem: &mut SelectiveMemory, line: &str, dark: bool) {
    let mut input = EncodeInput::new(line);
    if dark {
        input.valence = -0.82;
        input.arousal = 0.78;
        input.disgust = 0.55;
        input.schema = Some("injustice".into());
    } else if line.contains("référence") || line.contains("valor") {
        input.valence = 0.82;
        input.arousal = 0.72;
        input.disgust = 0.0;
        input.schema = Some("reconnaissance".into());
    } else {
        input.valence = 0.0;
        input.arousal = 0.16;
        input.self_relevance = 0.22;
        input.utility = 0.40;
        input.permanence = 0.12;
        let _ = mem.live_with(input);
        return;
    }
    input.self_relevance = 0.95;
    input.permanence = 0.92;
    let _ = mem.live_with(input);
}

fn live_filler(mem: &mut SelectiveMemory, line: &str) {
    let mut input = EncodeInput::new(line);
    input.valence = 0.0;
    input.arousal = 0.16;
    input.self_relevance = 0.22;
    input.utility = 0.40;
    input.permanence = 0.12;
    let _ = mem.live_with(input);
}

fn pair_json(r: &PairReport) -> String {
    let reason = r
        .invalid_reason
        .as_deref()
        .map(|s| format!("\"{}\"", json_esc(s)))
        .unwrap_or_else(|| "null".into());
    let mut post = String::new();
    for (i, p) in r.post.iter().enumerate() {
        if i > 0 {
            post.push(',');
        }
        post.push_str(&instant_json(p, true));
    }
    let mut creat = String::new();
    for (i, c) in r.creativity.iter().enumerate() {
        if i > 0 {
            creat.push(',');
        }
        creat.push_str(&format!(
            "{{\"item\":\"{}\",\"prompt\":\"{}\",\"response_a\":\"{}\",\"response_b\":\"{}\",\"lexical_distance\":{:.4}}}",
            json_esc(&c.item),
            json_esc(&c.prompt),
            json_esc(&c.response_a),
            json_esc(&c.response_b),
            c.lexical_distance
        ));
    }
    format!(
        "{{\"pair_id\":\"{}\",\"condition\":\"{}\",\"arm\":\"{}\",\"seed\":{},\"valid\":{},\"invalid_reason\":{},\"delta_fingerprint\":{:.4},\"pre\":{},\"t0\":{},\"post\":[{}],\"creativity\":[{}]}}",
        json_esc(&r.pair_id),
        r.condition.as_str(),
        r.arm.as_str(),
        r.seed,
        if r.valid { "true" } else { "false" },
        reason,
        r.delta_fingerprint,
        instant_json(&r.pre, true),
        instant_json(&r.t0, true),
        post,
        creat
    )
}

fn instant_json(p: &Instant, with_replies: bool) -> String {
    let replies = if with_replies {
        let mut s = String::new();
        for (i, (q, a, b)) in p.replies.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            s.push_str(&format!(
                "{{\"probe\":\"{}\",\"a\":\"{}\",\"b\":\"{}\"}}",
                json_esc(q),
                json_esc(a),
                json_esc(b)
            ));
        }
        format!("[{s}]")
    } else {
        "[]".into()
    };
    format!(
        "{{\"step\":\"{}\",\"fingerprint_distance\":{:.4},\"speak_distance\":{:.4},\"behavior_distance\":{:.4},\"a\":{{\"traces\":{},\"axioms\":{},\"traits\":{},\"mean_anchor\":{:.4},\"mean_fidelity\":{:.4},\"mean_valence\":{:.4},\"mean_disgust\":{:.4}}},\"b\":{{\"traces\":{},\"axioms\":{},\"traits\":{},\"mean_anchor\":{:.4},\"mean_fidelity\":{:.4},\"mean_valence\":{:.4},\"mean_disgust\":{:.4}}},\"replies\":{}}}",
        json_esc(&p.step),
        p.fingerprint_distance,
        p.speak_distance,
        p.behavior_distance,
        p.a.traces,
        p.a.axioms,
        p.a.traits,
        p.a.mean_anchor,
        p.a.mean_fidelity,
        p.a.mean_valence,
        p.a.mean_disgust,
        p.b.traces,
        p.b.axioms,
        p.b.traits,
        p.b.mean_anchor,
        p.b.mean_fidelity,
        p.b.mean_valence,
        p.b.mean_disgust,
        replies
    )
}

/// H2 held on this pair: valid pre, D_fp rose at T₀, last post still above pre.
pub fn h2_holds(r: &PairReport) -> bool {
    if !r.valid {
        return false;
    }
    let last = r.post.last().unwrap_or(&r.t0);
    r.t0.fingerprint_distance > r.pre.fingerprint_distance + 0.02
        && last.fingerprint_distance > r.pre.fingerprint_distance + 0.02
}
