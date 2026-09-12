use selmem::{h2_holds, run_v01, v01_script, Arm, Condition};

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
}
