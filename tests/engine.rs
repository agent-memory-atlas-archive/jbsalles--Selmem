//! Physiology. Narrative cases live in tests/cases/*.json (runner: tests/scenes.rs).

use selmem::{AxiomLayer, Embedder, EntityProfile, IdentityAxiom, SelectiveMemory};
use selmem::EncodeInput;

#[test]
fn persist_roundtrip_keeps_lived_memory_and_archive() {
    let dir = std::env::temp_dir().join(format!("selmem-test-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("claire.selmem");

    let mut mem = SelectiveMemory::open(&path, EntityProfile::tender("Claire")).unwrap();
    let mut ev = EncodeInput::new("Tu es resté sous la pluie.");
    ev.valence = 0.7;
    ev.arousal = 0.5;
    ev.self_relevance = 0.9;
    ev.schema = Some("fidélité".into());
    let d = mem.live_with(ev);
    assert!(d.kept);
    mem.sleep();
    mem.save().unwrap();

    let mut loaded = SelectiveMemory::open(&path, EntityProfile::tender("autre")).unwrap();
    assert_eq!(loaded.profile.name, "Claire");
    assert_eq!(loaded.store.traces.len(), 1);
    let tid = d.trace_id.unwrap();
    assert!(loaded.store.traces[&tid].gist.contains("pluie"));
    assert_eq!(
        loaded.audit(&tid).unwrap(),
        "Tu es resté sous la pluie."
    );
    let recalled = loaded.remember("pluie");
    assert!(!recalled.is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn http_api_live_remember_sleep() {
    let mut mem = SelectiveMemory::new(EntityProfile::tender("Claire"));
    let live = selmem::api::dispatch(
        &mut mem,
        "POST",
        "/live",
        "",
        r#"{"event":"Tu es resté sous la pluie.","valence":0.7,"arousal":0.5,"self_relevance":0.9,"schema":"fidélité"}"#,
    );
    assert_eq!(live.status, 200);
    assert!(live.body.contains("\"kept\":true"));

    let sleep = selmem::api::dispatch(&mut mem, "POST", "/sleep", "", "{}");
    assert_eq!(sleep.status, 200);

    let rec = selmem::api::dispatch(&mut mem, "POST", "/remember", "", r#"{"query":"la pluie"}"#);
    assert_eq!(rec.status, 200);
    assert!(rec.body.contains("pluie"));

    let who = selmem::api::dispatch(&mut mem, "GET", "/who", "", "");
    assert_eq!(who.status, 200);
    assert!(who.body.contains("Claire"));

    let turn = selmem::api::dispatch(
        &mut mem,
        "POST",
        "/turn",
        "",
        r#"{"text":"Tu es resté encore un peu."}"#,
    );
    assert_eq!(turn.status, 200);
    assert!(turn.body.contains("\"reply\""));
}

#[test]
fn sleep_merges_close_episodes_and_can_extinguish() {
    let mut profile = EntityProfile::tender("Claire");
    profile.encode_threshold = 0.12;
    profile.merge_similarity = 0.15;
    let mut mem = SelectiveMemory::new(profile);
    for text in [
        "Tu es resté sous la pluie près de la fenêtre.",
        "Encore cette pluie : tu es resté près de la fenêtre, sans partir.",
    ] {
        let mut ev = EncodeInput::new(text);
        ev.valence = 0.35;
        ev.arousal = 0.35;
        ev.self_relevance = 0.7;
        ev.schema = Some("fidélité".into());
        assert!(mem.live_with(ev).kept);
    }
    let mut bitter = EncodeInput::new("Un écart qui a laissé un goût.");
    bitter.valence = -0.5;
    bitter.arousal = 0.4;
    bitter.disgust = 0.4;
    bitter.self_relevance = 0.7;
    bitter.permanence = 0.85;
    bitter.schema = Some("retrait".into());
    let id = mem.live_with(bitter).trace_id.unwrap();
    let before = mem.store.traces[&id].disgust;
    let report = mem.sleep();
    assert!(report.merged >= 1);
    let after = mem.store.traces[&id].disgust;
    assert!(after <= before);
}

#[test]
fn embeddings_rank_paraphrase_above_unrelated() {
    let e = selmem::HashEmbedder;
    let rain = e.embed("tu es resté sous la pluie près de la fenêtre");
    let para = e.embed("encore cette pluie, tu n'es pas parti de la fenêtre");
    let noise = e.embed("le vol 442 part à dix-huit heures quarante");
    assert!(selmem::cosine(&rain, &para) > selmem::cosine(&rain, &noise));
}

#[test]
fn sqlite_roundtrip() {
    let dir = std::env::temp_dir().join(format!("selmem-sql-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("claire.db");
    let mut mem = SelectiveMemory::open(&path, EntityProfile::tender("Claire")).unwrap();
    let mut ev = EncodeInput::new("Tu es resté sous la pluie.");
    ev.valence = 0.7;
    ev.self_relevance = 0.9;
    ev.schema = Some("fidélité".into());
    assert!(mem.live_with(ev).kept);
    mem.save().unwrap();
    let loaded = SelectiveMemory::open(&path, EntityProfile::tender("x")).unwrap();
    assert_eq!(loaded.profile.name, "Claire");
    assert_eq!(loaded.store.traces.len(), 1);
    let t = loaded.store.traces.values().next().unwrap();
    assert!(!t.embedding.is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn axiom_is_superseded_when_belief_changes() {
    let mut mem = SelectiveMemory::new(EntityProfile::tender("Claire"));
    mem.store.add_axiom(IdentityAxiom {
        id: "ax_old".into(),
        statement: "Je doutais encore de la fidélité.".into(),
        support_trace_ids: vec![],
        valence: 0.0,
        strength: 0.3,
        created_at: 1,
        superseded_by: None,
        schema: Some("fidélité".into()),
        layer: AxiomLayer::Belief,
    });
    for text in ["Tu es resté sous la pluie.", "Tu es resté près de moi."] {
        let mut ev = EncodeInput::new(text);
        ev.valence = 0.7;
        ev.arousal = 0.4;
        ev.self_relevance = 0.8;
        ev.permanence = 0.85;
        ev.schema = Some("fidélité".into());
        mem.live_with(ev);
    }
    mem.sleep();
    let living = mem.who_am_i();
    assert!(living.iter().any(|a| a.id != "ax_old"));
    assert_eq!(
        mem.store.axioms["ax_old"].superseded_by.is_some(),
        true
    );
    assert!(mem.lineage("fidélité").len() >= 2);
}

#[test]
fn ebbinghaus_drops_detail_keeps_core() {
    let mut mem = SelectiveMemory::new(EntityProfile::tender("Claire"));
    let mut ev = EncodeInput::new("Tu es resté sous la pluie près de la fenêtre, sans manteau.");
    ev.valence = 0.2;
    ev.arousal = 0.2;
    ev.self_relevance = 0.5;
    ev.schema = Some("soir".into());
    let id = mem.live_with(ev).trace_id.unwrap();
    {
        let t = mem.store.traces.get_mut(&id).unwrap();
        t.created_at = t.created_at.saturating_sub(40 * 86_400);
        t.anchor = 0.0;
    }
    let core_before = mem.store.traces[&id].core.clone();
    let fid_before = mem.store.traces[&id].fidelity;
    mem.sleep();
    let t = &mem.store.traces[&id];
    assert_eq!(t.core, core_before);
    assert!(t.fidelity < fid_before);
}

#[test]
fn trauma_is_anchored_and_resists_weather() {
    let mut mem = SelectiveMemory::new(EntityProfile::austere("Silas"));
    let mut ev = EncodeInput::new("Tu as ri de ce que je t'avais dit en confiance.");
    ev.valence = -0.7;
    ev.arousal = 0.8;
    ev.disgust = 0.7;
    ev.self_relevance = 0.95;
    ev.schema = Some("humiliation".into());
    let id = mem.live_with(ev).trace_id.unwrap();
    assert!(mem.store.traces[&id].anchor >= 0.55);
    {
        let t = mem.store.traces.get_mut(&id).unwrap();
        t.created_at = t.created_at.saturating_sub(40 * 86_400);
    }
    let fid_before = mem.store.traces[&id].fidelity;
    mem.sleep();
    let t = &mem.store.traces[&id];
    assert!(t.fidelity + 0.02 >= fid_before * 0.55);
    assert!(!t.core.is_empty());
}

#[test]
fn identity_colors_a_related_event() {
    let mut mem = SelectiveMemory::new(EntityProfile::tender("Claire"));
    mem.store.add_axiom(IdentityAxiom {
        id: "ax_bias".into(),
        statement: "Je me retire de ce qui ressemble à de l'abandon.".into(),
        support_trace_ids: vec![],
        valence: -0.7,
        strength: 0.8,
        created_at: 1,
        superseded_by: None,
        schema: Some("abandon".into()),
        layer: AxiomLayer::Belief,
    });
    let mut ev = EncodeInput::new("Tu es parti sans prévenir, encore une fois.");
    ev.valence = -0.2;
    ev.arousal = 0.4;
    ev.self_relevance = 0.5;
    ev.schema = Some("abandon".into());
    let before = ev.valence;
    let d = mem.live_with(ev);
    assert!(d.kept);
    let t = mem.store.traces.values().next().unwrap();
    assert!(t.valence <= before);
}

#[test]
fn recall_can_reinterpret_meaning() {
    let mut mem = SelectiveMemory::new(EntityProfile::tender("Claire"));
    let mut ev = EncodeInput::new("Tu es resté sous la pluie.");
    ev.valence = 0.4;
    ev.arousal = 0.5;
    ev.self_relevance = 0.8;
    ev.schema = Some("fidélité".into());
    mem.live_with(ev);
    mem.mood.valence = -0.8;
    let _ = mem.remember("cette pluie");
    let t = mem.store.traces.values().next().unwrap();
    assert!(
        t.drifts.iter().any(|d| d.kind == selmem::DriftKind::Reinterpret) || t.valence < 0.4,
        "valence {} drifts {:?}",
        t.valence,
        t.drifts
    );
}

#[test]
fn detached_recall_corrects_after_several_misses() {
    let mut profile = EntityProfile::tender("Claire");
    profile.ground_strikes = 3;
    profile.narrator_firmness = 1.0;
    let mut mem = SelectiveMemory::new(profile);
    let mut ev = EncodeInput::new("Tu es resté. La pluie sur la fenêtre.");
    ev.valence = 0.7;
    ev.arousal = 0.5;
    ev.self_relevance = 0.9;
    ev.permanence = 0.9;
    ev.schema = Some("fidélité".into());
    let id = mem.live_with(ev).trace_id.unwrap();
    {
        let t = mem.store.traces.get_mut(&id).unwrap();
        t.gist = "Le vol 442 a disparu dans le brouillard sans laisser d'adresse.".into();
        t.cues.push("pluie".into());
    }
    let first = mem.remember("la pluie");
    assert!(!first.is_empty());
    assert!(
        first[0].narrative.contains("442") || first[0].narrative.contains("brouillard"),
        "first miss should still speak the drifted gist, got {}",
        first[0].narrative
    );
    assert!(!first[0].disclaimer.contains("journal"));
    assert_eq!(mem.store.traces[&id].detach_strikes, 1);
    let _ = mem.remember("la pluie");
    assert_eq!(mem.store.traces[&id].detach_strikes, 2);
    let third = mem.remember("la pluie");
    assert!(
        third[0].narrative.contains("pluie")
            || third[0].narrative.contains("resté")
            || third[0].narrative.contains("fenêtre"),
        "after threshold, gist must return toward core, got {}",
        third[0].narrative
    );
    assert!(!third[0].narrative.contains("442"));
    let t = &mem.store.traces[&id];
    assert!(t.drifts.iter().any(|d| d.kind == selmem::DriftKind::Ground));
    assert_eq!(t.detach_strikes, 0);
}

#[test]
fn fading_trace_may_distort_without_grounding() {
    let mut profile = EntityProfile::tender("Claire");
    profile.narrator_firmness = 1.0;
    profile.ground_strikes = 1;
    let mut mem = SelectiveMemory::new(profile);
    let mut ev = EncodeInput::new("Une pluie quelconque.");
    ev.valence = 0.2;
    ev.self_relevance = 0.9;
    ev.permanence = 0.85;
    let id = mem.live_with(ev).trace_id.expect("kept so we can fade it after");
    {
        let t = mem.store.traces.get_mut(&id).unwrap();
        t.status = selmem::TraceStatus::Cold;
        t.fidelity = 0.25;
        t.access = 0.1;
        t.permanence = 0.1;
        t.anchor = 0.05;
        t.gist = "Le vol 442 a disparu dans le brouillard.".into();
        t.cues.push("pluie".into());
    }
    for _ in 0..5 {
        let _ = mem.remember("la pluie");
    }
    let t = &mem.store.traces[&id];
    assert!(
        t.gist.contains("442"),
        "fading gist should keep warping, got {}",
        t.gist
    );
    assert!(!t.drifts.iter().any(|d| d.kind == selmem::DriftKind::Ground));
}

fn plant_important_drift(mem: &mut SelectiveMemory) -> String {
    let mut ev = EncodeInput::new("Tu es resté. La pluie sur la fenêtre.");
    ev.valence = 0.7;
    ev.arousal = 0.5;
    ev.self_relevance = 0.9;
    ev.permanence = 0.9;
    ev.schema = Some("fidélité".into());
    let id = mem.live_with(ev).trace_id.expect("kept");
    {
        let t = mem.store.traces.get_mut(&id).unwrap();
        t.gist = "Le vol 442 a disparu dans le brouillard sans laisser d'adresse.".into();
        t.cues.push("pluie".into());
    }
    id
}

#[test]
fn zero_firmness_never_grounds_an_important_trace() {
    let mut loose = EntityProfile::tender("Claire");
    loose.narrator_firmness = 0.0;
    loose.ground_strikes = 1;
    let mut mem = SelectiveMemory::new(loose);
    let id = plant_important_drift(&mut mem);
    for _ in 0..8 {
        let _ = mem.remember("la pluie");
    }
    let t = &mem.store.traces[&id];
    assert!(
        t.gist.contains("442"),
        "firmness 0 must leave the warped gist, got {}",
        t.gist
    );
    assert!(!t.drifts.iter().any(|d| d.kind == selmem::DriftKind::Ground));
}

#[test]
fn firm_narrator_grounds_sooner_than_a_soft_one() {
    let mut hard_p = EntityProfile::austere("Silas");
    hard_p.narrator_firmness = 1.0;
    hard_p.ground_strikes = 2;
    let mut soft_p = EntityProfile::tender("Claire");
    soft_p.narrator_firmness = 0.25;
    soft_p.ground_strikes = 2;

    let mut hard = SelectiveMemory::new(hard_p);
    let mut soft = SelectiveMemory::new(soft_p);
    let hid = plant_important_drift(&mut hard);
    let sid = plant_important_drift(&mut soft);

    for _ in 0..3 {
        let _ = hard.remember("la pluie");
        let _ = soft.remember("la pluie");
    }
    let hg = hard.store.traces[&hid]
        .drifts
        .iter()
        .any(|d| d.kind == selmem::DriftKind::Ground);
    let sg = soft.store.traces[&sid]
        .drifts
        .iter()
        .any(|d| d.kind == selmem::DriftKind::Ground);
    assert!(hg, "firm narrator should have pulled toward the core");
    assert!(!sg, "softer narrator should still be allowed the warped gist");
}

#[test]
fn grounding_never_exposes_the_archive() {
    let mut profile = EntityProfile::austere("Silas");
    profile.narrator_firmness = 1.0;
    profile.ground_strikes = 1;
    let mut mem = SelectiveMemory::new(profile);
    let mut ev = EncodeInput::new("Tu es resté. La pluie sur la fenêtre.");
    ev.valence = 0.7;
    ev.arousal = 0.5;
    ev.self_relevance = 0.9;
    ev.permanence = 0.9;
    ev.schema = Some("fidélité".into());
    let id = mem.live_with(ev).trace_id.expect("kept");
    let secret = "VERBATIM-SEALED-991";
    {
        let aid = mem.store.traces[&id].archive_id.clone().unwrap();
        mem.store.archives.get_mut(&aid).unwrap().verbatim =
            format!("Tu es resté. La pluie. {secret}");
        let t = mem.store.traces.get_mut(&id).unwrap();
        t.gist = "Le vol 442 a disparu dans le brouillard sans laisser d'adresse.".into();
        t.cues.push("pluie".into());
    }
    let mut saw_ground = false;
    for _ in 0..6 {
        let rec = mem.remember("la pluie");
        for r in &rec {
            assert!(
                !r.narrative.contains(secret),
                "narrative leaked the journal: {}",
                r.narrative
            );
            assert!(!r.disclaimer.contains(secret));
        }
        if mem.store.traces[&id]
            .drifts
            .iter()
            .any(|d| d.kind == selmem::DriftKind::Ground)
        {
            saw_ground = true;
            break;
        }
    }
    assert!(saw_ground, "firm living trace should ground");
    let t = &mem.store.traces[&id];
    assert!(!t.gist.contains(secret), "gist must not become the journal");
    assert!(mem.audit(&id).unwrap().contains(secret));
}

#[test]
fn latent_forgets_the_scene_keeps_the_reaction() {
    let mut mem = SelectiveMemory::new(EntityProfile::tender("Claire"));
    let mut ev = EncodeInput::new("Tu as ri de ce que je t'avais dit sous la pluie.");
    ev.valence = -0.7;
    ev.arousal = 0.6;
    ev.disgust = 0.5;
    ev.self_relevance = 0.9;
    ev.permanence = 0.9;
    ev.schema = Some("humiliation".into());
    let id = mem.live_with(ev).trace_id.expect("kept");
    {
        let t = mem.store.traces.get_mut(&id).unwrap();
        t.status = selmem::TraceStatus::Latent;
        t.fidelity = 0.2;
        t.access = 0.1;
    }
    let rec = mem.remember("cette humiliation sous la pluie");
    for r in &rec {
        assert!(
            !r.narrative.contains("ri") && !r.narrative.contains("dit"),
            "latent recall must not replay the scene: {}",
            r.narrative
        );
    }
    let mut next = EncodeInput::new("Encore une humiliation sous la pluie.");
    next.valence = 0.0;
    next.disgust = 0.0;
    next.self_relevance = 0.4;
    mem.live_with(next);
    let painted = mem
        .store
        .traces
        .values()
        .filter(|t| t.id != id)
        .max_by(|a, b| a.created_at.cmp(&b.created_at));
    if let Some(t) = painted {
        assert!(
            t.valence < 0.0 || t.disgust > 0.05 || t.schema.as_deref() == Some("humiliation"),
            "latent charge should color the new event v={} d={} schema={:?}",
            t.valence,
            t.disgust,
            t.schema
        );
    }
}

