//! One budget for every HTTP attempt, including scorer calls and retries.
use super::{
    json::{num, obj, text},
    runner::{log, Result},
};
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
struct Budget {
    out: PathBuf,
    limit: usize,
    used: usize,
}
thread_local! { static BUDGET: RefCell<Option<Budget>> = const { RefCell::new(None) }; }
pub(super) struct Session;
pub(super) fn begin(out: &Path, limit: usize) -> Session {
    BUDGET.with(|b| {
        *b.borrow_mut() = Some(Budget {
            out: out.into(),
            limit,
            used: 0,
        })
    });
    Session
}
impl Drop for Session {
    fn drop(&mut self) {
        BUDGET.with(|b| *b.borrow_mut() = None);
    }
}
pub(super) fn used() -> usize {
    BUDGET.with(|b| b.borrow().as_ref().map_or(0, |b| b.used))
}
pub(super) fn post(kind: &str, url: &str, key: Option<&str>, body: &str) -> Result<String> {
    for attempt in 1..=3 {
        let slot = BUDGET.with(|b| -> Result<_> {
            let mut b = b.borrow_mut();
            let Some(b) = b.as_mut() else { return Ok(None) };
            if b.used >= b.limit {
                return Err(format!(
                    "HTTP budget exhausted: {}/{} attempts",
                    b.used, b.limit
                ));
            }
            let id = b.used + 1;
            log(
                &b.out.join("http.jsonl"),
                &obj([
                    ("phase", text("start")),
                    ("id", num(id as f64)),
                    ("kind", text(kind)),
                    ("attempt", num(attempt as f64)),
                    ("endpoint", text(url)),
                    ("request_body", text(body)),
                ]),
            )?;
            b.used = id;
            Ok(Some((id, b.out.clone())))
        })?;
        if let Some((id, _)) = &slot {
            super::progress::line(format!("HTTP #{id} START {kind} attempt={attempt}"));
        }
        let started = Instant::now();
        let response = selmem::net::httpx::post_json(url, key, body);
        if let Some((id, out)) = &slot {
            log(
                &out.join("http.jsonl"),
                &obj([
                    ("phase", text("finish")),
                    ("id", num(*id as f64)),
                    ("kind", text(kind)),
                    ("seconds", num(started.elapsed().as_secs_f64())),
                    (
                        "response",
                        response
                            .as_ref()
                            .map(text)
                            .unwrap_or(super::json::Json::Null),
                    ),
                    (
                        "error",
                        response
                            .as_ref()
                            .err()
                            .map(text)
                            .unwrap_or(super::json::Json::Null),
                    ),
                ]),
            )?;
            super::progress::line(format!(
                "HTTP #{id} {} {kind}",
                if response.is_ok() { "DONE" } else { "FAILED" }
            ));
        }
        let transient = response.as_ref().err().is_some_and(|e| transient_error(e));
        let remaining = BUDGET.with(|b| b.borrow().as_ref().is_some_and(|b| b.used < b.limit));
        if transient && remaining && attempt < 3 {
            let wait = 30 * attempt;
            super::progress::line(format!("HTTP retry {kind} in {wait}s"));
            std::thread::sleep(Duration::from_secs(wait));
        } else {
            return response;
        }
    }
    unreachable!()
}
fn transient_error(e:&str)->bool {
    e.starts_with("curl: (28)") || ["HTTP 429:","HTTP 502:","HTTP 503:","HTTP 504:"].iter().any(|prefix|e.starts_with(prefix))
}
#[cfg(test)]
mod tests {
    #[test] fn timeouts_retry_but_permanent_errors_do_not(){
        assert!(super::transient_error("curl: (28) Operation timed out"));
        assert!(super::transient_error("HTTP 429: capacity"));
        assert!(!super::transient_error("HTTP 401: invalid key"));
        assert!(!super::transient_error("HTTP 400: invalid request"));
    }
    use super::*;
    #[test]
    fn zero_budget_never_attempts_network() {
        let _session = begin(Path::new("unused"), 0);
        assert!(post("test", "http://127.0.0.1:1", None, "{}")
            .unwrap_err()
            .contains("budget exhausted"));
        assert_eq!(used(), 0);
    }
    #[test]
    fn failed_connection_is_recorded_and_counts_once() {
        let out = std::env::temp_dir().join(format!("genesis-budget-{}", std::process::id()));
        std::fs::create_dir_all(&out).unwrap();
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        drop(listener);
        let _session = begin(&out, 1);
        assert!(post("test", &url, None, "{}").is_err());
        assert_eq!(used(), 1);
        assert!(post("test", &url, None, "{}")
            .unwrap_err()
            .contains("budget exhausted"));
        let rows = std::fs::read_to_string(out.join("http.jsonl")).unwrap();
        assert_eq!(rows.lines().count(), 2);
        assert!(!rows.lines().last().unwrap().contains("\"error\":null"));
        std::fs::remove_dir_all(out).unwrap();
    }
}
