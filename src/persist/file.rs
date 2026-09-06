use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use crate::core::model::{
    ArchiveRecord, AxiomLayer, Channel, DriftEvent, DriftKind, IdentityAxiom, MemoryTrace, Mood,
    TraceStatus,
};
use crate::core::profile::EntityProfile;
use crate::core::store::MemoryStore;

const MAGIC: &str = "SELMEM1";

pub struct Snapshot {
    pub profile: EntityProfile,
    pub mood: Mood,
    pub store: MemoryStore,
}

pub fn save(path: &Path, profile: &EntityProfile, mood: &Mood, store: &MemoryStore) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let tmp = path.with_extension("selmem.tmp");
    {
        let mut w = File::create(&tmp)?;
        writeln!(w, "{MAGIC}")?;
        write_profile(&mut w, profile)?;
        writeln!(w, "mood {} {} {}", mood.valence, mood.arousal, mood.disgust)?;

        writeln!(w, "archives {}", store.archives.len())?;
        for a in store.archives.values() {
            writeln!(w, "archive {} {}", a.id, a.created_at)?;
            write_blob(&mut w, &a.source)?;
            write_blob(&mut w, &a.verbatim)?;
        }

        writeln!(w, "traces {}", store.traces.len())?;
        for t in store.traces.values() {
            write_trace(&mut w, t)?;
        }

        writeln!(w, "axioms {}", store.axioms.len())?;
        for a in store.axioms.values() {
            write_axiom(&mut w, a)?;
        }

        let mut pairs = Vec::new();
        for (src, dsts) in &store.edges {
            for dst in dsts {
                if src < dst {
                    pairs.push((src.clone(), dst.clone()));
                }
            }
        }
        writeln!(w, "edges {}", pairs.len())?;
        for (src, dst) in pairs {
            writeln!(w, "edge {src} {dst}")?;
        }
    }
    fs::rename(tmp, path)?;
    Ok(())
}

pub fn load(path: &Path) -> io::Result<Snapshot> {
    let mut r = BufReader::new(File::open(path)?);
    let magic = read_line(&mut r)?;
    if magic != MAGIC {
        return fail(format!("fichier inconnu: {magic}"));
    }
    let profile = read_profile(&mut r)?;
    let mood = parse_mood(&read_line(&mut r)?)?;

    let mut store = MemoryStore::new();
    let n_arch = parse_count(&read_line(&mut r)?, "archives")?;
    for _ in 0..n_arch {
        let header = read_line(&mut r)?;
        let p: Vec<&str> = header.split_whitespace().collect();
        if p.len() < 3 || p[0] != "archive" {
            return fail("archive mal formée");
        }
        let source = read_blob(&mut r)?;
        let verbatim = read_blob(&mut r)?;
        let id = p[1].to_string();
        store.archives.insert(
            id.clone(),
            ArchiveRecord {
                id,
                created_at: parse_u64(p[2])?,
                source,
                verbatim,
            },
        );
    }

    let n_tr = parse_count(&read_line(&mut r)?, "traces")?;
    for _ in 0..n_tr {
        let t = read_trace(&mut r)?;
        store.traces.insert(t.id.clone(), t);
    }

    let n_ax = parse_count(&read_line(&mut r)?, "axioms")?;
    for _ in 0..n_ax {
        let a = read_axiom(&mut r)?;
        store.axioms.insert(a.id.clone(), a);
    }

    let n_ed = parse_count(&read_line(&mut r)?, "edges")?;
    for _ in 0..n_ed {
        let line = read_line(&mut r)?;
        let p: Vec<&str> = line.split_whitespace().collect();
        if p.len() != 3 || p[0] != "edge" {
            return fail("edge mal formée");
        }
        store.link(p[1], p[2]);
    }

    crate::persist::bump_id_counter(&store);
    Ok(Snapshot {
        profile,
        mood,
        store,
    })
}

fn write_profile(w: &mut impl Write, p: &EntityProfile) -> io::Result<()> {
    writeln!(w, "profile {}", p.name.replace(' ', "_"))?;
    writeln!(
        w,
        "params {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        p.encode_threshold,
        p.w_arousal,
        p.w_novelty,
        p.w_self,
        p.w_utility,
        p.w_goal,
        p.w_redundancy,
        p.decay_lambda,
        p.rehearsal_boost,
        p.embellish_gain,
        p.disgust_gain,
        p.disgust_cap,
        p.fidelity_loss_on_recall,
        p.reconsolidation_eta,
        p.mood_blend,
        p.cold_access,
        p.myth_access,
        p.max_recall,
        p.extinction_rate,
        p.merge_similarity
    )
}

