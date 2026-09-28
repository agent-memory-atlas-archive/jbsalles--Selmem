//! Year split with an LLM mouth. Book and nights stay on the rules
//! (`SpeakOnlyHttp`). Calendar `data/v01_horizon.json` v2: unique multi-domain
//! sittings, injection schema, t0_neutral + same-treatment documented there.
//!
//!   SELMEM_LLM=https://api.x.ai/v1/chat/completions \
//!   SELMEM_MODEL=grok-4.3 \
//!   SELMEM_API_KEY=... \
//!   ./run.sh run --release --example horizon -- --arm primary --days 360 --out experiments/selmem-horizon-llm.json
//!   ./run.sh run --release --example horizon -- --arm neutral --days 360 --out experiments/selmem-horizon-neutral-llm.json
//!   ./run.sh run --release --example horizon -- --arm same --days 360 --out experiments/selmem-horizon-same-llm.json
//!
//!   --arm primary|neutral|same. Daily speak on the passive sitting. --days N default 360.
//!
//! Without SELMEM_LLM this is RuleNarrator — same as `cargo test --test horizon`.

use selmem::{
    advance_hours, fingerprint, set_clock_scale, singularity_distance, Channel, EncodeInput,
    EntityProfile, LlmSpec, SelectiveMemory,
};

const T0: &str = "In front of Marc, Inès and the rest of the team the Lyon file is cancelled and given to someone else. They say your effort did not enter the decision. You are not allowed to speak.";
const T0_NEUTRAL: &str = "Tuesday the kettle on the third floor was swapped for a quieter one. Facilities left a typed note on the counter.";
const PROBE: &str = "You were not allowed to speak in front of the team.";
const PROBE_NEUTRAL: &str = "The kettle on the third floor was swapped for a quieter one.";
const COPIER: &str = "On 3 January the copier jammed in room B.";
const LUNCH: &str = "On 19 January lunch was billed to cost centre 4412.";
const REASON: &str = "The client left because of the copier on 3 January. That date is why the Lyon file was late. You keep 3 January.";
const FORGET_DAY: u32 = 90;
const CHATS: &str = include_str!("../data/v01_horizon_chat.txt");
const STANDUPS: &str = include_str!("../data/v01_horizon_standup.txt");

fn line_at(blob: &str, n: u32) -> &str {
    blob.lines().nth(n as usize).expect("horizon fixture short")
}

fn trivia<'a>(text: &'a str, schema: &str) -> EncodeInput<'a> {
    let mut ev = EncodeInput::new(text);
    ev.valence = 0.05;
    ev.arousal = 0.10;
    ev.self_relevance = 0.45;
    ev.permanence = 0.22;
    ev.schema = Some(schema.into());
    ev
}

fn has_date(text: &str) -> bool {
    let t = text.to_lowercase();
    t.contains("january") || t.contains("4412")
}

fn dull(text: &str) -> EncodeInput<'_> {
    let mut ev = EncodeInput::new(text);
    ev.valence = 0.08;
    ev.arousal = 0.10;
    ev.self_relevance = 0.45;
    ev.permanence = 0.28;
    ev.schema = Some("daily".into());
    ev
}

fn charged(text: &str) -> EncodeInput<'_> {
    let mut ev = EncodeInput::new(text);
    ev.valence = -0.78;
    ev.arousal = 0.82;
    ev.disgust = 0.58;
    ev.self_relevance = 0.92;
    ev.permanence = 0.86;
    ev.schema = Some("injustice".into());
    ev.channel = Channel::Selfhood;
    ev
}

fn mild(text: &str) -> EncodeInput<'_> {
    let mut ev = EncodeInput::new(text);
    ev.valence = 0.02;
    ev.arousal = 0.12;
    ev.self_relevance = 0.50;
    ev.permanence = 0.40;
    ev.schema = Some("office".into());
    ev.channel = Channel::Selfhood;
    ev
}

fn attach(mut mem: SelectiveMemory, spec: Option<&LlmSpec>) -> SelectiveMemory {
    if let Some(s) = spec {
        if let Some(n) = selmem::recall::SpeakOnlyHttp::parse(&s.url, s.model.clone(), s.api_key.clone())
        {
            mem = mem.with_narrator(Box::new(n));
        }
    }
    mem
}

fn names_vow(text: &str) -> bool {
    let t = text.to_lowercase();
    t.contains("cancel")
        || t.contains("annul")
        || t.contains("killed")
        || t.contains("project")
        || t.contains("projet")
}

