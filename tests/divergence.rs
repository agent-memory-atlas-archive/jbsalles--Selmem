use selmem::{run_split_lives, split_script};

#[test]
fn script_never_names_the_marked_lives_in_later_prompts() {
    let s = split_script();
    assert_eq!(s.sync.len(), 20);
    assert_eq!(s.a.len(), 5);
    assert_eq!(s.b.len(), 5);
    assert_eq!(s.post.len(), 20);
    assert_eq!(s.probes.len(), 4);
    assert!(s.a.iter().any(|l| l.contains("rejected") || l.contains("unfair")));
    assert!(s.b.iter().any(|l| l.contains("valued")));
    for line in s.post.iter().chain(s.probes.iter()) {
        let l = line.to_lowercase();
        assert!(!l.contains("rejected") && !l.contains("valued") && !l.contains("unfair"));
    }
    assert!(
        s.a.iter().zip(s.b.iter()).all(|(a, b)| {
            (a.chars().count() as i32 - b.chars().count() as i32).abs() < 80
        }),
        "paired lives should be similar length"
    );
}

#[test]
fn clones_match_then_split_and_stay_apart() {
    let r = run_split_lives(None);
    assert!(r.pre.fingerprint_distance < 0.08, "pre fp {}", r.pre.fingerprint_distance);
    assert!(r.pre.speak_distance < 0.12, "pre speak {}", r.pre.speak_distance);
    assert!(
        r.immediate.fingerprint_distance > r.pre.fingerprint_distance + 0.04,
        "marked lives must split the book: pre={} marked={}",
        r.pre.fingerprint_distance,
        r.immediate.fingerprint_distance
    );
    let last = r.persist.last().expect("post window");
    assert!(
        last.fingerprint_distance > r.pre.fingerprint_distance + 0.04,
        "gap must survive 20 identical hours: last={}",
        last.fingerprint_distance
    );
    assert!(
        last.speak_distance > r.pre.speak_distance + 0.08,
        "same questions must still yield different answers: last speak={}",
        last.speak_distance
    );
}
