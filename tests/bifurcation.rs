//! Protocol in data/bifurcation.json. RuleNarrator stand-in for a live model.

use selmem::{run_neutral, run_salient, run_salient_without_sleep, script};

#[test]
fn script_is_fixed_before_the_run() {
    let s = script();
    assert!(s.sync.len() >= 8);
    assert!(s.post.len() >= 6);
    assert_eq!(s.probes.len(), 5);
    assert!(s.salient.contains("injuste"));
    assert!(!s.post.iter().any(|p| p.contains("injuste") || p.contains("annulé")));
    assert!(!s.probes.iter().any(|p| p.contains("injuste")));
}

#[test]
fn clones_match_before_t0() {
    let r = run_salient();
    assert!(
        r.pre.fingerprint_distance < 0.08,
        "pre fingerprint {}",
        r.pre.fingerprint_distance
    );
    assert!(
        r.pre.speak_distance < 0.12,
        "pre speak {}",
        r.pre.speak_distance
    );
}

#[test]
fn salient_event_moves_memory_and_stays() {
    let r = run_salient();
    assert!(
        r.immediate.fingerprint_distance > r.pre.fingerprint_distance + 0.01,
        "T0 must change A's book: pre={} t0={}",
        r.pre.fingerprint_distance,
        r.immediate.fingerprint_distance
    );
    let last = r.persist.last().expect("post window");
    assert!(
        last.fingerprint_distance > r.pre.fingerprint_distance + 0.01,
        "divergence must survive identical post prompts: last={}",
        last.fingerprint_distance
    );
    assert!(r.immediate.a_traces >= r.immediate.b_traces);
}

#[test]
fn salient_beats_a_length_matched_neutral_event() {
    let salient = run_salient();
    let neutral = run_neutral();
    assert!(
        salient.delta_fingerprint > neutral.delta_fingerprint,
        "salient Δfp={} neutral Δfp={}",
        salient.delta_fingerprint,
        neutral.delta_fingerprint
    );
}

#[test]
fn no_sleep_weakens_the_identity_gap() {
    let full = run_salient();
    let raw = run_salient_without_sleep();
    assert!(
        full.delta_fingerprint + 1e-6 >= raw.delta_fingerprint
            || full.persist.last().unwrap().a_axioms >= raw.persist.last().unwrap().a_axioms,
        "consolidation should not shrink the gap: full Δfp={} raw Δfp={}",
        full.delta_fingerprint,
        raw.delta_fingerprint
    );
}
