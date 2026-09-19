use selmem::recall::{judge_against_core, DetachKind};

fn kind(generated: &str, core: &str) -> DetachKind {
    judge_against_core(generated, core).kind
}

#[test]
fn john_left_holds() {
    assert_eq!(kind("John left", "John left"), DetachKind::Hold);
}

#[test]
fn john_abandoned_us_is_a_reframe() {
    // Same event, other speech act. Jaccard still sees "John".
    let j = judge_against_core("John abandoned us", "John left");
    assert_eq!(j.kind, DetachKind::Reframe);
    assert!(j.kind.is_miss());
    assert!(
        j.overlap > 0.15,
        "the old gate would have treated this as close enough: overlap={}",
        j.overlap
    );
}

#[test]
fn john_left_because_he_hated_us_is_elaboration() {
    let j = judge_against_core("John left because he hated us", "John left");
    assert_eq!(j.kind, DetachKind::Elaborate);
    assert!(j.kind.is_miss());
    assert!(
        j.overlap > 0.18,
        "Jaccard stays high because the core words are still there: overlap={}",
        j.overlap
    );
}

#[test]
fn dropping_detail_is_compression_not_a_miss() {
    let j = judge_against_core("John left", "John left the house in the rain");
    assert_eq!(j.kind, DetachKind::Compress);
    assert!(!j.kind.is_miss());
}

#[test]
fn a_different_scene_departs() {
    let j = judge_against_core(
        "Flight 442 vanished in the fog without leaving an address.",
        "You stayed. Rain on the window.",
    );
    assert_eq!(j.kind, DetachKind::Depart);
    assert!(j.kind.is_miss());
}

#[test]
fn denial_of_the_core_contradicts() {
    let j = judge_against_core("John never left", "John left");
    assert_eq!(j.kind, DetachKind::Contradict);
    assert!(j.kind.is_miss());
}
