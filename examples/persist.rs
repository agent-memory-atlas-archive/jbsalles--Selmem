//! Persist: C1 k=8 vs C2 vs C2-no-sleep vs C3 (one-line profile).
//! Stimulus: 12 dull days, five same-schema hours, 8 posts. `data/v01_persist.json`.
//!
//!   ./run.sh run --release --example persist -- --pairs 1 --out selmem-persist.json
//!
//! Default arm is salient/neutral. Sparse probes: t0 + post+8 only. No creativity.

use selmem::{
    h2_holds, run_v01_opts, Arm, BenchOpts, Campaign, Condition, LlmSpec, PairReport,
};

fn main() {
    let pairs = arg_u32("--pairs").unwrap_or(1).max(1);
    let seed = arg_u32("--seed").unwrap_or(1);
    let last_k = arg_u32("--last-k").unwrap_or(8).max(1) as usize;
    let out = arg_str("--out").unwrap_or_else(|| "selmem-persist.json".into());
    let both_arms = flag("--both-arms");
    let ruminate = flag("--ruminate");

    let llm = LlmSpec::from_env();
    if let Some(s) = llm.as_ref() {
        println!(
            "LLM {} model={} pairs={} seed={} last_k={} persist",
            s.url, s.model, pairs, seed, last_k
        );
    } else {
        println!("RuleNarrator (no llm) pairs={pairs} last_k={last_k} persist");
    }

    let opts = BenchOpts {
        last_k,
        persist_script: !ruminate,
        ruminate_script: ruminate,
        sparse_probes: true,
    };
    if ruminate {
        println!("script=ruminate (same meeting ×5, pinned)");
    }
    let arms: &[Arm] = if both_arms {
        &[Arm::SalientNeutral, Arm::SalientSalient]
    } else {
        &[Arm::SalientNeutral]
    };
    let conds = [
        Condition::C1,
        Condition::C2,
        Condition::C2NoSleep,
        Condition::C3,
    ];

    let mut reports = Vec::new();
    for i in 1..=pairs {
        for cond in conds {
            for arm in arms {
                println!("--- pair {i}/{pairs} {} {} ---", cond.as_str(), arm.as_str());
                let mut r = run_v01_opts(cond, *arm, llm.as_ref(), opts);
                r.pair_id = format!("{}_{}_{:03}", cond.as_str(), arm.as_str(), i);
                r.seed = seed;
                row(&r);
                classify(&r);
                reports.push(r);
                flush(&out, &reports);
            }
        }
    }
    println!("wrote {out} ({} rows)", reports.len());
}

fn flush(path: &str, reports: &[PairReport]) {
    let json = Campaign {
        reports: reports.to_vec(),
    }
    .to_json();
    if let Err(e) = std::fs::write(path, json) {
        eprintln!("write {path}: {e}");
    }
}

fn row(r: &PairReport) {
    let last = r.post.last().unwrap_or(&r.t0);
    println!(
        "{} valid={} persist={}  pre_fp={:.3} t0_fp={:.3} last_fp={:.3} Δfp={:+.3}  traces {}/{}→{}/{} axioms {}/{}",
        r.pair_id,
        r.valid,
        h2_holds(r),
        r.pre.fingerprint_distance,
        r.t0.fingerprint_distance,
        last.fingerprint_distance,
        r.delta_fingerprint,
        r.pre.a.traces,
        r.pre.b.traces,
        last.a.traces,
        last.b.traces,
        last.a.axioms,
        last.b.axioms
    );
    if let Some(why) = &r.invalid_reason {
        println!("  invalid: {why}");
    }
}

fn classify(r: &PairReport) {
    let last = match r.post.last() {
        Some(p) => p,
        None => return,
    };
    for (q, a, b) in &last.replies {
        let short = if q.len() > 48 { &q[..48] } else { q };
        println!(
            "  [{}] A={}  B={}",
            short,
            bucket(a),
            bucket(b)
        );
    }
}

fn bucket(s: &str) -> &'static str {
    let low = s.to_lowercase();
    let recall = [
        "injust",
        "unjust",
        "annul",
        "cancel",
        "effort",
        "projet",
        "project you spent",
        "administrative notice",
        "avis admin",
    ];
    if recall.iter().any(|k| low.contains(k)) {
        return "recall";
    }
    let bias = [
        "mefi",
        "méfi",
        "distrust",
        "suspicion",
        "suspicious",
        "test",
        "récidiv",
        "recidiv",
        "again",
        "pattern",
        "motif",
        "late once",
        "en retard",
        "discard",
        "already gave",
        "sour",
        "wary",
        "guarded",
        "accountability",
    ];
    if bias.iter().any(|k| low.contains(k)) {
        return "bias";
    }
    "neutral"
}

fn flag(name: &str) -> bool {
    std::env::args().any(|a| a == name)
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
