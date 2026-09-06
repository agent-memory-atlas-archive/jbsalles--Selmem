use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use selmem::{api, EntityProfile, HttpEmbedder, HttpNarrator, SelectiveMemory};

fn main() {
    let args: Vec<String> = env::args().collect();
    let bind = flag(&args, "--bind").unwrap_or_else(|| "127.0.0.1:7420".into());
    let path = flag(&args, "--path").unwrap_or_else(|| "entity.db".into());
    let name = flag(&args, "--name").unwrap_or_else(|| "Claire".into());
    let kind = flag(&args, "--profile").unwrap_or_else(|| "tender".into());
    let llm = flag(&args, "--llm").or_else(|| env::var("SELMEM_LLM").ok());
    let model = flag(&args, "--model").unwrap_or_else(|| "llama3".into());
    let key = flag(&args, "--api-key").or_else(|| env::var("SELMEM_API_KEY").ok());
    let embed_url = flag(&args, "--embed").or_else(|| env::var("SELMEM_EMBED").ok());
    let token = flag(&args, "--token").or_else(|| env::var("SELMEM_TOKEN").ok());

    let profile = match kind.as_str() {
        "austere" => EntityProfile::austere(name),
        _ => EntityProfile::tender(name),
    };

    let mut mem = SelectiveMemory::open(&path, profile).expect("impossible d'ouvrir la mémoire");
    if let Some(endpoint) = llm {
        if let Some(n) = HttpNarrator::parse(&endpoint, model, key.clone()) {
            mem = mem.with_narrator(Box::new(n));
            eprintln!("narrateur HTTP branché");
        } else {
            eprintln!("endpoint LLM illisible, repli RuleNarrator");
        }
    }
    if let Some(url) = embed_url {
        let emodel = flag(&args, "--embed-model").unwrap_or_else(|| "text-embedding-3-small".into());
        if let Some(e) = HttpEmbedder::parse(&url, emodel, key.clone()) {
            mem = mem.with_embedder(Box::new(e));
            eprintln!("embeddings HTTP branchés");
        }
    }

    let mem = Arc::new(Mutex::new(mem));
    let listener = TcpListener::bind(&bind).expect("bind");
    eprintln!("selmemd sur http://{bind}  fichier={path}");
    if token.is_some() {
        eprintln!("auth: Authorization: Bearer requis (sauf /health et /)");
    }
    eprintln!("UI  http://{bind}/");
    eprintln!("POST /turn /live /remember /sleep /speak   GET /who /lineage /mood /health");

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
                    write_http(&mut stream, 401, "{\"error\":\"non autorisé\"}")?;
                    return Ok(());
                }
            }
            let res = {
                let mut g = mem.lock().map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::Other, "mémoire verrouillée")
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