fn read_profile(r: &mut impl BufRead) -> io::Result<EntityProfile> {
    let name_line = read_line(r)?;
    let name = name_line
        .strip_prefix("profile ")
        .ok_or_else(|| invalid("profile manquant"))?
        .to_string();
    let raw = read_line(r)?;
    let rest = raw.strip_prefix("params ").ok_or_else(|| invalid("params manquants"))?;
    let n: Vec<f32> = rest
        .split_whitespace()
        .map(|s| s.parse::<f32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(invalid)?;
    if n.len() < 18 {
        return fail("params incomplets");
    }
    Ok(EntityProfile {
        name,
        encode_threshold: n[0],
        w_arousal: n[1],
        w_novelty: n[2],
        w_self: n[3],
        w_utility: n[4],
        w_goal: n[5],
        w_redundancy: n[6],
        decay_lambda: n[7],
        rehearsal_boost: n[8],
        embellish_gain: n[9],
        disgust_gain: n[10],
        disgust_cap: n[11],
        fidelity_loss_on_recall: n[12],
        reconsolidation_eta: n[13],
        mood_blend: n[14],
        cold_access: n[15],
        myth_access: n[16],
        max_recall: n[17] as usize,
        extinction_rate: n.get(18).copied().unwrap_or(0.06),
        merge_similarity: n.get(19).copied().unwrap_or(0.32),
    })
}

fn write_trace(w: &mut impl Write, t: &MemoryTrace) -> io::Result<()> {
    writeln!(
        w,
        "trace {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        t.id,
        ch(t.channel),
        st(t.status),
        t.valence,
        t.arousal,
        t.disgust,
        t.self_relevance,
        t.fidelity,
        t.permanence,
        t.rehearsals,
        t.access,
        t.salience_at_encode,
        t.created_at,
        t.cues.len()
    )?;
    writeln!(
        w,
        "meta {} {} {}",
        opt(t.last_recalled_at),
        opt(t.last_consolidated_at),
        t.drifts.len()
    )?;
    write_blob(w, t.schema.as_deref().unwrap_or(""))?;
    write_blob(w, t.archive_id.as_deref().unwrap_or(""))?;
    write_blob(w, &t.gist)?;
    for c in &t.cues {
        write_blob(w, c)?;
    }
    for d in &t.drifts {
        writeln!(
            w,
            "drift {} {} {} {} {}",
            dk(d.kind),
            d.at,
            d.fidelity_delta,
            d.valence_delta,
            d.disgust_delta
        )?;
        write_blob(w, &d.note)?;
    }
    write!(w, "embed {}", t.embedding.len())?;
    for x in &t.embedding {
        write!(w, " {x}")?;
    }
    writeln!(w)?;
    write_blob(w, &t.core)?;
    writeln!(w, "anchor {}", t.anchor)?;
    Ok(())
}

fn read_trace(r: &mut impl BufRead) -> io::Result<MemoryTrace> {
    let header = read_line(r)?;
    let p: Vec<&str> = header.split_whitespace().collect();
    if p.len() < 15 || p[0] != "trace" {
        return fail("trace mal formée");
    }
    let cue_n: usize = p[14].parse().map_err(invalid)?;
    let meta = read_line(r)?;
    let m: Vec<&str> = meta.split_whitespace().collect();
    if m.len() < 4 || m[0] != "meta" {
        return fail("meta trace manquante");
    }
    let schema = empty_none(read_blob(r)?);
    let archive_id = empty_none(read_blob(r)?);
    let gist = read_blob(r)?;
    let mut cues = Vec::with_capacity(cue_n);
    for _ in 0..cue_n {
        cues.push(read_blob(r)?);
    }
    let drift_n: usize = m[3].parse().map_err(invalid)?;
    let mut drifts = Vec::with_capacity(drift_n);
    for _ in 0..drift_n {
        drifts.push(read_drift(r)?);
    }
    Ok(MemoryTrace {
        id: p[1].to_string(),
        channel: parse_ch(p[2])?,
        status: parse_st(p[3])?,
        valence: parse_f(p[4])?,
        arousal: parse_f(p[5])?,
        disgust: parse_f(p[6])?,
        self_relevance: parse_f(p[7])?,
        fidelity: parse_f(p[8])?,
        permanence: parse_f(p[9])?,
        rehearsals: p[10].parse().map_err(invalid)?,
        access: parse_f(p[11])?,
        salience_at_encode: parse_f(p[12])?,
        created_at: parse_u64(p[13])?,
        last_recalled_at: parse_opt(m[1])?,
        last_consolidated_at: parse_opt(m[2])?,
        schema,
        archive_id,
        gist,
        cues,
        drifts,
        embedding: {
            let line = read_line(r).unwrap_or_default();
            if let Some(rest) = line.strip_prefix("embed ") {
                rest.split_whitespace()
                    .skip(1)
                    .filter_map(|s| s.parse().ok())
                    .collect()
            } else {
                Vec::new()
            }
        },
        core: read_blob(r).unwrap_or_default(),
        anchor: {
            let line = read_line(r).unwrap_or_default();
            line.strip_prefix("anchor ")
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(0.0)
        },
    })
}

fn read_drift(r: &mut impl BufRead) -> io::Result<DriftEvent> {
    let line = read_line(r)?;
    let p: Vec<&str> = line.split_whitespace().collect();
    if p.len() < 6 || p[0] != "drift" {
        return fail("drift mal formée");
    }
    Ok(DriftEvent {
        kind: parse_dk(p[1])?,
        at: parse_u64(p[2])?,
        fidelity_delta: parse_f(p[3])?,
        valence_delta: parse_f(p[4])?,
        disgust_delta: parse_f(p[5])?,
        note: read_blob(r)?,
    })
}

fn write_axiom(w: &mut impl Write, a: &IdentityAxiom) -> io::Result<()> {
    writeln!(
        w,
        "axiom {} {} {} {} {} {} {}",
        a.id,
        a.created_at,
        a.valence,
        a.strength,
        a.support_trace_ids.len(),
        a.superseded_by.as_deref().unwrap_or("-"),
        layer_name(a.layer)
    )?;
    write_blob(w, &a.statement)?;
    write_blob(w, a.schema.as_deref().unwrap_or(""))?;
    for id in &a.support_trace_ids {
        writeln!(w, "support {id}")?;
    }
    Ok(())
}

fn read_axiom(r: &mut impl BufRead) -> io::Result<IdentityAxiom> {
    let header = read_line(r)?;
    let p: Vec<&str> = header.split_whitespace().collect();
    if p.len() < 7 || p[0] != "axiom" {
        return fail("axiom mal formée");
    }
    let layer = if p.len() >= 8 {
        parse_layer(p[7])
    } else {
        AxiomLayer::Belief
    };
    let n_sup: usize = p[5].parse().map_err(invalid)?;
    let statement = read_blob(r)?;
    let schema = empty_none(read_blob(r).unwrap_or_default());
    let mut support = Vec::with_capacity(n_sup);
    for _ in 0..n_sup {
        let line = read_line(r)?;
        support.push(
            line.strip_prefix("support ")
                .ok_or_else(|| invalid("support manquant"))?
                .to_string(),
        );
    }
    Ok(IdentityAxiom {
        id: p[1].to_string(),
        created_at: parse_u64(p[2])?,
        valence: parse_f(p[3])?,
        strength: parse_f(p[4])?,
        superseded_by: if p[6] == "-" {
            None
        } else {
            Some(p[6].to_string())
        },
        statement,
        schema,
        support_trace_ids: support,
        layer,
    })
}

fn write_blob(w: &mut impl Write, s: &str) -> io::Result<()> {
    let bytes = s.as_bytes();
    writeln!(w, "{}", bytes.len())?;
    w.write_all(bytes)?;
    w.write_all(b"\n")?;
    Ok(())
}

fn read_blob(r: &mut impl BufRead) -> io::Result<String> {
    let len: usize = read_line(r)?.parse().map_err(invalid)?;
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf)?;
    let mut nl = [0u8; 1];
    r.read_exact(&mut nl)?;
    if nl[0] != b'\n' {
        return fail("blob non terminé par newline");
    }
    String::from_utf8(buf).map_err(invalid)
}

