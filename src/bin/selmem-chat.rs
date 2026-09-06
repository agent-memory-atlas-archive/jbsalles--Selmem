use std::env;
use std::io::{self, BufRead, Write};

use selmem::{EncodeInput, EntityProfile, HttpEmbedder, HttpNarrator, SelectiveMemory};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = flag(&args, "--path").unwrap_or_else(|| "claire.db".into());
    let name = flag(&args, "--name").unwrap_or_else(|| "Claire".into());
    let kind = flag(&args, "--profile").unwrap_or_else(|| "tender".into());
    let llm = flag(&args, "--llm").or_else(|| env::var("SELMEM_LLM").ok());
    let model = flag(&args, "--model")
        .or_else(|| env::var("SELMEM_MODEL").ok())
        .unwrap_or_else(|| "llama3".into());
    let key = flag(&args, "--api-key").or_else(|| env::var("SELMEM_API_KEY").ok());

    let profile = if kind == "austere" {
        EntityProfile::austere(name)
    } else {
        EntityProfile::tender(name)
    };
    let mut mem = SelectiveMemory::open(&path, profile).expect("mémoire");
    if let Some(url) = llm {
        if let Some(n) = HttpNarrator::parse(&url, model, key.clone()) {
            mem = mem.with_narrator(Box::new(n));
            eprintln!("voix LLM branchée");
        }
    }
    if let Some(url) = flag(&args, "--embed").or_else(|| env::var("SELMEM_EMBED").ok()) {
        if let Some(e) = HttpEmbedder::parse(&url, "text-embedding-3-small", key.clone()) {
            mem = mem.with_embedder(Box::new(e));
            eprintln!("embeddings HTTP branchés");
        }
    }

    eprintln!("{} écoute. /sleep /who /mood /quit", mem.profile.name);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut turns: u32 = 0;
    for line in stdin.lock().lines() {
        let line = line.expect("stdin");
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match line {
            "/quit" | "/exit" => break,
            "/sleep" => {
                let r = mem.sleep();
                let _ = mem.save();
                println!(
                    "(nuit) sculptés={} merged={} extinguished={} axiomes={}",
                    r.sculpted.len(),
                    r.merged,
                    r.extinguished,
                    r.axioms.len()
                );
            }
            "/who" => {
                for a in mem.who_am_i() {
                    println!("· {}", a.statement);
                }
                if mem.who_am_i().is_empty() {
                    println!("(pas encore d'axiome)");
                }
            }
            "/mood" => {
                println!(
                    "humeur v={:.2} a={:.2} d={:.2}",
                    mem.mood.valence, mem.mood.arousal, mem.mood.disgust
                );
            }
            _ => {
                let (valence, arousal, disgust, schema) = affect(line);
                let mut ev = EncodeInput::new(line);
                ev.valence = valence;
                ev.arousal = arousal;
                ev.disgust = disgust;
                ev.self_relevance = 0.75;
                ev.schema = schema;
                let d = mem.live_with(ev);
                let reply = mem.speak(line);
                let _ = mem.save();
                turns += 1;
                if turns % 5 == 0 {
                    mem.sleep();
                    let _ = mem.save();
                }
                let mark = if d.kept { "tenu" } else { "laissé" };
                println!("{}\n  [{} S={:.2}]", reply, mark, d.score);
            }
        }
        let _ = stdout.flush();
    }
}

fn affect(text: &str) -> (f32, f32, f32, Option<String>) {
    selmem::encode::affect::guess(text)
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.windows(2).find_map(|w| {
        if w[0] == name {
            Some(w[1].clone())
        } else {
            None
        }
    })
}
