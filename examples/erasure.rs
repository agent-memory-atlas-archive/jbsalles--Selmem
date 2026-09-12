//! One-shot trivia should fade. A repeated aversion should not.
//!
//!   ./run.sh run --release --example erasure

use selmem::{run_erasure, LlmSpec};

fn main() {
    let r = run_erasure(LlmSpec::from_env().as_ref());
    println!("encoded  color={}  aversion={}", r.color_kept_at_encode, r.aversion_kept_at_encode);
    println!("traces after life+nights: {}", r.n_traces);
    println!("recalled color={}  aversion={}", r.color_recalled, r.aversion_recalled);
    println!("Q couleur  {}", r.color_answer);
    println!("Q interrompu  {}", r.aversion_answer);
    if !r.color_recalled && r.aversion_recalled {
        println!("erasure holds: trivia gone, repeated aversion still answers.");
    } else {
        println!("erasure did not separate the two facts on this run.");
    }
}
