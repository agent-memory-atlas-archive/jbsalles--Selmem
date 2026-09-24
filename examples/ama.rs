//! AMA-Bench adapter: ingest a trajectory into a vault, dump retrieved context.
//!
//!   ./run.sh run --release --example ama -- construct --mode c2 --book /tmp/ep.selmem < traj.txt
//!   ./run.sh run --release --example ama -- retrieve --book /tmp/ep.selmem --question "…"
//!
//! Modes: `c2` (gate + one sleep + reconstruct), `static` (gate, freeze, stored gist),
//! `lastk` (no organ; packs printed so the Python method can keep them).
//! Construction does not call a speaker. Retrieve is read-only.

use selmem::{
    EntityProfile, OrganCut, RecallBias, RecallWrite, SelectiveMemory,
};

fn main() {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap_or_else(|| "help".into());
    match cmd.as_str() {
        "construct" => construct(&mut args),
        "retrieve" => retrieve(&mut args),
        _ => {
            eprintln!(
                "usage:\n  ama construct --mode c2|static|lastk --book PATH [--pack-words N] < traj\n  ama retrieve --book PATH --question TEXT [--mode c2|static]"
            );
            std::process::exit(2);
        }
    }
}

fn construct(args: &mut impl Iterator<Item = String>) {
    let mut mode = "c2".to_string();
    let mut book = String::new();
    let mut pack_words: usize = 600;
    let rest: Vec<String> = args.collect();
    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "--mode" => {
                mode = rest.get(i + 1).cloned().unwrap_or(mode);
                i += 2;
            }
            "--book" => {
                book = rest.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--pack-words" => {
                pack_words = rest
                    .get(i + 1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(pack_words);
                i += 2;
            }
            _ => i += 1,
        }
    }
    if book.is_empty() {
        eprintln!("construct needs --book PATH");
        std::process::exit(2);
    }
    let traj = std::io::read_to_string(std::io::stdin()).unwrap_or_default();
    let packs = pack_traj(&traj, pack_words);

    if mode == "lastk" {
        println!(
            "{{\"ok\":true,\"mode\":\"lastk\",\"packs\":{},\"kept\":{},\"traces\":0}}",
            packs.len(),
            packs.len()
        );
        return;
    }

    let mut mem = SelectiveMemory::open(&book, EntityProfile::tender("ama"))
        .unwrap_or_else(|e| {
            eprintln!("open {book}: {e}");
            std::process::exit(1);
        });
    mem.cut = if mode == "static" {
        OrganCut::static_book()
    } else {
        OrganCut::full()
    };

    let mut kept = 0u32;
    let mut seen = 0u32;
    for p in &packs {
        seen += 1;
        let d = mem.live_log(p);
        if d.kept {
            kept += 1;
        }
    }
    if mode == "c2" {
        let _ = mem.sleep();
    }
    if let Err(e) = mem.save() {
        eprintln!("save {book}: {e}");
        std::process::exit(1);
    }
    let traces = mem.store.traces.len();
    let axioms = mem.store.living_axioms().len();
    println!(
        "{{\"ok\":true,\"mode\":\"{}\",\"packs\":{},\"kept\":{},\"traces\":{},\"axioms\":{}}}",
        json_esc(&mode),
        seen,
        kept,
        traces,
        axioms
    );
}

fn retrieve(args: &mut impl Iterator<Item = String>) {
    let mut book = String::new();
    let mut question = String::new();
    let mut mode = "c2".to_string();
    let rest: Vec<String> = args.collect();
    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "--book" => {
                book = rest.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--question" => {
                question = rest.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--mode" => {
                mode = rest.get(i + 1).cloned().unwrap_or(mode);
                i += 2;
            }
            _ => i += 1,
        }
    }
    if book.is_empty() || question.is_empty() {
        eprintln!("retrieve needs --book PATH --question TEXT");
        std::process::exit(2);
    }
    let mut mem = SelectiveMemory::open(&book, EntityProfile::tender("ama")).unwrap_or_else(|e| {
        eprintln!("open {book}: {e}");
        std::process::exit(1);
    });
    mem.cut = if mode == "static" {
        OrganCut::static_book()
    } else {
        OrganCut::full()
    };
    let (memories, dump) = mem.remember_with(
        &question,
        RecallWrite::ReadOnly,
        RecallBias::Observed,
        &[],
    );

    let mut out = String::new();
    out.push_str("# axioms\n");
    for ax in mem.who_am_i() {
        out.push_str("- ");
        out.push_str(&ax.statement);
        out.push('\n');
    }
    out.push_str("\n# selected\n");
    for (i, m) in memories.iter().enumerate() {
        out.push_str(&format!("[{} id={}]\n", i + 1, m.trace_id));
        out.push_str(&m.narrative);
        out.push_str("\n\n");
    }
    if memories.is_empty() {
        out.push_str("(none)\n");
    }
    out.push_str(&format!(
        "\n# retrieve packs={} selected={} pulled={} recon={}\n",
        dump.candidates.len(),
        dump.selected.len(),
        dump.pulled,
        dump.reconsolidated
    ));
    print!("{out}");
}

fn pack_traj(traj: &str, pack_words: usize) -> Vec<String> {
    let turns = split_turns(traj);
    if turns.is_empty() {
        return Vec::new();
    }
    let mut packs = Vec::new();
    let mut buf = String::new();
    let mut words = 0usize;
    for t in turns {
        let w = t.split_whitespace().count();
        if !buf.is_empty() && words + w > pack_words {
            packs.push(std::mem::take(&mut buf));
            words = 0;
        }
        if !buf.is_empty() {
            buf.push_str("\n\n");
        }
        buf.push_str(&t);
        words += w;
    }
    if !buf.is_empty() {
        packs.push(buf);
    }
    packs
}

fn split_turns(traj: &str) -> Vec<String> {
    let mut documents = Vec::new();
    let mut current: Vec<&str> = Vec::new();
    for line in traj.lines() {
        let s = line.trim_start();
        if s.starts_with("Turn ") || s.starts_with("Step ") {
            if !current.is_empty() {
                documents.push(current.join("\n"));
                current.clear();
            }
        }
        current.push(line);
    }
    if !current.is_empty() {
        documents.push(current.join("\n"));
    }
    if documents.is_empty() && !traj.trim().is_empty() {
        documents.push(traj.to_string());
    }
    documents
}

fn json_esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