fn read_line(r: &mut impl BufRead) -> io::Result<String> {
    let mut s = String::new();
    let n = r.read_line(&mut s)?;
    if n == 0 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "fin de fichier"));
    }
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    Ok(s)
}

fn parse_count(line: &str, key: &str) -> io::Result<usize> {
    let prefix = format!("{key} ");
    line.strip_prefix(&prefix)
        .ok_or_else(|| invalid(format!("{key} manquant")))?
        .parse()
        .map_err(invalid)
}

fn parse_mood(line: &str) -> io::Result<Mood> {
    let rest = line.strip_prefix("mood ").ok_or_else(|| invalid("mood manquant"))?;
    let p: Vec<&str> = rest.split_whitespace().collect();
    if p.len() < 3 {
        return fail("mood incomplet");
    }
    Ok(Mood {
        valence: parse_f(p[0])?,
        arousal: parse_f(p[1])?,
        disgust: parse_f(p[2])?,
    })
}

fn opt(v: Option<u64>) -> String {
    v.map(|n| n.to_string()).unwrap_or_else(|| "-".into())
}

fn parse_opt(s: &str) -> io::Result<Option<u64>> {
    if s == "-" {
        Ok(None)
    } else {
        Ok(Some(parse_u64(s)?))
    }
}

fn empty_none(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn ch(c: Channel) -> &'static str {
    match c {
        Channel::Selfhood => "self",
        Channel::World => "world",
    }
}
fn parse_ch(s: &str) -> io::Result<Channel> {
    match s {
        "self" => Ok(Channel::Selfhood),
        "world" => Ok(Channel::World),
        _ => fail("channel inconnu"),
    }
}
fn st(s: TraceStatus) -> &'static str {
    match s {
        TraceStatus::Active => "active",
        TraceStatus::Cold => "cold",
        TraceStatus::Myth => "myth",
        TraceStatus::Sealed => "sealed",
    }
}
fn parse_st(s: &str) -> io::Result<TraceStatus> {
    match s {
        "active" => Ok(TraceStatus::Active),
        "cold" => Ok(TraceStatus::Cold),
        "myth" => Ok(TraceStatus::Myth),
        "sealed" => Ok(TraceStatus::Sealed),
        _ => fail("status inconnu"),
    }
}
fn dk(k: DriftKind) -> &'static str {
    match k {
        DriftKind::Embellish => "embellish",
        DriftKind::AmplifyDisgust => "disgust",
        DriftKind::Fade => "fade",
        DriftKind::Merge => "merge",
        DriftKind::Weather => "weather",
        DriftKind::Rewrite => "rewrite",
        DriftKind::Reinterpret => "reinterpret",
    }
}
fn parse_dk(s: &str) -> io::Result<DriftKind> {
    match s {
        "embellish" => Ok(DriftKind::Embellish),
        "disgust" => Ok(DriftKind::AmplifyDisgust),
        "fade" => Ok(DriftKind::Fade),
        "merge" => Ok(DriftKind::Merge),
        "weather" => Ok(DriftKind::Weather),
        "rewrite" => Ok(DriftKind::Rewrite),
        "reinterpret" => Ok(DriftKind::Reinterpret),
        _ => fail("drift inconnue"),
    }
}
fn layer_name(l: AxiomLayer) -> &'static str {
    match l {
        AxiomLayer::Motif => "motif",
        AxiomLayer::Belief => "belief",
        AxiomLayer::Trait => "trait",
    }
}
fn parse_layer(s: &str) -> AxiomLayer {
    match s {
        "motif" => AxiomLayer::Motif,
        "trait" => AxiomLayer::Trait,
        _ => AxiomLayer::Belief,
    }
}

fn parse_f(s: &str) -> io::Result<f32> {
    s.parse().map_err(invalid)
}
fn parse_u64(s: &str) -> io::Result<u64> {
    s.parse().map_err(invalid)
}
fn fail<T>(msg: impl Into<String>) -> io::Result<T> {
    Err(io::Error::new(io::ErrorKind::InvalidData, msg.into()))
}
fn invalid<E: std::fmt::Display>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, e.to_string())
}
