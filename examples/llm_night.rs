//! One night with an HTTP narrator. Falls back to RuleNarrator if unset.
//!
//!   SELMEM_LLM=http://127.0.0.1:11434/v1/chat/completions \
//!   SELMEM_MODEL=llama3.1 \
//!   ./run.sh run --release --example llm_night
//!
//!   SELMEM_LLM=https://api.openai.com/v1/chat/completions \
//!   SELMEM_MODEL=gpt-4o-mini \
//!   SELMEM_API_KEY=sk-... \
//!   ./run.sh run --release --example llm_night

use selmem::{
    EncodeInput, EntityProfile, HttpNarrator, SelectiveMemory, Channel,
};

fn ingest(mut mem: SelectiveMemory, events: &[(&str, f32, f32, &str)]) -> SelectiveMemory {
    for (text, v, d, schema) in events {
        let mut ev = EncodeInput::new(text);
        ev.valence = *v;
        ev.arousal = 0.55;
        ev.disgust = *d;
        ev.self_relevance = 0.85;
        ev.schema = Some((*schema).into());
        ev.channel = Channel::Selfhood;
        mem.live_with(ev);
    }
    mem.sleep();
    mem
}

fn main() {
    let events = [
        (
            "Tu es resté. La pluie sur la fenêtre, et tu n'as pas cherché une excuse pour partir.",
            0.72,
            0.0,
            "fidélité",
        ),
        (
            "Tu as annulé au dernier moment, sans raison, alors que j'avais tout préparé.",
            -0.62,
            0.48,
            "abandon",
        ),
        (
            "Tu as ri de ce que je t'avais dit en confiance.",
            -0.55,
            0.52,
            "humiliation",
        ),
    ];

    let cfg = selmem::Config::get();
    let llm = cfg.llm();
    let model = cfg.model("gpt-4o-mini");
    let key = cfg.api_key();

    let mut claire = SelectiveMemory::new(EntityProfile::tender("Claire"));
    let mut silas = SelectiveMemory::new(EntityProfile::austere("Silas"));
    if let Some(url) = llm.as_deref() {
        if let Some(n) = HttpNarrator::parse(url, model.clone(), key.clone()) {
            println!("narrator HTTP {url} model={model}");
            claire = claire.with_narrator(Box::new(n));
        }
        if let Some(n) = HttpNarrator::parse(url, model, key) {
            silas = silas.with_narrator(Box::new(n));
        }
    } else {
        println!("no llm in .selmem / SELMEM_LLM — RuleNarrator / retell only");
    }

    let claire = ingest(claire, &events);
    let silas = ingest(silas, &events);
    println!("\n=== Claire ===");
    for t in claire.store.traces.values() {
        println!("• {}", t.gist);
    }
    println!("\n=== Silas ===");
    for t in silas.store.traces.values() {
        println!("• {}", t.gist);
    }
}
