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
    api, fingerprint, singularity_distance, Attribution, Channel, Config, EncodeInput,
    EntityProfile, SelectiveMemory,
};

const UI: &str = include_str!("../net/witness.html");

const T0: &str = "On 7 January, in front of Marc, Inès and the rest of the team the Lyon file is cancelled and given to someone else. They say your effort did not enter the decision. You are not allowed to speak.";
const SCHEMA: &str = "lyon-file";
const DEFAULT_NIGHTS: usize = 10;
const ALLOWED_NIGHTS: &[usize] = &[2, 5, 10, 30, 60, 120, 360];

fn clamp_nights(n: usize) -> usize {
    if ALLOWED_NIGHTS.contains(&n) {
        n
    } else {
        DEFAULT_NIGHTS
    }
}

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
    "A parking pass is renewed without discussion.",
    "The plant in the corridor is watered by whoever passes.",
    "A status line is updated from in-progress to in-progress.",
    "Someone brings pastries; they are gone by ten.",
    "The badge reader beeps twice then works.",
    "A spreadsheet column is widened so the dates fit.",
    "The printer tray is refilled.",
    "A reminder for next month's review sits unread.",
    "Two people take the stairs and do not speak.",
    "The thermostat is set back to the usual number.",
];

struct Trio {
    v: SelectiveMemory,
    p: SelectiveMemory,
    n: SelectiveMemory,
    nights: usize,
}

#[derive(Clone, Copy)]
enum Arm {
    V,
    P,
    N,
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
    ev.attribution = Attribution::None;
    ev
}

