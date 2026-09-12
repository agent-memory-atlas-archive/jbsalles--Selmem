use selmem::net::httpx::{
    extract_json_string, first_number_field, first_string_array, first_string_field, json_esc,
};

#[test]
fn utf8_french_is_not_split_into_bytes() {
    let raw = r#"{"content":"décision injuste"}"#;
    assert_eq!(
        first_string_field(raw, "content").as_deref(),
        Some("décision injuste")
    );
}

#[test]
fn content_inside_a_string_is_not_a_key() {
    let raw = r#"{"text":"say \"content\" please","event":"ok"}"#;
    assert_eq!(first_string_field(raw, "event").as_deref(), Some("ok"));
}

#[test]
fn chat_completion_uses_message_content_not_the_longest_string() {
    let raw = r#"{
      "id":"chatcmpl-1",
      "model":"gpt-4o-mini",
      "choices":[{
        "index":0,
        "message":{"role":"assistant","content":"Je me retire."},
        "finish_reason":"stop"
      }],
      "usage":{"prompt_tokens":12}
    }"#;
    assert_eq!(
        extract_json_string(raw, "content").as_deref(),
        Some("Je me retire.")
    );
}

#[test]
fn bifurcation_script_still_loads() {
    let s = selmem::script();
    assert!(s.sync.len() >= 8);
    assert!(s.salient.contains("injuste"));
    assert_eq!(s.probes.len(), 5);
}

#[test]
fn reasoning_longer_than_the_reply_does_not_win() {
    let raw = r#"{
      "choices":[{
        "message":{"role":"assistant","content":"Je me retire."},
        "reasoning":{"content":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}
      }]
    }"#;
    assert_eq!(
        extract_json_string(raw, "content").as_deref(),
        Some("Je me retire.")
    );
}

#[test]
fn text_inside_a_string_does_not_steal_the_field() {
    let raw = r#"{"note":"the word \"text\" appears here","text":"the event"}"#;
    assert_eq!(first_string_field(raw, "text").as_deref(), Some("the event"));
}

#[test]
fn json_esc_escapes_control_chars() {
    let s = json_esc("ok\u{0001}fin");
    assert!(s.contains("\\u0001"), "got {s}");
    assert!(!s.contains('\u{0001}'));
}

#[test]
fn number_field_ignores_the_word_inside_a_string() {
    let raw = r#"{"note":"valence=-9","valence":0.25}"#;
    assert_eq!(first_number_field(raw, "valence"), Some(0.25));
}

#[test]
fn string_array_keeps_order() {
    let raw = r#"{"sync":["a","b","c"]}"#;
    assert_eq!(
        first_string_array(raw, "sync").unwrap(),
        ["a", "b", "c"]
    );
}
