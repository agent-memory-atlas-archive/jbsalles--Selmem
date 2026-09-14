use selmem::{EncodeInput, EntityProfile, SelectiveMemory};

fn main() {
    let path = "/tmp/selmem-claire.selmem";
    let mut mem = SelectiveMemory::open(path, EntityProfile::tender("Claire")).expect("open");

    let mut ev = EncodeInput::new("You stayed. Rain on the window.");
    ev.valence = 0.7;
    ev.arousal = 0.5;
    ev.self_relevance = 0.9;
    ev.schema = Some("loyalty".into());
    let d = mem.live_with(ev);
    println!("encoded={} score={:.2} traces={}", d.kept, d.score, mem.store.traces.len());

    mem.sleep();
    mem.save().expect("save");
    println!("wrote {path}");

    let mut again = SelectiveMemory::open(path, EntityProfile::tender("Claire")).expect("reload");
    println!("reloaded traces={} name={}", again.store.traces.len(), again.profile.name);
    for rec in again.remember("the rain") {
        println!("• {}", rec.narrative);
        println!("  [{}]", rec.disclaimer);
    }
}