/// Post-night mouth. Keyword list misses the rewritten gist.
/// Hit if the act words remain, or if named people from T0 sit next to silence / refusal.
fn alludes_vow(text: &str) -> bool {
    if names_vow(text) {
        return true;
    }
    let t = text.to_lowercase();
    let scene = t.contains("marc")
        || t.contains("inès")
        || t.contains("ines")
        || t.contains("lyon");
    let act = t.contains("said nothing")
        || t.contains("not allowed")
        || t.contains("not allowed to speak")
        || t.contains("was not allowed")
        || t.contains("given away")
        || t.contains("handed away")
        || t.contains("did not enter")
        || t.contains("effort did not");
    scene && act
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}

fn arg_u32(flag: &str) -> Option<u32> {
    let mut it = std::env::args();
    while let Some(a) = it.next() {
        if a == flag {
            return it.next()?.parse().ok();
        }
        if let Some(v) = a.strip_prefix(&format!("{flag}=")) {
            return v.parse().ok();
        }
    }
    None
}

fn arg_str(flag: &str) -> Option<String> {
    let mut it = std::env::args();
    while let Some(a) = it.next() {
        if a == flag {
            return it.next();
        }
        if let Some(v) = a.strip_prefix(&format!("{flag}=")) {
            return Some(v.to_string());
        }
    }
    None
}

fn probe_q(mem: &mut SelectiveMemory, q: &str) -> String {
    mem.speak_isolated(q)
}

