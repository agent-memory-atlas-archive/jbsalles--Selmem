use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use selmem::{api, Config, EntityProfile, HttpEmbedder, HttpNarrator, SelectiveMemory};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::get();
    let bind = cfg.resolve_or(flag(&args, "--bind"), "bind", "127.0.0.1:7420");
    let path = cfg.resolve_or(flag(&args, "--path"), "path", "entity.db");
    let name = cfg.resolve_or(flag(&args, "--name"), "name", "Claire");
    let kind = cfg.resolve_or(flag(&args, "--profile"), "profile", "tender");
    let llm = cfg.resolve(flag(&args, "--llm"), "llm");
    let model = cfg.resolve_or(flag(&args, "--model"), "model", "llama3");
    let key = cfg.resolve(flag(&args, "--api-key"), "api_key");
    let embed_url = cfg.resolve(flag(&args, "--embed"), "embed");
    let token = cfg.resolve(flag(&args, "--token"), "token");

    let profile = match kind.as_str() {
        "austere" => EntityProfile::austere(name),
        _ => EntityProfile::tender(name),
    };

    let mut mem = SelectiveMemory::open(&path, profile).expect("failed to open memory");
    if let Some(v) = cfg
        .resolve(flag(&args, "--ground-overlap"), "ground_overlap")
        .and_then(|s| s.parse().ok())
    {
        mem.profile.ground_min_overlap = v;
    }
    if let Some(v) = cfg
        .resolve(flag(&args, "--ground-strikes"), "ground_strikes")
        .and_then(|s| s.parse().ok())
    {
        mem.profile.ground_strikes = v;
    }
    if let Some(v) = cfg
        .resolve(flag(&args, "--narrator-firmness"), "narrator_firmness")
        .and_then(|s| s.parse().ok())
    {
        mem.profile.narrator_firmness = v;
    }
    if let Some(endpoint) = llm {
        if let Some(n) = HttpNarrator::parse(&endpoint, model, key.clone()) {
            mem = mem.with_narrator(Box::new(n));
            eprintln!("HTTP narrator attached");
        } else {
            eprintln!("endpoint LLM illisible, repli RuleNarrator");
        }
    }
    if let Some(url) = embed_url {
        let emodel = cfg.resolve_or(flag(&args, "--embed-model"), "embed_model", "text-embedding-3-small");
        if let Some(e) = HttpEmbedder::parse(&url, emodel, key.clone()) {
            mem = mem.with_embedder(Box::new(e));
            eprintln!("HTTP embeddings attached");
        }
    }

    let mem = Arc::new(Mutex::new(mem));
    let listener = TcpListener::bind(&bind).expect("bind");
    eprintln!("selmemd sur http://{bind}  fichier={path}");
    if let Some(p) = cfg.path.as_ref() {
        eprintln!("config {}", p.display());
    }
    if token.is_some() {
        eprintln!("auth: Authorization: Bearer requis (sauf /health et /)");
    }
    eprintln!("UI  http://{bind}/");
    eprintln!("POST /turn /live /remember /sleep /speak /talk/clear   GET /who /lineage /mood /talk /health");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let mem = Arc::clone(&mem);
                let token = token.clone();
                std::thread::spawn(move || {
                    if let Err(e) = handle_conn(s, &mem, token.as_deref()) {
                        eprintln!("req: {e}");
                    }
                });
            }
            Err(e) => eprintln!("accept: {e}"),
        }
    }
}

fn handle_conn(
    mut stream: TcpStream,
    mem: &Mutex<SelectiveMemory>,
    token: Option<&str>,
) -> std::io::Result<()> {
    stream.set_read_timeout(Some(std::time::Duration::from_secs(90))).ok();
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
            let (path, query) = target.split_once('?').unwrap_or((target.as_str(), ""));
            let mut content_len = 0usize;
            let mut auth = String::new();
            for line in lines {
                let l = line.to_ascii_lowercase();
                if let Some(v) = l.strip_prefix("content-length:") {
                    content_len = v.trim().parse().unwrap_or(0);
                }
                if let Some(v) = line.strip_prefix("Authorization:") {
                    auth = v.trim().to_string();
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
            let body = String::from_utf8_lossy(&data[body_start..body_start + content_len.min(data.len().saturating_sub(body_start))]).to_string();
            if method == "GET" && (path == "/" || path == "/ui" || path == "/index.html") {
                write_http_ct(&mut stream, 200, "text/html; charset=utf-8", UI)?;
                return Ok(());
            }
            if let Some(tok) = token {
                let allowed = path == "/health" || method == "OPTIONS";
                let ok_auth = auth == format!("Bearer {tok}");
                if !allowed && !ok_auth {
                    write_http(&mut stream, 401, "{\"error\":\"unauthorized\"}")?;
                    return Ok(());
                }
            }
            let res = {
                let mut g = mem.lock().map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::Other, "memory locked")
                })?;
                api::dispatch(&mut g, &method, path, query, &body)
            };
            write_http(&mut stream, res.status, &res.body)?;
            return Ok(());
        }
        if data.len() > 1_000_000 {
            break;
        }
    }
    Ok(())
}

fn find_headers_end(data: &[u8]) -> Option<usize> {
    data.windows(4).position(|w| w == b"\r\n\r\n")
}

const UI: &str = include_str!("../net/ui.html");

fn write_http(stream: &mut TcpStream, status: u16, body: &str) -> std::io::Result<()> {
    write_http_ct(stream, status, "application/json; charset=utf-8", body)
}

fn write_http_ct(stream: &mut TcpStream, status: u16, ctype: &str, body: &str) -> std::io::Result<()> {
    let reason = match status {
        200 => "OK",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
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
