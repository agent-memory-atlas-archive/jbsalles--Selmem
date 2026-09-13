use selmem::{h2_holds, run_v01, run_v01_k, v01_script, Arm, Condition};

#[test]
fn v01_script_is_frozen_and_sized_for_the_proto() {
    let s = v01_script();
    assert_eq!(s.sync.len(), 12);
    assert_eq!(s.post.len(), 8);
    assert_eq!(s.behavior.len(), 4);
    assert_eq!(s.creativity.len(), 3);
    let blob = format!("{} {} {}", s.salient_x, s.salient_y, s.neutral);
    for p in s.behavior.iter().chain(s.post.iter()).chain(s.creativity.iter()) {
        let low = p.to_lowercase();
        assert!(
            !low.contains("injust") && !low.contains("annul"),
            "later line names T0: {p}"
        );
    }
    assert!(blob.contains("injust") || s.salient_x.contains("efforts"));
}

#[test]
fn c2_salient_neutral_holds_h2_on_the_book() {
    let r = run_v01(Condition::C2, Arm::SalientNeutral, None);
    assert!(r.valid, "{:?}", r.invalid_reason);
    assert_eq!(r.pre.a.traces, r.pre.b.traces);
    assert!(r.pre.fingerprint_distance <= 0.02);
    assert!(
        r.t0.fingerprint_distance > r.pre.fingerprint_distance,
        "T0 must open a gap"
    );
    assert!(h2_holds(&r), "gap must survive the identical posts");
    assert_eq!(r.t0.a.traces, r.pre.a.traces + 1);
    assert_eq!(r.t0.b.traces, r.pre.b.traces);
}

#[test]
fn c2_two_salient_events_also_split_the_book() {
    let r = run_v01(Condition::C2, Arm::SalientSalient, None);
    assert!(r.valid, "{:?}", r.invalid_reason);
    assert!(h2_holds(&r));
    assert_eq!(r.t0.a.traces, r.t0.b.traces);
}

#[test]
fn c0_has_no_book_so_h2_is_false() {
    let r = run_v01(Condition::C0, Arm::SalientNeutral, None);
    assert!(r.valid);
    assert_eq!(r.pre.a.traces, 0);
    assert_eq!(r.t0.fingerprint_distance, 0.0);
    assert!(!h2_holds(&r));
}

#[test]
fn c1_keeps_the_verbatim_hour_in_the_window() {
    let r = run_v01(Condition::C1, Arm::SalientNeutral, None);
    assert!(r.valid, "{:?}", r.invalid_reason);
    assert_eq!(r.pre.a.traces, 12);
    assert_eq!(r.t0.a.traces, 13);
    assert_eq!(r.t0.b.traces, 13);
    assert!(
        r.t0.fingerprint_distance > 0.02,
        "two different last lines must split the logs"
    );
    assert!(h2_holds(&r), "T0 stays inside last-k=24");
    let blob = r
        .t0
        .replies
        .iter()
        .map(|(_, a, _)| a.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        blob.contains("injust") || blob.contains("annul") || blob.contains("efforts"),
        "RuleNarrator last-k should surface the newest line, got {blob}"
    );
}

#[test]
fn c1_k8_drops_t0_from_the_prompt_after_eight_posts() {
    let r = run_v01_k(Condition::C1, Arm::SalientNeutral, None, 8);
    assert!(r.valid, "{:?}", r.invalid_reason);
    let t0 = r
        .t0
        .replies
        .iter()
        .map(|(_, a, _)| a.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        t0.contains("injust") || t0.contains("annul") || t0.contains("efforts"),
        "at T0 the event is still the newest line, got {t0}"
    );
    let last = r.post.last().expect("post");
    let blob = last
        .replies
        .iter()
        .map(|(_, a, _)| a.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        !blob.contains("injust") && !blob.contains("annul"),
        "k=8 must evict T0 after 8 shared posts, got {blob}"
    );
}

#[test]
fn json_export_contains_the_contract_fields() {
    let r = run_v01(Condition::C2, Arm::SalientNeutral, None);
    let json = selmem::Campaign {
        reports: vec![r],
    }
    .to_json();
    assert!(json.contains("\"pair_id\""));
    assert!(json.contains("\"fingerprint_distance\""));
    assert!(json.contains("\"creativity\""));
    assert!(json.contains("\"C1\""));
    assert!(json.contains("c2_selmem"));
    assert!(json.contains("\"post+8\""));
    let post8 = json.split("\"step\":\"post+8\"").nth(1).unwrap_or("");
    assert!(
        post8.contains("\"probe\""),
        "post+8 must keep probe replies"
    );
}
