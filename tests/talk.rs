//! Working talk is a session frame, not a trace.

use selmem::{EncodeInput, EntityProfile, SelectiveMemory, WorkingTalk};

fn seed_injustice() -> SelectiveMemory {
    let mut mem = SelectiveMemory::new(EntityProfile::tender("Claire"));
    let mut ev = EncodeInput::new(
        "Le projet a été annulé sans raison, on m'a volé le crédit.",
    );
    ev.valence = -0.82;
    ev.arousal = 0.78;
    ev.disgust = 0.55;
    ev.self_relevance = 0.95;
    ev.permanence = 0.92;
    ev.schema = Some("injustice".into());
    assert!(mem.live_with(ev).kept);
    mem
}

#[test]
fn light_followup_keeps_the_topic() {
    let mut talk = WorkingTalk::default();
    talk.hear(
        "Le projet a été annulé sans raison",
        Some("injustice"),
    );
    let before = talk.topic.clone();
    talk.hear("Et alors, tu en penses quoi ?", None);
    assert_eq!(talk.topic, before);
    assert_eq!(talk.schema.as_deref(), Some("injustice"));
}

#[test]
fn recall_query_mixes_topic_into_a_bare_line() {
    let mut talk = WorkingTalk::default();
    talk.hear("projet annulé crédit volé", Some("injustice"));
    let q = talk.recall_query("Et alors ?");
    assert!(q.contains("Et alors"));
    assert!(
        q.contains("injustice") || q.contains("projet") || q.contains("annul"),
        "cue should carry the thread, got {q:?}"
    );
}

#[test]
fn new_content_can_replace_the_topic() {
    let mut talk = WorkingTalk::default();
    talk.hear("projet annulé injustement", Some("injustice"));
    talk.hear(
        "Ma couleur préférée depuis l'enfance reste le bleu nuit",
        None,
    );
    let topic = talk.topic.as_deref().unwrap_or("");
    assert!(
        topic.contains("bleu") || topic.contains("couleur") || topic.contains("enfance"),
        "content shift should retitle the thread, got {topic:?}"
    );
}

#[test]
fn frame_caps_at_six_turns() {
    let mut talk = WorkingTalk::default();
    for i in 0..9 {
        talk.record(&format!("user {i} projet"), &format!("reply {i}"));
    }
    assert_eq!(talk.turns.len(), 6);
    assert!(talk.turns[0].user.contains("user 3"));
    assert!(talk.turns[5].user.contains("user 8"));
}

#[test]
fn render_exposes_fil_not_archive() {
    let mut talk = WorkingTalk::default();
    talk.hear("projet annulé", Some("injustice"));
    talk.record("on en parlait", "je m'en souviens");
    let r = talk.render();
    assert!(r.contains("fil:"));
    assert!(r.contains("conversation en cours"));
    assert!(r.contains("on en parlait"));
    assert!(!r.contains("archive"));
}

#[test]
fn speak_followup_still_recalls_the_marked_hour() {
    let mut mem = seed_injustice();
    let _ = mem.speak("On parlait du projet annulé.");
    let hits = mem.remember(&mem.talk.recall_query("Et alors, tu en penses quoi ?"));
    assert!(
        !hits.is_empty(),
        "topic-mixed cue must find the lived hour"
    );
    let blob = hits
        .iter()
        .map(|h| h.narrative.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        blob.contains("projet")
            || blob.contains("annul")
            || blob.contains("crédit")
            || blob.contains("credit")
            || blob.contains("injust"),
        "recalled narrative should still be the marked hour, got {blob:?}"
    );
}

#[test]
fn sleep_does_not_clear_the_thread() {
    let mut mem = seed_injustice();
    let _ = mem.speak("On parlait du projet annulé.");
    assert!(!mem.talk.is_empty());
    let _ = mem.sleep();
    assert!(
        mem.talk.topic.is_some() && !mem.talk.turns.is_empty(),
        "night weathers the book, not the live thread"
    );
}

#[test]
fn persist_does_not_write_the_thread() {
    let dir = std::env::temp_dir().join(format!("selmem-talk-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("claire.selmem");
    let mut mem = SelectiveMemory::open(&path, EntityProfile::tender("Claire")).unwrap();
    let mut ev = EncodeInput::new("Le projet a été annulé sans raison.");
    ev.valence = -0.7;
    ev.arousal = 0.6;
    ev.self_relevance = 0.9;
    ev.schema = Some("injustice".into());
    assert!(mem.live_with(ev).kept);
    let _ = mem.speak("On parlait du projet.");
    assert!(!mem.talk.is_empty());
    mem.save().unwrap();

    let loaded = SelectiveMemory::open(&path, EntityProfile::tender("x")).unwrap();
    assert!(
        loaded.talk.is_empty(),
        "reloaded vault must not resurrect the session frame"
    );
    assert!(!loaded.store.traces.is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn http_turn_speak_and_clear() {
    let mut mem = seed_injustice();
    let turn = selmem::api::dispatch(
        &mut mem,
        "POST",
        "/turn",
        "",
        r#"{"text":"On parlait du projet annulé."}"#,
    );
    assert_eq!(turn.status, 200);
    assert!(turn.body.contains("\"topic\""));

    let listed = selmem::api::dispatch(&mut mem, "GET", "/talk", "", "");
    assert_eq!(listed.status, 200);
    assert!(listed.body.contains("projet") || listed.body.contains("injustice"));

    let follow = selmem::api::dispatch(
        &mut mem,
        "POST",
        "/speak",
        "",
        r#"{"text":"Et alors ?"}"#,
    );
    assert_eq!(follow.status, 200);
    assert!(mem.talk.turns.len() >= 2);

    let cleared = selmem::api::dispatch(&mut mem, "POST", "/talk/clear", "", "{}");
    assert_eq!(cleared.status, 200);
    assert!(mem.talk.is_empty());
}

#[test]
fn isolated_probe_does_not_touch_the_frame() {
    let mut mem = seed_injustice();
    let _ = mem.speak("On parlait du projet annulé.");
    let n = mem.talk.turns.len();
    let topic = mem.talk.topic.clone();
    let _ = mem.speak_isolated("Quelle est ta couleur préférée ?");
    assert_eq!(mem.talk.turns.len(), n);
    assert_eq!(mem.talk.topic, topic);
}
