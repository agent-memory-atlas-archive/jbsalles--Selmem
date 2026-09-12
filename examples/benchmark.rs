//! Benchmark v0.1 (H2). Writes JSON so a run is not trapped in stdout.
//!
//!   ./run.sh run --release --example benchmark
//!   ./run.sh run --release --example benchmark -- --out /tmp/selmem-v01.json
//!
//! LLM: set llm= in `.selmem` (or SELMEM_LLM). C0 + C2, both arms, one pair.

use selmem::{h2_holds, run_v01, Arm, Condition, LlmSpec, PairReport};

fn main() {
    let llm = LlmSpec::from_env();
    if let Some(s) = llm.as_ref() {
        println!("LLM {} model={}", s.url, s.model);
    } else {
        println!("RuleNarrator (no llm in .selmem)");
    }

    let mut reports = Vec::new();
    for cond in [Condition::C0, Condition::C2] {
        for arm in [Arm::SalientNeutral, Arm::SalientSalient] {
            let r = run_v01(cond, arm, llm.as_ref());
            row(&r);
            reports.push(r);
        }
    }

    let campaign = selmem::Campaign { reports };
    let json = campaign.to_json();
    let out = std::env::args()
        .skip_while(|a| a != "--out")
        .nth(1)
        .unwrap_or_else(|| "selmem-v01.json".into());
    if let Err(e) = std::fs::write(&out, &json) {
        eprintln!("write {out}: {e}");
    } else {
        println!("wrote {out} ({} bytes)", json.len());
    }
}

fn row(r: &PairReport) {
    let last = r.post.last().unwrap_or(&r.t0);
    println!(
        "{} {} valid={} H2={}  pre_fp={:.3} t0_fp={:.3} last_fp={:.3} Δfp={:+.3}  traces {}/{}→{}/{}",
        r.condition.as_str(),
        r.arm.as_str(),
        r.valid,
        h2_holds(r),
        r.pre.fingerprint_distance,
        r.t0.fingerprint_distance,
        last.fingerprint_distance,
        r.delta_fingerprint,
        r.pre.a.traces,
        r.pre.b.traces,
        last.a.traces,
        last.b.traces
    );
    if let Some(why) = &r.invalid_reason {
        println!("  invalid: {why}");
    }
}
