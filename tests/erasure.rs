use selmem::run_erasure;

#[test]
fn trivia_fades_repeated_aversion_does_not() {
    let r = run_erasure(None);
    assert!(r.color_kept_at_encode, "the color must enter before it can be forgotten");
    assert!(r.aversion_kept_at_encode, "the aversion must enter");
    assert!(
        !r.color_recalled,
        "one-shot colour should drop out of recall: {}",
        r.color_answer
    );
    assert!(
        r.aversion_recalled,
        "repeated aversion should still answer: {}",
        r.aversion_answer
    );
    let color_l = r.color_answer.to_lowercase();
    assert!(
        !color_l.contains("bleu") && !color_l.contains("blue"),
        "spoken answer still names the colour: {}",
        r.color_answer
    );
}
