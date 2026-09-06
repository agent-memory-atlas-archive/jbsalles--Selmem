use crate::net::httpx::{extract_json_array_f32, json_esc, post_json};
use crate::encode::scoring::token_set;

pub const EMBED_DIM: usize = 96;

pub trait Embedder: Send + Sync {
    fn embed(&self, text: &str) -> Vec<f32>;
}

#[derive(Default, Clone)]
pub struct HashEmbedder;

pub struct HttpEmbedder {
    pub url: String,
    pub model: String,
    pub api_key: Option<String>,
    fallback: HashEmbedder,
}

impl HttpEmbedder {
    pub fn parse(endpoint: &str, model: impl Into<String>, api_key: Option<String>) -> Option<Self> {
        if !(endpoint.starts_with("http://") || endpoint.starts_with("https://")) {
            return None;
        }
        Some(Self {
            url: endpoint.to_string(),
            model: model.into(),
            api_key,
            fallback: HashEmbedder,
        })
    }
}

impl Embedder for HttpEmbedder {
    fn embed(&self, text: &str) -> Vec<f32> {
        let body = format!(
            "{{\"model\":\"{}\",\"input\":\"{}\"}}",
            json_esc(&self.model),
            json_esc(text)
        );
        match post_json(&self.url, self.api_key.as_deref(), &body) {
            Ok(raw) => extract_json_array_f32(&raw, "embedding").unwrap_or_else(|| self.fallback.embed(text)),
            Err(_) => self.fallback.embed(text),
        }
    }
}

impl Embedder for HashEmbedder {
    fn embed(&self, text: &str) -> Vec<f32> {
        let mut v = vec![0.0f32; EMBED_DIM];
        let lower = text.to_lowercase();
        let chars: Vec<char> = lower.chars().filter(|c| c.is_alphanumeric()).collect();
        for n in 2..=3 {
            if chars.len() >= n {
                for w in chars.windows(n) {
                    let s: String = w.iter().collect();
                    bump(&mut v, &s, 1.0);
                }
            }
        }
        for tok in token_set(&lower) {
            bump(&mut v, &tok, 1.4);
        }
        l2_normalize(&mut v);
        v
    }
}

fn bump(v: &mut [f32], s: &str, w: f32) {
    let h = fnv(s);
    let i = (h as usize) % v.len();
    v[i] += w;
    let j = ((h >> 17) as usize) % v.len();
    if j != i {
        v[j] += w * 0.5;
    }
}

fn fnv(s: &str) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

pub fn l2_normalize(v: &mut [f32]) {
    let n = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if n > 1e-8 {
        for x in v.iter_mut() {
            *x /= n;
        }
    }
}

pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || b.is_empty() || a.len() != b.len() {
        return 0.0;
    }
    let mut dot = 0.0;
    let mut na = 0.0;
    let mut nb = 0.0;
    for i in 0..a.len() {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }
    let d = na.sqrt() * nb.sqrt();
    if d < 1e-8 {
        0.0
    } else {
        (dot / d).clamp(-1.0, 1.0)
    }
}

pub fn novelty_emb(query: &[f32], existing: &[Vec<f32>]) -> f32 {
    if existing.is_empty() {
        return 1.0;
    }
    let nearest = existing
        .iter()
        .map(|e| cosine(query, e))
        .fold(0.0_f32, f32::max);
    (1.0 - nearest).clamp(0.0, 1.0)
}
