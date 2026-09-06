use selmem::{EncodeInput, EntityProfile, SelectiveMemory};

fn main() {
    let path = "/tmp/selmem-claire.selmem";
    let mut mem = SelectiveMemory::open(path, EntityProfile::tender("Claire")).expect("open");

    let mut ev = EncodeInput::new("Tu es resté. La pluie sur la fenêtre.");
    ev.valence = 0.7;
    ev.arousal = 0.5;
    ev.self_relevance = 0.9;
    ev.schema = Some("fidélité".into());
    let d = mem.live_with(ev);
    println!("encodé={} score={:.2} traces={}", d.kept, d.score, mem.store.traces.len());

    mem.sleep();
    mem.save().expect("save");
    println!("écrit {path}");

    let mut again = SelectiveMemory::open(path, EntityProfile::tender("Claire")).expect("reload");
    println!("rechargé traces={} nom={}", again.store.traces.len(), again.profile.name);
    for rec in again.remember("la pluie") {
        println!("• {}", rec.narrative);
        println!("  [{}]", rec.disclaimer);
    }
}
