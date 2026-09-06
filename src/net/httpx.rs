use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::Command;
use std::time::Duration;

pub fn post_json(url: &str, api_key: Option<&str>, body: &str) -> Result<String, String> {
    if url.starts_with("https://") || which_curl() {
        return curl_post(url, api_key, body);
    }
    raw_http_post(url, api_key, body)
}

fn which_curl() -> bool {
    Command::new("curl").arg("--version").output().is_ok()
}

fn curl_post(url: &str, api_key: Option<&str>, body: &str) -> Result<String, String> {
    let mut cmd = Command::new("curl");
    cmd.args(["-sS", "--max-time", "25", "-X", "POST", url, "-H", "Content-Type: application/json"]);
    if let Some(k) = api_key {
        cmd.arg("-H").arg(format!("Authorization: Bearer {k}"));
    }
    cmd.arg("--data-binary").arg(body);
    let out = cmd.output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).into());
    }
    String::from_utf8(out.stdout).map_err(|e| e.to_string())
}

fn raw_http_post(url: &str, api_key: Option<&str>, body: &str) -> Result<String, String> {
    let rest = url
        .strip_prefix("http://")
        .ok_or_else(|| "URL http(s) attendue".to_string())?;
    let (hostport, path) = rest.split_once('/').unwrap_or((rest, ""));
    let (host, port) = if let Some((h, p)) = hostport.split_once(':') {
        (h, p.parse::<u16>().unwrap_or(80))
    } else {
        (hostport, 80)
    };
    let path = if path.is_empty() {
        "/".to_string()
    } else if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    };
    let mut req = format!(
        "POST {path} HTTP/1.1\r\nHost: {host}:{port}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n",
        body.len()
    );
    if let Some(k) = api_key {
        req.push_str(&format!("Authorization: Bearer {k}\r\n"));
    }
    req.push_str("\r\n");
    req.push_str(body);
    let mut stream = TcpStream::connect(format!("{host}:{port}")).map_err(|e| e.to_string())?;
    stream.set_read_timeout(Some(Duration::from_secs(20))).ok();
    stream.write_all(req.as_bytes()).map_err(|e| e.to_string())?;
    let mut raw = String::new();
    stream.read_to_string(&mut raw).map_err(|e| e.to_string())?;
    Ok(raw.split("\r\n\r\n").nth(1).unwrap_or(&raw).to_string())
}

pub fn json_esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

pub fn extract_json_string(body: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\"");
    let mut last = None;
    let mut best: Option<String> = None;
    let mut search = body;
    while let Some(i) = search.find(&pat) {
        let abs = body.len() - search.len() + i;
        if let Some(s) = parse_json_string_value(&body[abs + pat.len()..]) {
            let skip = matches!(s.as_str(), "assistant" | "user" | "system" | "tool" | "model");
            if !skip && best.as_ref().map(|b| s.len() > b.len()).unwrap_or(true) {
                best = Some(s.clone());
            }
            last = Some(s);
        }
        search = &body[abs + pat.len()..];
    }
    best.or(last)
}

fn parse_json_string_value(after_key: &str) -> Option<String> {
    let after = after_key.trim_start().strip_prefix(':')?.trim_start();
    if !after.starts_with('"') {
        return None;
    }
    let bytes = after.as_bytes();
    let mut out = String::new();
    let mut j = 1;
    while j < bytes.len() {
        match bytes[j] {
            b'"' => return Some(out),
            b'\\' if j + 1 < bytes.len() => {
                match bytes[j + 1] {
                    b'n' => out.push('\n'),
                    b't' => out.push('\t'),
                    b'"' => out.push('"'),
                    b'\\' => out.push('\\'),
                    b'/' => out.push('/'),
                    c => out.push(c as char),
                }
                j += 2;
            }
            c => {
                out.push(c as char);
                j += 1;
            }
        }
    }
    None
}

pub fn extract_json_array_f32(body: &str, key: &str) -> Option<Vec<f32>> {
    let pat = format!("\"{key}\"");
    let mut search = body;
    let mut best: Option<Vec<f32>> = None;
    while let Some(i) = search.find(&pat) {
        let abs = body.len() - search.len() + i;
        let after = body[abs + pat.len()..].trim_start();
        if let Some(rest) = after.strip_prefix(':') {
            if let Some(vals) = parse_f32_array(rest.trim_start()) {
                if best.as_ref().map(|b| vals.len() > b.len()).unwrap_or(true) {
                    best = Some(vals);
                }
            }
        }
        search = &body[abs + pat.len()..];
    }
    best
}

fn parse_f32_array(after: &str) -> Option<Vec<f32>> {
    let start = after.find('[')?;
    let rest = &after[start + 1..];
    let end = rest.find(']')?;
    let vals: Vec<f32> = rest[..end]
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    if vals.is_empty() {
        None
    } else {
        Some(vals)
    }
}
