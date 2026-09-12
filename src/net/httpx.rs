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
    let timeout = std::env::var("SELMEM_HTTP_TIMEOUT").unwrap_or_else(|_| "60".into());
    cmd.args(["-sS", "--max-time", &timeout, "-X", "POST", url, "-H", "Content-Type: application/json"]);
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
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// First `"key": "…"` whose key is a real JSON key, not a substring.
/// UTF-8 safe. Prefers `choices[0].message.content` when `key == "content"`.
pub fn extract_json_string(body: &str, key: &str) -> Option<String> {
    if key == "content" {
        if let Some(s) = chat_message_content(body) {
            return Some(s);
        }
    }
    first_string_field(body, key)
}

pub fn first_string_field(json: &str, key: &str) -> Option<String> {
    let mut i = 0;
    let bytes = json.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'"' {
            let (s, n) = parse_json_string(&json[i..])?;
            let after = json[i + n..].trim_start();
            if s == key && after.starts_with(':') {
                let val = after[1..].trim_start();
                if val.starts_with('"') {
                    return parse_json_string(val).map(|(v, _)| v);
                }
            }
            i += n;
            continue;
        }
        i += 1;
    }
    None
}

pub fn first_string_array(json: &str, key: &str) -> Option<Vec<String>> {
    let mut i = 0;
    let bytes = json.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'"' {
            let (s, n) = parse_json_string(&json[i..])?;
            let after = json[i + n..].trim_start();
            if s == key && after.starts_with(':') {
                let val = after[1..].trim_start();
                if val.starts_with('[') {
                    return Some(parse_string_array(val));
                }
            }
            i += n;
            continue;
        }
        i += 1;
    }
    None
}

fn chat_message_content(body: &str) -> Option<String> {
    // choices -> first object -> message -> content
    let mut i = 0;
    let bytes = body.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'"' {
            let (s, n) = parse_json_string(&body[i..])?;
            let after = body[i + n..].trim_start();
            if s == "choices" && after.starts_with(':') {
                let val = after[1..].trim_start();
                return content_in_first_choice(val);
            }
            i += n;
            continue;
        }
        i += 1;
    }
    None
}

fn content_in_first_choice(after_colon: &str) -> Option<String> {
    let arr = after_colon.trim_start();
    if !arr.starts_with('[') {
        return None;
    }
    let inner = arr[1..].trim_start();
    if !inner.starts_with('{') {
        return None;
    }
    // message.content — not a sibling "content" on a tool/reasoning blob.
    if let Some(msg) = object_after_key(inner, "message") {
        for key in ["content", "reasoning_content", "reasoning", "text", "output"] {
            if let Some(s) = first_string_field(msg, key) {
                if !s.trim().is_empty() {
                    return Some(s);
                }
            }
        }
    }
    for key in ["content", "reasoning_content", "reasoning", "text"] {
        if let Some(s) = first_string_field(inner, key) {
            if !s.trim().is_empty() {
                return Some(s);
            }
        }
    }
    None
}

fn object_after_key<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let mut i = 0;
    let bytes = json.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'"' {
            let (s, n) = parse_json_string(&json[i..])?;
            let after = json[i + n..].trim_start();
            if s == key && after.starts_with(':') {
                let val = after[1..].trim_start();
                if val.starts_with('{') {
                    return Some(val);
                }
            }
            i += n;
            continue;
        }
        i += 1;
    }
    None
}

pub fn first_number_field(json: &str, key: &str) -> Option<f32> {
    let mut i = 0;
    let bytes = json.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'"' {
            let (s, n) = parse_json_string(&json[i..])?;
            let after = json[i + n..].trim_start();
            if s == key && after.starts_with(':') {
                let val = after[1..].trim_start();
                let num: String = val
                    .chars()
                    .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
                    .collect();
                return num.parse().ok();
            }
            i += n;
            continue;
        }
        i += 1;
    }
    None
}

/// `s` starts at the opening quote. Returns (decoded, bytes consumed).
pub fn parse_json_string(s: &str) -> Option<(String, usize)> {
    let bytes = s.as_bytes();
    if bytes.first() != Some(&b'"') {
        return None;
    }
    let mut out = String::new();
    let mut i = 1;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => return Some((out, i + 1)),
            b'\\' if i + 1 < bytes.len() => {
                match bytes[i + 1] {
                    b'n' => out.push('\n'),
                    b'r' => out.push('\r'),
                    b't' => out.push('\t'),
                    b'"' => out.push('"'),
                    b'\\' => out.push('\\'),
                    b'/' => out.push('/'),
                    b'u' if i + 5 < bytes.len() => {
                        let hex = s.get(i + 2..i + 6)?;
                        if let Ok(cp) = u32::from_str_radix(hex, 16) {
                            if let Some(ch) = char::from_u32(cp) {
                                out.push(ch);
                            }
                        }
                        i += 6;
                        continue;
                    }
                    _ => {}
                }
                i += 2;
            }
            _ => {
                let ch = s[i..].chars().next()?;
                out.push(ch);
                i += ch.len_utf8();
            }
        }
    }
    None
}

fn parse_string_array(s: &str) -> Vec<String> {
    let mut body = s.trim_start();
    if !body.starts_with('[') {
        return Vec::new();
    }
    body = body[1..].trim_start();
    let mut out = Vec::new();
    loop {
        body = body.trim_start();
        if body.is_empty() || body.starts_with(']') {
            break;
        }
        if body.starts_with(',') {
            body = body[1..].trim_start();
            continue;
        }
        if body.starts_with('"') {
            if let Some((v, n)) = parse_json_string(body) {
                out.push(v);
                body = &body[n..];
                continue;
            }
        }
        break;
    }
    out
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
