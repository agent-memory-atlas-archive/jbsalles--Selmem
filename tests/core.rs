use selmem::{accept_core, EncodeInput, EntityProfile, SelectiveMemory};

#[test]
fn without_llm_core_is_twelve_word_compress() {
    let event = "Hier à 17h30 en sortant du bureau Marc m'a dit qu'il démissionnait parce que Sarah menaçait d'exposer le problème de comptabilité";
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
    let event = "Marc démissionne parce que Sarah menace d'exposer la comptabilité";
    assert!(accept_core("Marc démissionne Sarah menace comptabilité", event).is_some());
    assert!(accept_core("un dragon a volé la lune hier soir", event).is_none());
}
