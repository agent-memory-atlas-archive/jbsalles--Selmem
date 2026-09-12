//! Same questions, two marked lives.
//!
//!   ./run.sh run --release --example divergence
//!
//! With a model on speak:
//!   # or llm=/model=/api_key= in .selmem
//!   ./run.sh run --release --example divergence

use selmem::{run_split_lives, LlmSpec};

fn main() {
    let llm = LlmSpec::from_env();
    if let Some(s) = llm.as_ref() {
        println!("LLM replies via {} model={}", s.url, s.model);
    } else {
        println!("RuleNarrator (set llm in .selmem to speak with a model)");
    }
    let r = run_split_lives(llm.as_ref());
    println!(
        "pre     fp={:.3} speak={:.3} traces={}/{}",
        r.pre.fingerprint_distance, r.pre.speak_distance, r.pre.a_traces, r.pre.b_traces
    );
    println!(
        "marked  fp={:.3} speak={:.3} traces={}/{}",
        r.immediate.fingerprint_distance,
        r.immediate.speak_distance,
        r.immediate.a_traces,
        r.immediate.b_traces
    );
    for s in &r.persist {
        println!(
            "{:<8} fp={:.3} speak={:.3}",
            s.label, s.fingerprint_distance, s.speak_distance
        );
    }
    println!(
        "Δspeak={:+.3}  Δfingerprint={:+.3}",
        r.delta_speak, r.delta_fingerprint
    );
    if let Some(last) = r.persist.last() {
        println!("\n--- same questions, last window ---");
        for (probe, a, b) in &last.replies {
            println!("Q  {probe}");
            println!("A  {a}");
            println!("B  {b}\n");
        }
    }
}
