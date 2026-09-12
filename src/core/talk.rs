//! Working conversation frame. Not the lived book.
//!
//! The LLM call is stateless. This frame is what lets the next turn
//! still know what the last turns were about. Sleep does not clear it.
//! Persist does not write it: a session, not a sculpture.

use crate::encode::scoring::{lexical_similarity, token_set};

const MAX_TURNS: usize = 6;
const MAX_LINE: usize = 220;

const LIGHT: &[&str] = &[
    "alors", "quoi", "donc", "mais", "comme", "cette", "cette", "tres",
    "très", "plus", "dans", "pour", "avec", "sans", "comment", "pourquoi",
    "quand", "quoi", "oui", "non", "bien", "tout", "rien", "cela", "cette",
    "what", "about", "think", "that", "this", "when", "just", "very", "well",
    "how", "why", "and", "the", "you", "toi", "tu", "te", "ton", "tes",
    "mon", "mes", "une", "des", "les", "est", "suis", "pas", "plus",
    "pense", "penses", "dis", "dit", "vois", "voir", "encore", "toujours",
];

#[derive(Clone, Debug)]
pub struct TalkTurn {
    pub user: String,
    pub reply: String,
}

#[derive(Clone, Debug, Default)]
pub struct WorkingTalk {
    pub topic: Option<String>,
    pub schema: Option<String>,
    pub turns: Vec<TalkTurn>,
}

impl WorkingTalk {
    pub fn is_empty(&self) -> bool {
        self.topic.is_none() && self.schema.is_none() && self.turns.is_empty()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Cue mixed into recall so "et alors ?" can still find the thread.
    pub fn recall_query(&self, user: &str) -> String {
        match self.topic.as_deref() {
            Some(t) if !t.is_empty() => format!("{t} {user}"),
            _ => user.to_string(),
        }
    }

    pub fn hear(&mut self, user: &str, schema: Option<&str>) {
        if let Some(s) = schema.map(str::trim).filter(|s| !s.is_empty() && *s != "-") {
            self.schema = Some(s.to_string());
            if self.topic.is_none() {
                self.topic = Some(s.to_string());
            }
        }
        let content = content_tokens(user);
        if content.len() < 2 {
            return;
        }
        let candidate = content.into_iter().take(8).collect::<Vec<_>>().join(" ");
        match self.topic.as_deref() {
            None => self.topic = Some(candidate),
            Some(t) if lexical_similarity(user, t) < 0.10 => {
                self.topic = Some(candidate);
            }
            Some(_) => {}
        }
    }

    pub fn record(&mut self, user: &str, reply: &str) {
        self.turns.push(TalkTurn {
            user: clip(user, MAX_LINE),
            reply: clip(reply, MAX_LINE),
        });
        if self.turns.len() > MAX_TURNS {
            let drop = self.turns.len() - MAX_TURNS;
            self.turns.drain(0..drop);
        }
    }

    pub fn render(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        let mut out = String::new();
        if let Some(t) = self.topic.as_deref() {
            out.push_str("fil: ");
            out.push_str(t);
            out.push('\n');
        }
        if let Some(s) = self.schema.as_deref() {
            out.push_str("schema du fil: ");
            out.push_str(s);
            out.push('\n');
        }
        if !self.turns.is_empty() {
            out.push_str("conversation en cours:\n");
            for t in &self.turns {
                out.push_str("- humain: ");
                out.push_str(&t.user);
                out.push('\n');
                out.push_str("- soi: ");
                out.push_str(&t.reply);
                out.push('\n');
            }
        }
        out
    }
}

fn clip(s: &str, n: usize) -> String {
    let t = s.trim();
    if t.chars().count() <= n {
        t.to_string()
    } else {
        t.chars().take(n).collect()
    }
}

fn content_tokens(text: &str) -> Vec<String> {
    token_set(text)
        .into_iter()
        .filter(|w| !LIGHT.contains(&w.as_str()))
        .collect()
}
