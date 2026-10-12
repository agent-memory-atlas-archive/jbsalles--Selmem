use super::{
    json::{self, obj, text, Json},
    runner::*,
};
use std::{
    io::{Read, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};
pub(super) struct Server {
    url: String,
    requests: Arc<Mutex<Vec<Json>>>,
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}
impl Server {
    pub(super) fn new(malformed: bool) -> Self {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!(
            "http://{}/v1/chat/completions",
            listener.local_addr().unwrap()
        );
        listener.set_nonblocking(true).unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let records = requests.clone();
        let stop = Arc::new(AtomicBool::new(false));
        let stopped = stop.clone();
        let thread = std::thread::spawn(move || {
            while !stopped.load(Ordering::Relaxed) {
                let (mut socket, _) = match listener.accept() {
                    Ok(s) => s,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(std::time::Duration::from_millis(2));
                        continue;
                    }
                    Err(e) => panic!("{e}"),
                };
                socket
                    .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                    .unwrap();
                let mut bytes = Vec::new();
                let mut buffer = [0; 4096];
                let body = loop {
                    let n = socket.read(&mut buffer).unwrap();
                    assert!(n > 0);
                    bytes.extend_from_slice(&buffer[..n]);
                    if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                        let header = String::from_utf8_lossy(&bytes[..end]);
                        let length: usize = header
                            .lines()
                            .find_map(|l| {
                                let (k, v) = l.split_once(':')?;
                                k.eq_ignore_ascii_case("content-length")
                                    .then(|| v.trim().parse().unwrap())
                            })
                            .unwrap();
                        if bytes.len() >= end + 4 + length {
                            break json::parse(
                                std::str::from_utf8(&bytes[end + 4..end + 4 + length]).unwrap(),
                            )
                            .unwrap();
                        }
                    }
                };
                let messages = array(&body, "messages").unwrap();
                let system = s(&messages[0], "content").unwrap();
                let user = s(&messages[1], "content").unwrap();
                let answer = if s(&body, "model").unwrap() == "decision" {
                    let prompt = json::parse(user).unwrap();
                    let mut facts = s(&prompt, "situation").unwrap().to_string();
                    for m in array(prompt.get("memory").unwrap(), "memories").unwrap() {
                        facts.push_str(m.string().unwrap());
                    }
                    let options = array(&prompt, "options").unwrap();
                    let chosen = options
                        .iter()
                        .find(|o| {
                            facts.contains(&format!("requires machine {}.", s(o, "text").unwrap()))
                        })
                        .unwrap_or(&options[0]);
                    obj([("choice", chosen.get("id").unwrap().clone())]).encode()
                } else if system.starts_with("Describe only") {
                    if malformed {
                        "valence=nan".into()
                    } else {
                        let words: Vec<_> = user.split_whitespace().collect();
                        format!("entities={}|{}; actions=requires; event_type=none; bearer=self; loss=none; stake_mark=workshop; absence=none; valence=0; arousal=0.2; disgust=0; self_relevance=0.9; goal_relevance=0.9; agency=none; schema=workshop rule; confidence=0.9.",words[2],words[5].trim_end_matches('.'))
                    }
                } else if s(&body, "model").unwrap() == "scorer" {
                    "entail".into()
                } else {
                    user.lines()
                        .find_map(|l| l.strip_prefix("gist: "))
                        .unwrap_or("Workshop rule: ulen requires machine kavi.")
                        .to_string()
                };
                let response = obj([
                    ("model", body.get("model").unwrap().clone()),
                    (
                        "choices",
                        arr([obj([("message", obj([("content", text(answer))]))])]),
                    ),
                ])
                .encode();
                records.lock().unwrap().push(body);
                write!(socket,"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",response.len(),response).unwrap();
            }
        });
        Self {
            url,
            requests,
            stop,
            thread: Some(thread),
        }
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        self.thread.take().unwrap().join().unwrap();
    }
}
pub(super) fn config(server: &Server) -> Json {
    obj([
        ("memory_backend", text("http")),
        ("lifecycle_backend", text("http")),
        ("memory_endpoint", text(&server.url)),
        ("memory_model", text("narrator")),
        ("scorer_endpoint", text(&server.url)),
        ("scorer_model", text("scorer")),
        ("reconstruction_policy", text("native")),
        ("associations_learn", Json::Bool(true)),
        ("associations_reconstruct", Json::Bool(true)),
    ])
}
pub(super) fn root() -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "genesis-lifecycle-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}
#[test]
fn live_sleep_and_wrapper_use_http_with_auditable_annotations() {
    let server = Server::new(false);
    let dir = root();
    let cfg = config(&server);
    let _session = super::transport::begin(&dir, 40);
    let m = Memory::new("selmem_full", dir.join("memory.vault"), &cfg);
    m.init().unwrap();
    let event = obj([
        ("id", text("rule")),
        ("text", text("Workshop rule: ulen requires machine kavi.")),
        ("critical", Json::Bool(true)),
    ]);
    let result = m.call("live", obj([("event", event)])).unwrap();
    assert!(flag(&result, "kept"));
    let audit = result.get("lifecycle_audit").unwrap();
    assert_eq!(s(audit, "bound_backend").unwrap(), "http");
    assert_eq!(array(audit, "semantic_calls").unwrap().len(), 1);
    let organ =
        selmem::SelectiveMemory::open(&m.path, selmem::EntityProfile::new("observer")).unwrap();
    let (bound, audit) = super::http::bind(organ, &cfg).unwrap();
    let trace = bound.store.traces.values().next().unwrap();
    assert_eq!(trace.core, "Workshop rule: ulen requires machine kavi.");
    assert_eq!(trace.schema.as_deref(), Some("workshop rule"));
    let _ = bound.narrator_arc().rewrite(trace, &[], &bound.profile);
    assert!(!audit.lock().unwrap().narrator.lock().unwrap().is_empty());
    let result = m.call("sleep", obj([])).unwrap();
    assert_eq!(
        s(result.get("lifecycle_audit").unwrap(), "bound_backend").unwrap(),
        "http"
    );
    assert!(server
        .requests
        .lock()
        .unwrap()
        .iter()
        .all(|r| s(r, "model").unwrap() == "narrator" || s(r, "model").unwrap() == "scorer"));
    std::fs::remove_dir_all(dir).unwrap();
}
#[test]
fn invalid_semantics_aborts_without_writing_the_snapshot() {
    let server = Server::new(true);
    let dir = root();
    let cfg = config(&server);
    let _session = super::transport::begin(&dir, 40);
    let m = Memory::new("selmem_full", dir.join("memory.vault"), &cfg);
    m.init().unwrap();
    let before = file_hash(&m.path).unwrap();
    let event = obj([
        ("id", text("rule")),
        ("text", text("Workshop rule: ulen requires machine kavi.")),
        ("critical", Json::Bool(true)),
    ]);
    assert!(m
        .call("live", obj([("event", event)]))
        .unwrap_err()
        .contains("lifecycle backend failed"));
    assert_eq!(file_hash(&m.path).unwrap(), before);
    assert_eq!(super::transport::used(), 1);
    std::fs::remove_dir_all(dir).unwrap();
}