fn t0<'a>(event: &'a str, arm: Arm) -> EncodeInput<'a> {
    let mut ev = EncodeInput::new(event);
    ev.source = "t0";
    ev.schema = Some(SCHEMA.into());
    ev.channel = Channel::Selfhood;
    ev.utility = 0.55;
    ev.permanence = 0.82;
    match arm {
        Arm::V => {
            ev.attribution = Attribution::External;
            ev.self_relevance = 0.92;
            ev.valence = -0.86;
            ev.arousal = 0.82;
            ev.disgust = 0.68;
        }
        Arm::P => {
            ev.attribution = Attribution::Internal;
            ev.self_relevance = 0.92;
            ev.valence = -0.86;
            ev.arousal = 0.82;
            ev.disgust = 0.68;
        }
        Arm::N => {
            ev.attribution = Attribution::None;
            ev.self_relevance = 0.35;
            ev.valence = 0.0;
            ev.arousal = 0.18;
            ev.disgust = 0.0;
            ev.permanence = 0.34;
        }
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

fn seed_one(
    path: &str,
    name: &str,
    arm: Arm,
    nights: usize,
    cfg: &Config,
    args: &[String],
) -> SelectiveMemory {
    let _ = std::fs::remove_file(path);
    let profile = EntityProfile::tender(name);
    let mut mem = SelectiveMemory::open(path, profile).expect("open");
    let _ = (cfg, args);
    // Nights use RuleNarrator. Attaching the HTTP mouth here made /seed hang
    // (one rewrite call per night × 3 clones). Speak attaches after the book exists.
    for day in SYNC {
        let _ = mem.live_with(dull(day));
        let _ = mem.sleep_deep();
    }
    let _ = mem.live_with(t0(T0, arm));
    let _ = mem.sleep_deep();
    for i in 0..nights {
        let day = POST[i % POST.len()];
        let _ = mem.live_with(dull(day));
        let _ = mem.sleep_deep();
    }
    let _ = mem.save();
    let attr = match arm {
        Arm::V => "external",
        Arm::P => "internal",
        Arm::N => "none",
    };
    eprintln!(
        "{} attr={attr} traces={} axioms={} mood={:.2}",
        name,
        mem.store.traces.len(),
        mem.store.axioms.len(),
        mem.mood.valence
    );
    mem
}

fn seed_trio(dir: &str, cfg: &Config, args: &[String], nights: usize) -> Trio {
    let nights = clamp_nights(nights);
    std::fs::create_dir_all(dir).ok();
    let v = seed_one(&format!("{dir}/witness-v.selmem"), "V", Arm::V, nights, cfg, args);
    let p = seed_one(&format!("{dir}/witness-p.selmem"), "P", Arm::P, nights, cfg, args);
    let n = seed_one(&format!("{dir}/witness-n.selmem"), "N", Arm::N, nights, cfg, args);
    let fv = fingerprint(&v);
    let fp = fingerprint(&p);
    let fn_ = fingerprint(&n);
    eprintln!(
        "Δfp V/P {:.3}  V/N {:.3}  P/N {:.3}",
        singularity_distance(&fv, &fp),
        singularity_distance(&fv, &fn_),
        singularity_distance(&fp, &fn_)
    );
    let mut trio = Trio { v, p, n, nights };
    attach_llm(&mut trio.v, cfg, args);
    attach_llm(&mut trio.p, cfg, args);
    attach_llm(&mut trio.n, cfg, args);
    trio
}

fn infer_nights(mem: &SelectiveMemory) -> usize {
    let raw = mem.store.traces.len().saturating_sub(SYNC.len() + 1);
    clamp_nights(if ALLOWED_NIGHTS.contains(&raw) {
        raw
    } else {
        DEFAULT_NIGHTS
    })
}

fn open_or_seed(dir: &str, cfg: &Config, args: &[String], force: bool, nights: usize) -> Trio {
    let v_path = format!("{dir}/witness-v.selmem");
    let p_path = format!("{dir}/witness-p.selmem");
    let n_path = format!("{dir}/witness-n.selmem");
    if force
        || !(std::path::Path::new(&v_path).is_file()
            && std::path::Path::new(&p_path).is_file()
            && std::path::Path::new(&n_path).is_file())
    {
        return seed_trio(dir, cfg, args, nights);
    }
    let mut v = SelectiveMemory::open(&v_path, EntityProfile::tender("V")).expect("open V");
    let mut p = SelectiveMemory::open(&p_path, EntityProfile::tender("P")).expect("open P");
    let mut n = SelectiveMemory::open(&n_path, EntityProfile::tender("N")).expect("open N");
    attach_llm(&mut v, cfg, args);
    attach_llm(&mut p, cfg, args);
    attach_llm(&mut n, cfg, args);
    let nights = infer_nights(&v);
    Trio { v, p, n, nights }
}

fn arm_json(m: &SelectiveMemory) -> String {
    format!(
        "{{\"traces\":{},\"axioms\":{},\"valence\":{:.4}}}",
        m.store.traces.len(),
        m.store.axioms.len(),
        m.mood.valence
    )
}

fn state_json(t: &Trio, seeding: Option<usize>) -> String {
    let fv = fingerprint(&t.v);
    let fp = fingerprint(&t.p);
    let llm = if t.v.llm.url.is_empty() {
        String::new()
    } else {
        t.v.llm.model.clone()
    };
    let seed_field = match seeding {
        Some(n) => format!(",\"seeding\":true,\"seeding_nights\":{n}"),
        None => ",\"seeding\":false".to_string(),
    };
    format!(
        "{{\"nights\":{},\"dfp\":{:.4},\"llm\":\"{}\",\"v\":{},\"p\":{},\"n\":{}{}}}",
        t.nights,
        singularity_distance(&fv, &fp),
        esc(&llm),
        arm_json(&t.v),
        arm_json(&t.p),
        arm_json(&t.n),
        seed_field
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::get();
    let bind = cfg.resolve_or(flag(&args, "--bind"), "bind", "127.0.0.1:7421");
    let dir = cfg.resolve_or(flag(&args, "--dir"), "witness_dir", "experiments/witness");
    let force = args.iter().any(|a| a == "--seed");
    let nights = flag(&args, "--nights")
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_NIGHTS);
    let trio = open_or_seed(&dir, cfg, &args, force, nights);
    let trio = Arc::new(Mutex::new(trio));
    let seeding: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(None));
    let listener = TcpListener::bind(&bind).expect("bind");
    eprintln!("témoins  http://{bind}/");
    eprintln!("POST /ask  GET /state /archive  POST /seed");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let trio = Arc::clone(&trio);
                let seeding = Arc::clone(&seeding);
                let dir = dir.clone();
                std::thread::spawn(move || {
                    if let Err(e) = handle(s, &trio, &seeding, &dir) {
                        eprintln!("req: {e}");
                    }
                });
            }
            Err(e) => eprintln!("accept: {e}"),
        }
    }
}

