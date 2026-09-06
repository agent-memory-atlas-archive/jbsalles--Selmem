use selmem::{fingerprint, singularity_distance, Channel, EntityProfile, SelectiveMemory};
use selmem::EncodeInput;

struct Scene {
    event: &'static str,
    valence: f32,
    arousal: f32,
    disgust: f32,
    self_relevance: f32,
    schema: Option<&'static str>,
    channel: Channel,
    utility: f32,
    permanence: f32,
}

fn scenes() -> Vec<Scene> {
    vec![
        Scene {
            event: "Tu es resté. La pluie sur la fenêtre, et tu n'as pas cherché une excuse pour partir.",
            valence: 0.72,
            arousal: 0.55,
            disgust: 0.0,
            self_relevance: 0.9,
            schema: Some("fidélité"),
            channel: Channel::Selfhood,
            utility: 0.4,
            permanence: 0.4,
        },
        Scene {
            event: "Tu as annulé au dernier moment, sans raison, alors que j'avais tout préparé.",
            valence: -0.62,
            arousal: 0.7,
            disgust: 0.48,
            self_relevance: 0.85,
            schema: Some("abandon"),
            channel: Channel::Selfhood,
            utility: 0.4,
            permanence: 0.3,
        },
        Scene {
            event: "On a parlé de presque rien pendant une heure. C'était suffisant.",
            valence: 0.4,
            arousal: 0.25,
            disgust: 0.0,
            self_relevance: 0.6,
            schema: Some("présence"),
            channel: Channel::Selfhood,
            utility: 0.4,
            permanence: 0.0,
        },
        Scene {
            event: "Tu as ri de ce que je t'avais dit en confiance.",
            valence: -0.55,
            arousal: 0.65,
            disgust: 0.52,
            self_relevance: 0.8,
            schema: Some("humiliation"),
            channel: Channel::Selfhood,
            utility: 0.4,
            permanence: 0.0,
        },
        Scene {
            event: "Le rendez-vous est mardi 10h, salle B.",
            valence: 0.0,
            arousal: 0.1,
            disgust: 0.0,
            self_relevance: 0.1,
            schema: Some("agenda"),
            channel: Channel::World,
            utility: 0.9,
            permanence: 0.9,
        },
        Scene {
            event: "Un commentaire anodin sur le temps qu'il fait.",
            valence: 0.05,
            arousal: 0.05,
            disgust: 0.0,
            self_relevance: 0.05,
            schema: None,
            channel: Channel::Selfhood,
            utility: 0.05,
            permanence: 0.0,
        },
    ]
}

fn run(mut mem: SelectiveMemory) -> SelectiveMemory {
    println!("\n=== {} ===", mem.profile.name);
    for s in scenes() {
        let mut input = EncodeInput::new(s.event);
        input.valence = s.valence;
        input.arousal = s.arousal;
        input.disgust = s.disgust;
        input.self_relevance = s.self_relevance;
        input.schema = s.schema.map(|x| x.to_string());
        input.channel = s.channel;
        input.utility = s.utility;
        input.permanence = s.permanence;
        let d = mem.live_with(input);
        let mark = if d.kept { "tenu" } else { "oublié d'emblée" };
        let preview: String = s.event.chars().take(64).collect();
        println!("  [{mark} | S={:.2}] {preview}", d.score);
    }
    let report = mem.sleep();
    println!(
        "  -- sommeil -- sculptés={} merged={} extinguished={} cold={} myth={} axiomes={}",
        report.sculpted.len(),
        report.merged,
        report.extinguished,
        report.cold,
        report.myth,
        report.axioms.len()
    );
    for ax in mem.who_am_i() {
        println!("  axiome ({:.2}): {}", ax.strength, ax.statement);
    }
    println!("  -- rappel : ce soir-là --");
    for rec in mem.remember("ce soir-là, toi et moi") {
        println!("  • {}", rec.narrative);
        println!("    [{}]", rec.disclaimer);
    }
    mem
}

fn main() {
    let claire = run(SelectiveMemory::new(EntityProfile::tender("Claire")));
    let silas = run(SelectiveMemory::new(EntityProfile::austere("Silas")));
    let d = singularity_distance(&fingerprint(&claire), &fingerprint(&silas));
    println!("\nDistance de singularité Claire–Silas : {d:.3}  (0 = identiques, 1 = vies disjointes)");
}
