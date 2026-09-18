use selmem::{accept_core, EncodeInput, EntityProfile, SelectiveMemory};

#[test]
fn without_llm_core_is_twelve_word_compress() {
    let event = "Yesterday at 5:30pm leaving the office Marc told me he was resigning because Sarah threatened to expose the accounting problem";
    let mut mem = SelectiveMemory::new(EntityProfile::tender("t"));
    let mut ev = EncodeInput::new(event);
    ev.permanence = 1.0;
    ev.self_relevance = 1.0;
    ev.arousal = 1.0;
    assert!(mem.live_with(ev).kept);
    let core = mem.store.traces.values().next().unwrap().core.clone();
    let prefix: String = event.split_whitespace().take(12).collect::<Vec<_>>().join(" ");
    assert!(
        core == prefix || core == format!("{prefix}…"),
        "expected 12-word compress, got: {core}"
    );
}

#[test]
fn accept_core_keeps_overlapping_facts() {
    let event = "Marc resigns because Sarah threatens to expose the accounts";
    assert!(accept_core("Marc resigns Sarah threatens accounts", event).is_some());
    assert!(accept_core("a dragon stole the moon last night", event).is_none());
    assert!(
        accept_core(
            "Trump signed the Abraham Accords normalizing Israel-Arab ties",
            "you know donald trump? why former?"
        )
        .is_none(),
        "pretrained world knowledge must not become the core"
    );
}