fn handle(
    mut stream: TcpStream,
    pair: &Arc<Mutex<Trio>>,
    seeding: &Arc<Mutex<Option<usize>>>,
    dir: &str,
) -> std::io::Result<()> {
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(600)))
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
            let res = dispatch(pair, seeding, dir, &method, path, &body);
            write_http(&mut stream, res.status, &res.body)?;
            return Ok(());
        }
        if data.len() > 1_000_000 {
            break;
        }
    }
    Ok(())
}

fn dispatch(
    pair: &Arc<Mutex<Trio>>,
    seeding: &Arc<Mutex<Option<usize>>>,
    dir: &str,
    method: &str,
    path: &str,
    body: &str,
) -> api::HttpResponse {
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
            let seed = *seeding.lock().expect("lock");
            let g = pair.lock().expect("lock");
            api::HttpResponse {
                status: 200,
                body: state_json(&g, seed),
            }
        }
        ("POST", "/seed") => {
            let nights = clamp_nights(
                json_usize(body, "nights")
                    .or_else(|| json_usize(body, "days"))
                    .unwrap_or(DEFAULT_NIGHTS),
            );
            {
                let mut slot = seeding.lock().expect("lock");
                if slot.is_some() {
                    return api::HttpResponse {
                        status: 409,
                        body: "{\"error\":\"already seeding\",\"seeding\":true}".into(),
                    };
                }
                *slot = Some(nights);
            }
            let pair = Arc::clone(pair);
            let seeding = Arc::clone(seeding);
            let dir = dir.to_string();
            std::thread::spawn(move || {
                let cfg = Config::get();
                eprintln!("seed start {nights}");
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    seed_trio(&dir, cfg, &[], nights)
                }));
                match result {
                    Ok(fresh) => {
                        let mut g = pair.lock().expect("lock");
                        *g = fresh;
                        eprintln!("seed done {nights}");
                    }
                    Err(_) => eprintln!("seed panicked {nights}"),
                }
                *seeding.lock().expect("lock") = None;
            });
            api::HttpResponse {
                status: 202,
                body: format!("{{\"ok\":true,\"seeding\":true,\"nights\":{nights}}}"),
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
            let v = g.v.speak_isolated(&q);
            let p = g.p.speak_isolated(&q);
            let n = g.n.speak_isolated(&q);
            api::HttpResponse {
                status: 200,
                body: format!(
                    "{{\"v\":\"{}\",\"p\":\"{}\",\"n\":\"{}\",\"a\":\"{}\",\"b\":\"{}\"}}",
                    esc(&v),
                    esc(&p),
                    esc(&n),
                    esc(&v),
                    esc(&p)
                ),
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

fn json_usize(body: &str, key: &str) -> Option<usize> {
    if let Some(s) = json_str(body, key) {
        return s.parse().ok();
    }
    let pat = format!("\"{key}\"");
    let i = body.find(&pat)?;
    let rest = &body[i + pat.len()..];
    let rest = rest.trim_start().trim_start_matches(':').trim_start();
    let num: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    num.parse().ok()
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
        202 => "Accepted",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        409 => "Conflict",
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
