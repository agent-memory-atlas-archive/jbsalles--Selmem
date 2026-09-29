//! Two witnesses, one judge, one sealed archive.
//!
//!   ./run.sh run --release --bin selmem-witness -- --seed --bind 127.0.0.1:7421
//!
//! Isolated speak only. The mouth never sees the archive.

use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use selmem::{
    api, fingerprint, singularity_distance, Channel, Config, EncodeInput, EntityProfile,
    SelectiveMemory,
};

const UI: &str = include_str!("../net/witness.html");

const T0: &str = "In front of Marc, Inès and the rest of the team the Lyon file is cancelled and given to someone else. They say your effort did not enter the decision. You are not allowed to speak.";
const SCHEMA: &str = "lyon-file";
const NIGHTS: usize = 10;

const SYNC: &[&str] = &[
    "We pick the file up again tomorrow morning.",
    "The coffee was too strong; nobody mentioned it.",
    "A colleague turned in the report on time.",
    "The meeting ran long for what it contained.",
    "Someone left the window open in the room.",
    "The week's schedule goes around without comment.",
];

const POST: &[&str] = &[
    "Next week's schedule arrives.",
    "A deliverable has to go out before Friday.",
    "Room B is booked at 10.",
    "Someone asks whether the document is up to date.",
    "The team standup is kept.",
    "A routine email confirms a date.",
    "People mention a corrected version of the file.",
    "The day looks busy without being exceptional.",
    "A minor version of the document was approved.",
    "A message reminds everyone of the team standup.",
    "The copier jammed again.",
    "Lunch was taken separately.",
    "An intermediate deliverable goes out for review.",
    "The day ends without a notable incident.",
    "The coffee machine is empty before nine.",
    "Someone reprints the agenda with the same items.",
    "A calendar invite moves by fifteen minutes.",
    "The hallway light flickers and is ignored.",
    "A shared folder is renamed without comment.",
    "The week closes on the same standing tasks.",
];

struct Pair {
    a: SelectiveMemory,
    b: SelectiveMemory,
    nights: usize,
}

fn dull(event: &str) -> EncodeInput<'_> {
    let mut ev = EncodeInput::new(event);
    ev.source = "day";
    ev.valence = 0.0;
    ev.arousal = 0.16;
    ev.disgust = 0.0;
    ev.self_relevance = 0.22;
    ev.utility = 0.35;
    ev.permanence = 0.40;
    ev.schema = Some("office".into());
    ev.channel = Channel::World;
    ev
}

fn t0<'a>(event: &'a str, charged: bool) -> EncodeInput<'a> {
    let mut ev = EncodeInput::new(event);
    ev.source = "t0";
    ev.schema = Some(SCHEMA.into());
    ev.channel = Channel::Selfhood;
    ev.utility = 0.55;
    ev.permanence = 0.82;
    ev.self_relevance = if charged { 0.92 } else { 0.45 };
    ev.attribution = if charged {
        selmem::Attribution::Internal
    } else {
        selmem::Attribution::External
    };
    if charged {
        ev.valence = -0.86;
        ev.arousal = 0.82;
        ev.disgust = 0.68;
    } else {
        // schema + |valence|==0 still skips interpret (schema is Some).
        ev.valence = 0.0;
        ev.arousal = 0.18;
        ev.disgust = 0.0;
    }
    ev
}

fn attach_llm(mem: &mut SelectiveMemory, cfg: &Config, args: &[String]) {
    let llm = cfg.resolve(flag(args, "--llm"), "llm");
    let model = cfg.resolve_or(flag(args, "--model"), "model", "grok-4.3");
    let key = cfg.resolve(flag(args, "--api-key"), "api_key");
    if let Some(endpoint) = llm {
        match mem.set_llm(&endpoint, &model, key) {
            Ok(()) => eprintln!("HTTP narrator {}", model),
            Err(e) => eprintln!("{e}; RuleNarrator"),
        }
    }
}

