use selmem::recall::{apply_grounding, is_grounding_miss, judge_against_core, DetachKind};
use selmem::{DriftKind, EncodeInput, EntityProfile, SelectiveMemory};

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
    assert!(!j.kind.is_miss());
    assert!(j.kind.is_color());
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
fn reframe_is_not_a_grounding_miss() {
    assert!(!is_grounding_miss(
        "John abandoned us",
        "John left",
        0.18
    ));
}

#[test]
fn reframe_colors_the_mouth_and_leaves_the_book() {
    let mut profile = EntityProfile::austere("Silas");
    profile.narrator_firmness = 1.0;
    profile.ground_strikes = 1;
    let mut mem = SelectiveMemory::new(profile.clone());
    let mut ev = EncodeInput::new("John left");
    ev.self_relevance = 0.9;
    ev.permanence = 0.9;
    ev.arousal = 0.5;
    ev.schema = Some("loyalty".into());
    let id = mem.live_with(ev).trace_id.expect("kept");
    let gist_before = mem.store.traces[&id].gist.clone();
    let core = mem.store.traces[&id].core.clone();
    let out = {
        let t = mem.store.traces.get_mut(&id).unwrap();
        apply_grounding(t, &profile, "John abandoned us", &core, None)
    };
    assert_eq!(out.spoken_text, "John abandoned us");
    assert!(!out.pulled_toward_core);
    let t = &mem.store.traces[&id];
    assert_eq!(t.gist, gist_before);
    assert_eq!(t.detach_strikes, 0);
    assert!(t.drifts.iter().any(|d| d.kind == DriftKind::Color));
    assert!(!t.drifts.iter().any(|d| d.kind == DriftKind::Ground));
}

#[test]
fn denial_of_the_core_contradicts() {
    let j = judge_against_core("John never left", "John left");
    assert_eq!(j.kind, DetachKind::Contradict);
    assert!(j.kind.is_miss());
}
