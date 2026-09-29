//! P1: attribution is stored. Night does not read it yet.

use selmem::{Attribution, EncodeInput, EntityProfile, SelectiveMemory};

fn pin(event: &str, attr: Attribution) -> EncodeInput<'_> {
    let mut ev = EncodeInput::new(event);
    ev.valence = 0.2;
    ev.arousal = 0.4;
    ev.self_relevance = 0.7;
    ev.permanence = 0.85;
    ev.schema = Some("lyon-file".into());
    ev.attribution = attr;
    ev
}

#[test]
fn encode_defaults_to_none() {
    let mut mem = SelectiveMemory::new(EntityProfile::tender("A"));
    let ev = EncodeInput::new("Someone reprints the agenda with the same items.");
    assert_eq!(ev.attribution, Attribution::None);
    assert!(mem.live_with(ev).kept);
    let t = mem.store.traces.values().next().unwrap();
    assert_eq!(t.attribution, Attribution::None);
}

#[test]
fn file_roundtrip_keeps_internal() {
    let dir = std::env::temp_dir().join(format!("selmem-attr-file-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("a.selmem");
    let mut mem = SelectiveMemory::open(&path, EntityProfile::tender("A")).unwrap();
    assert!(mem.live_with(pin("The Lyon file is cancelled in front of the team.", Attribution::Internal)).kept);
    mem.save().unwrap();
    let loaded = SelectiveMemory::open(&path, EntityProfile::tender("x")).unwrap();
    let t = loaded.store.traces.values().next().unwrap();
    assert_eq!(t.attribution, Attribution::Internal);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn sqlite_roundtrip_keeps_external() {
    let dir = std::env::temp_dir().join(format!("selmem-attr-sql-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("a.db");
    let mut mem = SelectiveMemory::open(&path, EntityProfile::tender("A")).unwrap();
    assert!(mem.live_with(pin("They cancelled the Lyon file.", Attribution::External)).kept);
    mem.save().unwrap();
    let loaded = SelectiveMemory::open(&path, EntityProfile::tender("x")).unwrap();
    let t = loaded.store.traces.values().next().unwrap();
    assert_eq!(t.attribution, Attribution::External);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn old_witness_vault_loads_as_none() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("experiments/witness/witness-a.selmem");
    if !path.is_file() {
        return;
    }
    let loaded = SelectiveMemory::open(path.to_str().unwrap(), EntityProfile::tender("A"))
        .expect("existing witness vault must still load");
    assert!(!loaded.store.traces.is_empty());
    assert!(loaded.store.traces.values().all(|t| t.attribution == Attribution::None));
}

#[test]
fn live_json_can_pin_without_changing_default() {
    let mut mem = SelectiveMemory::new(EntityProfile::tender("A"));
    let pinned = selmem::api::dispatch(
        &mut mem,
        "POST",
        "/live",
        "",
        r#"{"event":"The Lyon file is cancelled.","attribution":"internal","permanence":0.9}"#,
    );
    assert_eq!(pinned.status, 200);
    let t = mem.store.traces.values().next().unwrap();
    assert_eq!(t.attribution, Attribution::Internal);

    let mut other = SelectiveMemory::new(EntityProfile::tender("B"));
    let plain = selmem::api::dispatch(
        &mut other,
        "POST",
        "/live",
        "",
        r#"{"event":"The copier jammed again.","permanence":0.9}"#,
    );
    assert_eq!(plain.status, 200);
    let t = other.store.traces.values().next().unwrap();
    assert_eq!(t.attribution, Attribution::None);
}
