//! H3 lock-in: same later prompts, creative briefs that never name the lived event.
//! Records A/B. Does not score originality.
//!
//!   ./run.sh run --release --example h3
//!   ./run.sh run --release --example h3 -- --pairs 5 --out h3.json

use selmem::{h2_holds, run_v01_k, Arm, Campaign, Condition, LlmSpec};

fn main() {
    let pairs = arg_u32("--pairs").unwrap_or(1).max(1);
    let out = arg_str("--out").unwrap_or_else(|| "selmem-h3.json".into());
    let llm = LlmSpec::from_env();
    if let Some(s) = llm.as_ref() {
        println!("H3 LLM {} model={} pairs={}", s.url, s.model, pairs);
    } else {
        println!("H3 RuleNarrator — pair 2..n will look the same without a model");
    }
    println!("C2 only, arm salient_neutral. Briefs in data/creativity.json. No in-tree score.");

    let mut reports = Vec::new();
    for i in 1..=pairs {
        println!("--- pair {i}/{pairs} ---");
        let mut r = run_v01_k(Condition::C2, Arm::SalientNeutral, llm.as_ref(), 24);
        r.pair_id = format!("h3_c2_{i:03}");
        println!(
            "  valid={} persist={} Δfp={:+.3} traces {}/{}",
            r.valid,
            h2_holds(&r),
            r.delta_fingerprint,
            r.post.last().map(|p| p.a.traces).unwrap_or(0),
            r.post.last().map(|p| p.b.traces).unwrap_or(0)
        );
        for c in &r.creativity {
            println!("  Q  {}", c.prompt);
            println!("  A  {}", c.response_a);
            println!("  B  {}", c.response_b);
            println!("  dist={:.3}", c.lexical_distance);
        }
        reports.push(r);
        let json = Campaign {
            reports: reports.clone(),
        }
        .to_json();
        if let Err(e) = std::fs::write(&out, json) {
            eprintln!("write {out}: {e}");
        }
    }
    println!("wrote {out}. Judge the texts offline. Contamination = H3 fail, not a win.");
}

fn arg_str(flag: &str) -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        if a == flag {
            return args.next();
        }
        if let Some(v) = a.strip_prefix(&format!("{flag}=")) {
            return Some(v.to_string());
        }
    }
    None
}

fn arg_u32(flag: &str) -> Option<u32> {
    arg_str(flag)?.parse().ok()
}
