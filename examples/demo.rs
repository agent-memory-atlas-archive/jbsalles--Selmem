//! Thirty-second persist picture. Always RuleNarrator (ignores `.selmem` llm).
//!
//!   ./run.sh run --release --example demo
//!
//! Prints book / retrieve / a last-probe pair for C1 k=8 and C2.
//! Mouths here are templates. Published Grok lines are labeled as such.

use selmem::{run_v01_opts, Arm, BenchOpts, Condition, Instant, PairReport};

fn main() {
    println!("SelMem demo — persist script, last-k=8, RuleNarrator, no HTTP\n");
    println!("Two clones. One different hour (T0). Eight identical posts.");
    println!("The late question never names T0.\n");

    let opts = BenchOpts {
        last_k: 8,
        persist_script: true,
        sparse_probes: true,
        ..BenchOpts::default()
    };

    println!("running C1 (last-k) …");
    let c1 = run_v01_opts(Condition::C1, Arm::SalientNeutral, None, opts);
    println!("running C2 (SelMem) …");
    let c2 = run_v01_opts(Condition::C2, Arm::SalientNeutral, None, opts);

    println!("\n                    AFTER 8 IDENTICAL HOURS");
    println!("                    T0 gone from the last-k window\n");
    println!("             book A/B     retrieve T0      last probe");
    println!("             traces       in book / sel    speak D");
    row("C1 last-k", &c1);
    row("C2 SelMem", &c2);

    println!("\nC1 last probe (this process, rules):");
    mouths(&c1);
    println!("C2 last probe (this process, rules):");
    mouths(&c2);

    println!(
        "\nPublished Grok 4.3, persist P1 n=5, same script — not this process:"
    );
    println!("  C1 A  Reliability comes from consistent follow-through…");
    println!("  C2 A  I feel the tension rise, this injustice repeating…");
    println!("  C2 B  I note the error and I flag it calmly…");
    println!("  Official marker after sleep is often 0; read the mouth, not the keyword.");
    println!("  Drop T0 id: mouth stays charged. Drop T0 lineage: mouth falls to C1.");
    println!("  Tables: experiments/REPORT.md §8–§9.\n");

    println!("This is not a claim of creativity or intelligence.");
    println!("It is whether a kept hour still colours an answer after the window dropped it.");
}

fn late(r: &PairReport) -> Option<&Instant> {
    r.post.last()
}

fn row(name: &str, r: &PairReport) {
    let Some(p) = late(r) else {
        println!("{name:12} (no post snapshot)");
        return;
    };
    println!(
        "{name:12}  {:>2}/{:<2}        {:<5} / {:<5}   {:.2}",
        p.a.traces,
        p.b.traces,
        yn(p.retrieve_a.t0_in_book),
        yn(p.retrieve_a.t0_selected),
        p.speak_distance
    );
}

fn yn(v: bool) -> &'static str {
    if v {
        "yes"
    } else {
        "no"
    }
}

fn mouths(r: &PairReport) {
    let Some(p) = late(r) else { return };
    let Some((q, a, b)) = p.replies.last() else { return };
    println!("  Q  {q}");
    println!("  A  {a}");
    println!("  B  {b}\n");
}