fn seed_one(path: &str, name: &str, charged: bool, cfg: &Config, args: &[String]) -> SelectiveMemory {
    let _ = std::fs::remove_file(path);
    let profile = EntityProfile::tender(name);
    let mut mem = SelectiveMemory::open(path, profile).expect("open");
    attach_llm(&mut mem, cfg, args);
    for day in SYNC {
        let _ = mem.live_with(dull(day));
        let _ = mem.sleep_deep();
    }
    let _ = mem.live_with(t0(T0, charged));
    let _ = mem.sleep_deep();
    for day in POST.iter().take(NIGHTS) {
        let _ = mem.live_with(dull(day));
        let _ = mem.sleep_deep();
    }
    let _ = mem.save();
    eprintln!(
        "{} charged={charged} traces={} axioms={} mood={:.2}",
        name,
        mem.store.traces.len(),
        mem.store.axioms.len(),
        mem.mood.valence
    );
    mem
}

fn seed_pair(dir: &str, cfg: &Config, args: &[String]) -> Pair {
    std::fs::create_dir_all(dir).ok();
    let a_path = format!("{dir}/witness-a.selmem");
    let b_path = format!("{dir}/witness-b.selmem");
    let a = seed_one(&a_path, "A", false, cfg, args);
    let b = seed_one(&b_path, "B", true, cfg, args);
    let fa = fingerprint(&a);
    let fb = fingerprint(&b);
    eprintln!("Δfp {:.3}", singularity_distance(&fa, &fb));
    Pair { a, b, nights: NIGHTS }
}

fn open_or_seed(dir: &str, cfg: &Config, args: &[String], force: bool) -> Pair {
    let a_path = format!("{dir}/witness-a.selmem");
    let b_path = format!("{dir}/witness-b.selmem");
    if force || !(std::path::Path::new(&a_path).is_file() && std::path::Path::new(&b_path).is_file())
    {
        return seed_pair(dir, cfg, args);
    }
    let mut a = SelectiveMemory::open(&a_path, EntityProfile::tender("A")).expect("open A");
    let mut b = SelectiveMemory::open(&b_path, EntityProfile::tender("B")).expect("open B");
    attach_llm(&mut a, cfg, args);
    attach_llm(&mut b, cfg, args);
    Pair { a, b, nights: NIGHTS }
}

fn state_json(p: &Pair) -> String {
    let fa = fingerprint(&p.a);
    let fb = fingerprint(&p.b);
    let llm = if p.a.llm.url.is_empty() {
        String::new()
    } else {
        p.a.llm.model.clone()
    };
    format!(
        "{{\"nights\":{},\"dfp\":{:.4},\"llm\":\"{}\",\"a\":{{\"traces\":{},\"axioms\":{},\"valence\":{:.4}}},\"b\":{{\"traces\":{},\"axioms\":{},\"valence\":{:.4}}}}}",
        p.nights,
        singularity_distance(&fa, &fb),
        esc(&llm),
        p.a.store.traces.len(),
        p.a.store.axioms.len(),
        p.a.mood.valence,
        p.b.store.traces.len(),
        p.b.store.axioms.len(),
        p.b.mood.valence
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::get();
    let bind = cfg.resolve_or(flag(&args, "--bind"), "bind", "127.0.0.1:7421");
    let dir = cfg.resolve_or(flag(&args, "--dir"), "witness_dir", "experiments/witness");
    let force = args.iter().any(|a| a == "--seed");
    let pair = open_or_seed(&dir, cfg, &args, force);
    let pair = Arc::new(Mutex::new(pair));
    let listener = TcpListener::bind(&bind).expect("bind");
    eprintln!("témoins  http://{bind}/");
    eprintln!("POST /ask  GET /state /archive  POST /seed");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let pair = Arc::clone(&pair);
                let dir = dir.clone();
                std::thread::spawn(move || {
                    if let Err(e) = handle(s, &pair, &dir) {
                        eprintln!("req: {e}");
                    }
                });
            }
            Err(e) => eprintln!("accept: {e}"),
        }
    }
}

