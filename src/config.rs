//! Runtime options. A `.selmem` file in the working directory, not the book.
//!
//! A vault starts with `SELMEM1`. A config file is `key=value` lines.
//! Precedence: CLI flag > `SELMEM_*` env > file > default.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static LOADED: OnceLock<Config> = OnceLock::new();

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub path: Option<PathBuf>,
    values: HashMap<String, String>,
}

impl Config {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn get() -> &'static Config {
        LOADED.get_or_init(Self::discover)
    }

    pub fn discover() -> Self {
        if let Some(p) = flag_from_args("--config").or_else(|| std::env::var("SELMEM_CONFIG").ok()) {
            return Self::load_path(Path::new(&p)).unwrap_or_default();
        }
        for candidate in [".selmem", "selmem.conf"] {
            let p = Path::new(candidate);
            if p.is_file() && !is_vault(p) {
                return Self::load_path(p).unwrap_or_default();
            }
        }
        Self::default()
    }

    pub fn load_path(path: &Path) -> std::io::Result<Self> {
        let raw = std::fs::read_to_string(path)?;
        let mut cfg = Self::parse(&raw);
        cfg.path = Some(path.to_path_buf());
        Ok(cfg)
    }

    pub fn parse(raw: &str) -> Self {
        let mut values = HashMap::new();
        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with("SELMEM1") {
                continue;
            }
            let line = line.strip_prefix("export ").unwrap_or(line).trim();
            let Some((k, v)) = line.split_once('=').or_else(|| line.split_once(':')) else {
                continue;
            };
            let key = normalize_key(k);
            if key.is_empty() {
                continue;
            }
            let val = unquote(v.trim());
            if !val.is_empty() {
                values.insert(key, val);
            }
        }
        Self {
            path: None,
            values,
        }
    }

    pub fn file(&self, key: &str) -> Option<&str> {
        self.values.get(&normalize_key(key)).map(String::as_str)
    }

    /// CLI, then env `SELMEM_*`, then the `.selmem` file.
    pub fn resolve(&self, cli: Option<String>, key: &str) -> Option<String> {
        if let Some(v) = cli.filter(|s| !s.is_empty()) {
            return Some(v);
        }
        let env_name = env_name(key);
        if let Ok(v) = std::env::var(&env_name) {
            if !v.is_empty() {
                return Some(v);
            }
        }
        self.file(key).map(str::to_string)
    }

    pub fn resolve_or(&self, cli: Option<String>, key: &str, default: &str) -> String {
        self.resolve(cli, key)
            .unwrap_or_else(|| default.to_string())
    }

    pub fn llm(&self) -> Option<String> {
        self.resolve(None, "llm")
    }

    pub fn model(&self, default: &str) -> String {
        self.resolve_or(None, "model", default)
    }

    pub fn api_key(&self) -> Option<String> {
        self.resolve(None, "api_key")
    }

    pub fn temp(&self) -> String {
        self.resolve_or(None, "temp", "0")
    }

    pub fn reasoning(&self) -> String {
        self.resolve_or(None, "reasoning", "none")
    }

    pub fn http_timeout(&self) -> String {
        self.resolve_or(None, "http_timeout", "60")
    }
}

fn normalize_key(raw: &str) -> String {
    let k = raw.trim().trim_start_matches("SELMEM_").to_ascii_lowercase();
    match k.as_str() {
        "key" | "apikey" => "api_key".into(),
        "timeout" => "http_timeout".into(),
        "embedmodel" | "embed_model" => "embed_model".into(),
        "ground-overlap" | "ground_overlap" => "ground_overlap".into(),
        "ground-strikes" | "ground_strikes" => "ground_strikes".into(),
        "narrator-firmness" | "narrator_firmness" => "narrator_firmness".into(),
        other => other.replace('-', "_"),
    }
}

fn env_name(key: &str) -> String {
    let k = normalize_key(key);
    let upper = match k.as_str() {
        "api_key" => "API_KEY",
        "http_timeout" => "HTTP_TIMEOUT",
        "embed_model" => "EMBED_MODEL",
        other => return format!("SELMEM_{}", other.to_ascii_uppercase()),
    };
    format!("SELMEM_{k}", k = upper)
}

fn unquote(v: &str) -> String {
    let v = v.trim();
    if v.len() >= 2 {
        let b = v.as_bytes();
        if (b[0] == b'"' && *b.last().unwrap() == b'"')
            || (b[0] == b'\'' && *b.last().unwrap() == b'\'')
        {
            return v[1..v.len() - 1].to_string();
        }
    }
    v.to_string()
}

fn is_vault(path: &Path) -> bool {
    std::fs::read_to_string(path)
        .map(|s| s.starts_with("SELMEM1"))
        .unwrap_or(false)
}

fn flag_from_args(name: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    args.windows(2).find_map(|w| {
        if w[0] == name {
            Some(w[1].clone())
        } else {
            None
        }
    })
}
