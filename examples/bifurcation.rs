//! Memory bifurcation. RuleNarrator by default; LLM replies if SELMEM_LLM is set.
//!
//!   ./run.sh run --release --example bifurcation
//!
//!   SELMEM_LLM=https://api.openai.com/v1/chat/completions \
//!   SELMEM_MODEL=gpt-4o-mini \
//!   SELMEM_API_KEY=sk-... \
//!   ./run.sh run --release --example bifurcation

use selmem::{
    run_neutral, run_neutral_llm, run_salient, run_salient_llm, run_salient_without_sleep,
    BifurcationReport, LlmSpec,
};

fn row(title: &str, r: &BifurcationReport) {
    println!("=== {} ===", r.condition);
    println!(
        "  pre     fp={:.3} speak={:.3} traces={}/{}",
        r.pre.fingerprint_distance, r.pre.speak_distance, r.pre.a_traces, r.pre.b_traces
    );
    println!(
        "  t0      fp={:.3} speak={:.3} traces={}/{}",
        r.immediate.fingerprint_distance,
        r.immediate.speak_distance,
        r.immediate.a_traces,
        r.immediate.b_traces
    );
    for s in &r.persist {
        println!(
            "  {:<7} fp={:.3} speak={:.3} axioms={}/{}",
            s.label, s.fingerprint_distance, s.speak_distance, s.a_axioms, s.b_axioms
        );
    }
    println!(
        "  Δspeak={:+.3}  Δfingerprint={:+.3}  ({title})",
        r.delta_speak, r.delta_fingerprint
    );
    if let Some(last) = r.persist.last() {
        if !last.replies.is_empty() {
            println!("  --- last probes ---");
            let mut rules = 0;
            let n = last.replies.len();
            for (probe, a, b) in &last.replies {
                println!("  Q  {}", probe);
                println!("  A  {}", a);
                println!("  B  {}", b);
                if looks_like_rules(a) {
                    rules += 1;
                }
                if looks_like_rules(b) {
                    rules += 1;
                }
            }
            if n > 0 {
                println!(
                    "  narrator: {}/{} answers are the RuleNarrator template (Cela me revient)",
                    rules,
                    n * 2
                );
            }
        }
    }
    println!();
}

fn looks_like_rules(s: &str) -> bool {
    s.contains("Cela me revient") || s.contains("Je reconnais un motif")
}

fn main() {
    let llm = LlmSpec::from_env();
    let (salient, neutral, ablated) = if let Some(spec) = llm.as_ref() {
        let key = if spec.api_key.as_deref().unwrap_or("").is_empty() {
            "KEY=missing"
        } else {
            "KEY=set"
        };
        println!("LLM replies via {} model={} {key}", spec.url, spec.model);
        if matches!(
            std::env::var("SELMEM_QUICK").as_deref(),
            Ok("1") | Ok("true") | Ok("yes")
        ) {
            println!("SELMEM_QUICK — salient LLM only, 2 probes, one persist window");
            (
                run_salient_llm(spec),
                run_neutral(),
                run_salient_without_sleep(),
            )
        } else {
            (
                run_salient_llm(spec),
                run_neutral_llm(spec),
                run_salient_without_sleep(),
            )
        }
    } else {
        println!("no llm in .selmem / SELMEM_LLM — RuleNarrator only");
        (run_salient(), run_neutral(), run_salient_without_sleep())
    };
    row("salient event on A only", &salient);
    row("length-matched neutral event on A", &neutral);
    row("salient but no sleep / consolidation", &ablated);
    let ok = salient.delta_speak > neutral.delta_speak
        && salient.immediate.fingerprint_distance > salient.pre.fingerprint_distance;
    println!(
        "{}",
        if ok {
            "H1 vs H0 (this run): salient Δ > neutral Δ, and T0 moved the fingerprint."
        } else {
            "H0 not rejected on this run."
        }
    );
}
