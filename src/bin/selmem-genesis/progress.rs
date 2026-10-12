//! Flushed stdout progress; never part of prompts, RNG or experimental state.
use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc, OnceLock,
    },
    thread,
    time::{Duration, Instant},
};
static ENABLED: AtomicBool = AtomicBool::new(false);
static START: OnceLock<Instant> = OnceLock::new();
static CALLS: AtomicU64 = AtomicU64::new(1);
pub(super) fn enable(enabled: bool) {
    ENABLED.store(enabled, Ordering::Relaxed);
}
pub(super) fn line(message: impl AsRef<str>) {
    if !ENABLED.load(Ordering::Relaxed) {
        return;
    }
    let elapsed = START.get_or_init(Instant::now).elapsed().as_secs();
    let mut stdout = io::stdout().lock();
    // Broken output pipes must not corrupt or abort scientific evidence writes.
    let _ = writeln!(
        stdout,
        "[GENESIS +{:02}:{:02}:{:02}] {}",
        elapsed / 3600,
        (elapsed / 60) % 60,
        elapsed % 60,
        message.as_ref()
    );
    let _ = stdout.flush();
}
pub(super) struct Call {
    label: String,
    started: Instant,
    stop: Option<mpsc::Sender<()>>,
    thread: Option<thread::JoinHandle<()>>,
    finished: bool,
}
impl Call {
    pub(super) fn start(label: impl Into<String>) -> Self {
        let mut call = Self {
            label: label.into(),
            started: Instant::now(),
            stop: None,
            thread: None,
            finished: false,
        };
        if ENABLED.load(Ordering::Relaxed) {
            let id = CALLS.fetch_add(1, Ordering::Relaxed);
            call.label = format!("operation #{id}: {}", call.label);
            line(format!("START {}", call.label));
            let (tx, rx) = mpsc::channel();
            let label = call.label.clone();
            let started = call.started;
            call.stop = Some(tx);
            call.thread = Some(thread::spawn(move || {
                while matches!(
                    rx.recv_timeout(Duration::from_secs(10)),
                    Err(mpsc::RecvTimeoutError::Timeout)
                ) {
                    line(format!(
                        "WAIT {label} — {:.0}s elapsed",
                        started.elapsed().as_secs_f64()
                    ));
                }
            }));
        }
        call
    }
    fn stop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
    pub(super) fn finish(mut self, success: bool) {
        self.stop();
        line(format!(
            "{} {} ({:.1}s)",
            if success { "DONE" } else { "FAILED" },
            self.label,
            self.started.elapsed().as_secs_f64()
        ));
        self.finished = true;
    }
}
impl Drop for Call {
    fn drop(&mut self) {
        self.stop();
        if !self.finished {
            line(format!(
                "FAILED {} ({:.1}s)",
                self.label,
                self.started.elapsed().as_secs_f64()
            ));
        }
    }
}