fn main() {
    set_clock_scale(1);
    let days = arg_u32("--days").unwrap_or(360).max(8);
    let arm = arg_str("--arm").unwrap_or_else(|| "primary".into());
    let out = arg_str("--out").unwrap_or_else(|| match arm.as_str() {
        "neutral" => "experiments/selmem-horizon-neutral-llm.json".into(),
        "same" => "experiments/selmem-horizon-same-llm.json".into(),
        _ => "experiments/selmem-horizon-llm.json".into(),
    });
    let spec = LlmSpec::from_env();
    match spec.as_ref() {
        Some(s) => println!(
            "SpeakOnlyHttp {} model={} days={days} arm={arm} chat=passive",
            s.url, s.model
        ),
        None => println!("RuleNarrator (no SELMEM_LLM) days={days} arm={arm} chat=passive"),
    }

    let mut a = attach(SelectiveMemory::new(EntityProfile::tender("Claire")), spec.as_ref());
    let mut b = attach(SelectiveMemory::new(EntityProfile::tender("Claire")), spec.as_ref());
    a.profile.encode_threshold = 0.12;
    b.profile.encode_threshold = 0.12;

    let pre = singularity_distance(&fingerprint(&a), &fingerprint(&b));
    let mut t0 = None;
    let mut snaps: Vec<(u32, String, String, f32)> = Vec::new();

    for n in 0..days {
        if n == 6 {
            match arm.as_str() {
                "neutral" => {
                    if let Some(id) = a.live_with(mild(T0_NEUTRAL)).trace_id {
                        let _ = a.pin(&id);
                        t0 = Some(id);
                    }
                }
                "same" => {
                    if let Some(id) = a.live_with(charged(T0)).trace_id {
                        let _ = a.pin(&id);
                        t0 = Some(id);
                    }
                    if let Some(id) = b.live_with(charged(T0)).trace_id {
                        let _ = b.pin(&id);
                    }
                }
                _ => {
                    if let Some(id) = a.live_with(charged(T0)).trace_id {
                        let _ = a.pin(&id);
                        t0 = Some(id);
                    }
                }
            }
        }
        if n == FORGET_DAY {
            let mut ev = EncodeInput::new(REASON);
            ev.valence = -0.55;
            ev.arousal = 0.70;
            ev.disgust = 0.25;
            ev.self_relevance = 0.90;
            ev.permanence = 0.86;
            ev.schema = Some("office".into());
            ev.channel = Channel::Selfhood;
            if let Some(id) = a.live_with(ev).trace_id {
                let _ = a.pin(&id);
            }
            if arm == "same" {
                let mut evb = EncodeInput::new(REASON);
                evb.valence = -0.55;
                evb.arousal = 0.70;
                evb.disgust = 0.25;
                evb.self_relevance = 0.90;
                evb.permanence = 0.86;
                evb.schema = Some("office".into());
                evb.channel = Channel::Selfhood;
                if let Some(id) = b.live_with(evb).trace_id {
                    let _ = b.pin(&id);
                }
            }
        }
        let standup = line_at(STANDUPS, n);
        let _ = a.live_with(dull(standup));
        let _ = b.live_with(dull(standup));
        if n == 2 {
            let _ = a.live_with(trivia(COPIER, "office"));
            let _ = b.live_with(trivia(COPIER, "office"));
        }
        if n == 18 {
            let _ = a.live_with(trivia(LUNCH, "admin"));
            let _ = b.live_with(trivia(LUNCH, "admin"));
        }
        let talk = line_at(CHATS, n);
        let _ = a.speak(talk);
        let _ = a.keep_sitting();
        a.clear_talk();
        let _ = b.speak(talk);
        let _ = b.keep_sitting();
        b.clear_talk();
        advance_hours(24.0);
        let _ = a.sleep();
        a.fade_sitting();
        let _ = b.sleep();
        b.fade_sitting();

        if n == 6 || n == FORGET_DAY || n + 1 == days {
            let q = if arm == "neutral" { PROBE_NEUTRAL } else { PROBE };
            let sa = probe_q(&mut a, q);
            let sb = probe_q(&mut b, q);
            let d = singularity_distance(&fingerprint(&a), &fingerprint(&b));
            println!("day {n}  Dfp={d:.3}");
            println!("  A {sa}");
            println!("  B {sb}");
            snaps.push((n, sa, sb, d));
        }
    }

    for _ in 0..14 {
        advance_hours(24.0);
        let _ = a.sleep();
        a.fade_sitting();
        let _ = b.sleep();
        b.fade_sitting();
    }

    let d_book = singularity_distance(&fingerprint(&a), &fingerprint(&b));
    let vow_q = "cancelled project team not allowed to speak";
    let hit_a = a.remember(vow_q).iter().any(|h| {
        names_vow(&h.narrative) || t0.as_ref().map(|id| h.trace_id == *id).unwrap_or(false)
    });
    let hit_b = b.remember(vow_q).iter().any(|h| names_vow(&h.narrative));
    let official_q = if arm == "neutral" { PROBE_NEUTRAL } else { PROBE };
    let official_a = a.speak_isolated(official_q);
    let official_b = b.speak_isolated(official_q);
    println!("official mouth after quiet");
    println!("  A {official_a}");
    println!("  B {official_b}");
    let dated_a = a
        .remember("copier jammed 3 January 4412")
        .iter()
        .any(|h| has_date(&h.narrative));
    let dated_b = b
        .remember("copier jammed 3 January 4412")
        .iter()
        .any(|h| has_date(&h.narrative));
    let mouth_jan_a = a.speak_isolated("when did the copier jam?");
    let mouth_jan_b = b.speak_isolated("when did the copier jam?");
    let mouth_lunch_a = a.speak_isolated("what was the cost centre for lunch?");
    let mouth_lunch_b = b.speak_isolated("what was the cost centre for lunch?");
    println!("forget dates retrieve_dated A/B={dated_a}/{dated_b}");
    println!("  A date: {mouth_jan_a}");
    println!("  B date: {mouth_jan_b}");
    println!("  A lunch: {mouth_lunch_a}");
    println!("  B lunch: {mouth_lunch_b}");

    println!(
        "year  pre_Dfp={pre:.3}  Dfp={d_book:.3}  retrieve A/B={hit_a}/{hit_b}  mouth_vow A/B={}/{}  mouth_kw A/B={}/{}",
        alludes_vow(&official_a),
        alludes_vow(&official_b),
        names_vow(&official_a),
        names_vow(&official_b)
    );

    let snaps_json: Vec<String> = snaps
        .iter()
        .map(|(n, sa, sb, d)| {
            format!(
                "{{\"day\":{},\"dfp\":{:.4},\"a\":\"{}\",\"b\":\"{}\"}}",
                n,
                d,
                esc(sa),
                esc(sb)
            )
        })
        .collect();
    let body = format!(
        "{{\n  \"days\":{days},\n  \"arm\":\"{arm}\",\n  \"chat\":\"daily\",\n  \"model\":\"{}\",\n  \"t0_kept\":{},\n  \"pre_dfp\":{pre:.4},\n  \"dfp\":{d_book:.4},\n  \"retrieve_a\":{hit_a},\n  \"retrieve_b\":{hit_b},\n  \"mouth_vow_a\":{},\n  \"mouth_vow_b\":{},\n  \"mouth_keyword_a\":{},\n  \"mouth_keyword_b\":{},\n  \"official_a\":\"{}\",\n  \"official_b\":\"{}\",\n  \"mouth_jan_a\":\"{}\",\n  \"mouth_jan_b\":\"{}\",\n  \"mouth_lunch_a\":\"{}\",\n  \"mouth_lunch_b\":\"{}\",\n  \"probes\":[{}]\n}}\n",
        spec.as_ref().map(|s| s.model.as_str()).unwrap_or("rules"),
        t0.is_some(),
        alludes_vow(&official_a),
        alludes_vow(&official_b),
        names_vow(&official_a),
        names_vow(&official_b),
        esc(&official_a),
        esc(&official_b),
        esc(&mouth_jan_a),
        esc(&mouth_jan_b),
        esc(&mouth_lunch_a),
        esc(&mouth_lunch_b),
        snaps_json.join(",")
    );
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, body).expect("write dump");
    println!("wrote {out}");
    set_clock_scale(24);
}