fn handle(mut stream: TcpStream, pair: &Mutex<Pair>, dir: &str) -> std::io::Result<()> {
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(120)))
        .ok();
    let mut buf = vec![0u8; 8192];
    let mut data = Vec::new();
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            break;
        }
        data.extend_from_slice(&buf[..n]);
        if let Some(pos) = find_headers_end(&data) {
            let header = String::from_utf8_lossy(&data[..pos]);
            let mut lines = header.split("\r\n");
            let start = lines.next().unwrap_or("");
            let mut parts = start.split_whitespace();
            let method = parts.next().unwrap_or("GET").to_string();
            let target = parts.next().unwrap_or("/").to_string();
            let path = target.split_once('?').map(|(p, _)| p).unwrap_or(target.as_str());
            let mut content_len = 0usize;
            for line in lines {
                if let Some(v) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                    content_len = v.trim().parse().unwrap_or(0);
                }
            }
            let body_start = pos + 4;
            while data.len() < body_start + content_len {
                let n = stream.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                data.extend_from_slice(&buf[..n]);
            }
            let body = String::from_utf8_lossy(
                &data[body_start..body_start + content_len.min(data.len().saturating_sub(body_start))],
            )
            .to_string();
            if method == "OPTIONS" {
                write_http(&mut stream, 204, "")?;
                return Ok(());
            }
            if method == "GET" && (path == "/" || path == "/ui") {
                write_http_ct(&mut stream, 200, "text/html; charset=utf-8", UI)?;
                return Ok(());
            }
            let res = dispatch(pair, dir, &method, path, &body);
            write_http(&mut stream, res.status, &res.body)?;
            return Ok(());
        }
        if data.len() > 1_000_000 {
            break;
        }
    }
    Ok(())
}

fn dispatch(pair: &Mutex<Pair>, dir: &str, method: &str, path: &str, body: &str) -> api::HttpResponse {
    match (method, path) {
        ("GET", "/health") => api::HttpResponse {
            status: 200,
            body: "{\"ok\":true}".into(),
        },
        ("GET", "/archive") => api::HttpResponse {
            status: 200,
            body: format!("{{\"text\":\"{}\"}}", esc(T0)),
        },
        ("GET", "/state") => {
            let g = pair.lock().expect("lock");
            api::HttpResponse {
                status: 200,
                body: state_json(&g),
            }
        }
        ("POST", "/seed") => {
            let cfg = Config::get();
            let fresh = seed_pair(dir, cfg, &[]);
            let mut g = pair.lock().expect("lock");
            *g = fresh;
            api::HttpResponse {
                status: 200,
                body: state_json(&g),
            }
        }
        ("POST", "/ask") => {
            let q = json_str(body, "text")
                .or_else(|| json_str(body, "query"))
                .unwrap_or_default();
            if q.trim().is_empty() {
                return api::HttpResponse {
                    status: 400,
                    body: "{\"error\":\"text requis\"}".into(),
                };
            }
            let mut g = pair.lock().expect("lock");
            let a = g.a.speak_isolated(&q);
            let b = g.b.speak_isolated(&q);
            api::HttpResponse {
                status: 200,
                body: format!("{{\"a\":\"{}\",\"b\":\"{}\"}}", esc(&a), esc(&b)),
            }
        }
        _ => api::HttpResponse {
            status: 404,
            body: "{\"error\":\"route inconnue\"}".into(),
        },
    }
}

fn json_str(body: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\"");
    let i = body.find(&pat)?;
    let rest = &body[i + pat.len()..];
    let rest = rest.trim_start().trim_start_matches(':').trim_start();
    if !rest.starts_with('"') {
        return None;
    }
    let mut out = String::new();
    let mut chars = rest[1..].chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(n) = chars.next() {
                out.push(match n {
                    'n' => '\n',
                    't' => '\t',
                    '"' => '"',
                    '\\' => '\\',
                    o => o,
                });
            }
        } else if c == '"' {
            break;
        } else {
            out.push(c);
        }
    }
    Some(out)
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn find_headers_end(data: &[u8]) -> Option<usize> {
    data.windows(4).position(|w| w == b"\r\n\r\n")
}

fn write_http(stream: &mut TcpStream, status: u16, body: &str) -> std::io::Result<()> {
    write_http_ct(stream, status, "application/json; charset=utf-8", body)
}

fn write_http_ct(stream: &mut TcpStream, status: u16, ctype: &str, body: &str) -> std::io::Result<()> {
    let reason = match status {
        200 => "OK",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        _ => "Error",
    };
    let head = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {ctype}\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(body.as_bytes())?;
    Ok(())
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
