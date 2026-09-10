use crate::encode::EncodeInput;
use crate::engine::SelectiveMemory;
use crate::core::model::Channel;

pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

pub fn dispatch(mem: &mut SelectiveMemory, method: &str, path: &str, query: &str, body: &str) -> HttpResponse {
    if method == "OPTIONS" {
        return HttpResponse {
            status: 204,
            body: String::new(),
        };
    }
    match (method, path) {
        ("GET", "/health") => ok(format!(
            "{{\"ok\":true,\"name\":\"{}\",\"traces\":{},\"archives\":{},\"axioms\":{}}}",
            json_esc(&mem.profile.name),
            mem.store.traces.len(),
            mem.store.archives.len(),
            mem.store.axioms.len()
        )),
        ("GET", "/profile") => ok(format!(
            "{{\"name\":\"{}\",\"encode_threshold\":{:.4},\"embellish_gain\":{:.4},\"disgust_gain\":{:.4},\"decay_lambda\":{:.4},\"ground_min_overlap\":{:.4},\"ground_strikes\":{},\"narrator_firmness\":{:.4}}}",
            json_esc(&mem.profile.name),
            mem.profile.encode_threshold,
            mem.profile.embellish_gain,
            mem.profile.disgust_gain,
            mem.profile.decay_lambda,
            mem.profile.ground_min_overlap,
            mem.profile.ground_strikes,
            mem.profile.narrator_firmness
        )),
        ("POST", "/profile") => {
            if let Some(v) = json_f32(body, "encode_threshold") {
                mem.profile.encode_threshold = v;
            }
            if let Some(v) = json_f32(body, "embellish_gain") {
                mem.profile.embellish_gain = v;
            }
            if let Some(v) = json_f32(body, "disgust_gain") {
                mem.profile.disgust_gain = v;
            }
            if let Some(v) = json_f32(body, "decay_lambda") {
                mem.profile.decay_lambda = v;
            }
            if let Some(v) = json_f32(body, "ground_min_overlap") {
                mem.profile.ground_min_overlap = v;
            }
            if let Some(v) = json_f32(body, "ground_strikes") {
                mem.profile.ground_strikes = v.max(1.0) as usize;
            }
            if let Some(v) = json_f32(body, "narrator_firmness") {
                mem.profile.narrator_firmness = v.clamp(0.0, 1.0);
            }
            ok("{\"ok\":true}".into())
        }
        ("GET", "/mood") => ok(format!(
            "{{\"valence\":{:.4},\"arousal\":{:.4},\"disgust\":{:.4}}}",
            mem.mood.valence, mem.mood.arousal, mem.mood.disgust
        )),
        ("GET", "/who") => {
            let axioms: Vec<String> = mem
                .who_am_i()
                .into_iter()
                .map(|a| {
                    format!(
                        "{{\"id\":\"{}\",\"layer\":\"{}\",\"strength\":{:.3},\"valence\":{:.3},\"statement\":\"{}\"}}",
                        json_esc(&a.id),
                        match a.layer {
                            crate::core::model::AxiomLayer::Motif => "motif",
                            crate::core::model::AxiomLayer::Belief => "belief",
                            crate::core::model::AxiomLayer::Trait => "trait",
                        },
                        a.strength,
                        a.valence,
                        json_esc(&a.statement)
                    )
                })
                .collect();
            ok(format!("{{\"name\":\"{}\",\"axioms\":[{}]}}", json_esc(&mem.profile.name), axioms.join(",")))
        }
        ("GET", "/lineage") => {
            let schema = query_param(query, "schema").unwrap_or("");
            let items: Vec<String> = mem
                .lineage(schema)
                .into_iter()
                .map(|a| {
                    format!(
                        "{{\"id\":\"{}\",\"statement\":\"{}\",\"superseded_by\":\"{}\"}}",
                        json_esc(&a.id),
                        json_esc(&a.statement),
                        json_esc(a.superseded_by.as_deref().unwrap_or(""))
                    )
                })
                .collect();
            ok(format!("{{\"schema\":\"{}\",\"chain\":[{}]}}", json_esc(schema), items.join(",")))
        }
        ("GET", "/audit") => {
            let id = query_param(query, "id").unwrap_or("");
            match mem.audit(id) {
                Some(v) => ok(format!(
                    "{{\"trace_id\":\"{}\",\"verbatim\":\"{}\"}}",
                    json_esc(id),
                    json_esc(v)
                )),
                None => err(404, "archive introuvable"),
            }
        }
        ("POST", "/turn") => {
            let text = json_str(body, "text")
                .or_else(|| json_str(body, "event"))
                .unwrap_or_default();
            if text.trim().is_empty() {
                return err(400, "text requis");
            }
            let mut input = EncodeInput::new(&text);
            let (v, a, d, schema) = guess_affect(&text);
            input.valence = json_f32(body, "valence").unwrap_or(v);
            input.arousal = json_f32(body, "arousal").unwrap_or(a);
            input.disgust = json_f32(body, "disgust").unwrap_or(d);
            input.self_relevance = json_f32(body, "self_relevance").unwrap_or(0.75);
            if let Some(s) = json_str(body, "schema") {
                input.schema = Some(s);
            } else {
                input.schema = schema;
            }
            let dec = mem.live_with(input);
            let reply = mem.speak(&text);
            let _ = mem.save();
            ok(format!(
                "{{\"kept\":{},\"score\":{:.4},\"reason\":\"{}\",\"reply\":\"{}\",\"mood\":{{\"valence\":{:.3},\"arousal\":{:.3},\"disgust\":{:.3}}}}}",
                if dec.kept { "true" } else { "false" },
                dec.score,
                json_esc(&dec.reason),
                json_esc(&reply),
                mem.mood.valence,
                mem.mood.arousal,
                mem.mood.disgust
            ))
        }
        ("POST", "/live") => {
            let event = json_str(body, "event").unwrap_or_default();
            if event.trim().is_empty() {
                return err(400, "event requis");
            }
            let mut input = EncodeInput::new(&event);
            if let Some(v) = json_f32(body, "valence") {
                input.valence = v;
            }
            if let Some(v) = json_f32(body, "arousal") {
                input.arousal = v;
            }
            if let Some(v) = json_f32(body, "disgust") {
                input.disgust = v;
            }
            if let Some(v) = json_f32(body, "self_relevance") {
                input.self_relevance = v;
            }
            if let Some(v) = json_f32(body, "utility") {
                input.utility = v;
            }
            if let Some(v) = json_f32(body, "goal_align") {
                input.goal_align = v;
            }
            if let Some(v) = json_f32(body, "permanence") {
                input.permanence = v;
            }
            if let Some(s) = json_str(body, "schema") {
                if !s.is_empty() {
                    input.schema = Some(s);
                }
            }
            if let Some(ch) = json_str(body, "channel") {
                input.channel = if ch == "world" {
                    Channel::World
                } else {
                    Channel::Selfhood
                };
            }
            let d = mem.live_with(input);
            let _ = mem.save();
            let tid = d.trace_id.as_deref().unwrap_or("");
            ok(format!(
                "{{\"kept\":{},\"score\":{:.4},\"reason\":\"{}\",\"trace_id\":\"{}\",\"archive_id\":\"{}\"}}",
                if d.kept { "true" } else { "false" },
                d.score,
                json_esc(&d.reason),
                json_esc(tid),
                json_esc(&d.archive_id)
            ))
        }
        ("POST", "/remember") => {
            let query_txt = json_str(body, "query").unwrap_or_default();
            if query_txt.trim().is_empty() {
                return err(400, "query requis");
            }
            let recs = mem.remember(&query_txt);
            let _ = mem.save();
            let items: Vec<String> = recs
                .iter()
                .map(|r| {
                    format!(
                        "{{\"trace_id\":\"{}\",\"fidelity\":{:.3},\"channel\":\"{}\",\"schema\":\"{}\",\"disclaimer\":\"{}\",\"narrative\":\"{}\"}}",
                        json_esc(&r.trace_id),
                        r.fidelity,
                        if matches!(r.channel, Channel::World) { "world" } else { "self" },
                        json_esc(r.schema.as_deref().unwrap_or("")),
                        json_esc(&r.disclaimer),
                        json_esc(&r.narrative)
                    )
                })
                .collect();
            ok(format!("{{\"memories\":[{}]}}", items.join(",")))
        }
        ("POST", "/sleep") => {
            let report = mem.sleep();
            let _ = mem.save();
            ok(format!(
                "{{\"faded\":{},\"cold\":{},\"myth\":{},\"merged\":{},\"extinguished\":{},\"weathered\":{},\"rewritten\":{},\"sculpted\":{},\"axioms\":{}}}",
                report.faded,
                report.cold,
                report.myth,
                report.merged,
                report.extinguished,
                report.weathered,
                report.rewritten,
                report.sculpted.len(),
                report.axioms.len()
            ))
        }
        ("POST", "/speak") => {
            let user = json_str(body, "text")
                .or_else(|| json_str(body, "query"))
                .unwrap_or_default();
            if user.trim().is_empty() {
                return err(400, "text requis");
            }
            let reply = mem.speak(&user);
            let _ = mem.save();
            ok(format!("{{\"reply\":\"{}\"}}", json_esc(&reply)))
        }
        ("POST", "/save") => match mem.save() {
            Ok(()) => ok("{\"saved\":true}".into()),
            Err(e) => err(500, &e.to_string()),
        },
        _ => err(404, "route inconnue"),
    }
}

fn ok(body: String) -> HttpResponse {
    HttpResponse { status: 200, body }
}

fn err(status: u16, msg: &str) -> HttpResponse {
    HttpResponse {
        status,
        body: format!("{{\"error\":\"{}\"}}", json_esc(msg)),
    }
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

fn query_param<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    query.split('&').find_map(|pair| {
        let (k, v) = pair.split_once('=')?;
        if k == key {
            Some(v)
        } else {
            None
        }
    })
}

fn json_str(body: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\"");
    let i = body.find(&pat)?;
    let after = body[i + pat.len()..].trim_start();
    let after = after.strip_prefix(':')?.trim_start();
    if after.starts_with("null") {
        return None;
    }
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

fn json_f32(body: &str, key: &str) -> Option<f32> {
    let pat = format!("\"{key}\"");
    let i = body.find(&pat)?;
    let after = body[i + pat.len()..].trim_start().strip_prefix(':')?.trim_start();
    let num: String = after
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-' )
        .collect();
    num.parse().ok()
}

fn guess_affect(text: &str) -> (f32, f32, f32, Option<String>) {
    crate::encode::affect::guess(text)
}
