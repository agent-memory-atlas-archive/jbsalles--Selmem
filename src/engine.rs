//! The organ the rest of the crate talks to.
//!
//! Typical loop:
//! `live_with` (encode) → `remember` / `speak` (reconstruct) → `sleep` (consolidate).
//! Persistence (`save` / `open`) is a vault. The narrator never opens it.

use std::path::{Path, PathBuf};

use crate::dream::{self, DreamReport};
use crate::encode::embed::{Embedder, HashEmbedder};
use crate::encode::{self, EncodeDecision, EncodeInput};
use crate::core::model::{IdentityAxiom, Mood, RecalledMemory};
use crate::recall::narrator::{Narrator, RuleNarrator};
use crate::persist;
use crate::core::profile::EntityProfile;
use crate::recall;
use crate::core::store::MemoryStore;

pub struct SelectiveMemory {
    pub profile: EntityProfile,
    pub store: MemoryStore,
    pub mood: Mood,
    pub path: Option<PathBuf>,
    narrator: Box<dyn Narrator>,
    embedder: Box<dyn Embedder>,
}

impl SelectiveMemory {
    pub fn new(profile: EntityProfile) -> Self {
        Self {
            profile,
            store: MemoryStore::new(),
            mood: Mood::default(),
            path: None,
            narrator: Box::new(RuleNarrator),
            embedder: Box::new(HashEmbedder),
        }
    }

    pub fn open(path: impl AsRef<Path>, profile: EntityProfile) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        if path.exists() {
            let snap = if is_sqlite(&path) {
                crate::persist::sqlite::load(&path)?
            } else {
                persist::load(&path)?
            };
            Ok(Self {
                profile: snap.profile,
                store: snap.store,
                mood: snap.mood,
                path: Some(path),
                narrator: Box::new(RuleNarrator),
                embedder: Box::new(HashEmbedder),
            })
        } else {
            Ok(Self {
                profile,
                store: MemoryStore::new(),
                mood: Mood::default(),
                path: Some(path),
                narrator: Box::new(RuleNarrator),
                embedder: Box::new(HashEmbedder),
            })
        }
    }

    pub fn with_narrator(mut self, narrator: Box<dyn Narrator>) -> Self {
        self.narrator = narrator;
        self
    }

    pub fn with_embedder(mut self, embedder: Box<dyn Embedder>) -> Self {
        self.embedder = embedder;
        self
    }

    pub fn save(&self) -> std::io::Result<()> {
        let Some(path) = &self.path else {
            return Ok(());
        };
        if is_sqlite(path) {
            crate::persist::sqlite::save(path, &self.profile, &self.mood, &self.store)
        } else {
            persist::save(path, &self.profile, &self.mood, &self.store)
        }
    }

    pub fn live(&mut self, event: &str) -> EncodeDecision {
        self.live_with(EncodeInput::new(event))
    }

    pub fn live_with(&mut self, mut input: EncodeInput<'_>) -> EncodeDecision {
        let uninterpreted =
            input.schema.is_none() && input.valence.abs() < 0.08 && input.disgust < 0.08;
        if uninterpreted {
            let (v, a, d, s) = encode::affect::guess(input.event);
            input.valence = v;
            input.arousal = a;
            input.disgust = d;
            input.schema = s;
            let axioms: Vec<String> = self
                .store
                .living_axioms()
                .into_iter()
                .map(|ax| ax.statement.clone())
                .collect();
            if let Some(interp) = self.narrator.interpret(input.event, &self.mood, &axioms) {
                input.valence = interp.valence;
                input.arousal = interp.arousal;
                input.disgust = interp.disgust;
                if input.schema.is_none() {
                    input.schema = interp.schema;
                }
                input.self_relevance = input.self_relevance.max(interp.self_relevance);
            }
        }
        encode::identity::paint(&self.store, &self.mood, &mut input);
        let valence = input.valence;
        let arousal = input.arousal;
        let disgust = input.disgust;
        let decision = encode::encode(&mut self.store, &self.profile, input, self.embedder.as_ref());
        if decision.kept {
            self.mood.blend(
                &Mood {
                    valence,
                    arousal,
                    disgust,
                },
                self.profile.mood_blend,
            );
        }
        decision
    }

    pub fn remember(&mut self, query: &str) -> Vec<RecalledMemory> {
        let recalled = recall::recall(
            &mut self.store,
            &self.profile,
            self.narrator.as_ref(),
            self.embedder.as_ref(),
            query,
            &self.mood,
        );
        if !recalled.is_empty() {
            let mut v = 0.0;
            let mut a = 0.0;
            let mut d = 0.0;
            let n = recalled.len() as f32;
            for r in &recalled {
                if let Some(t) = self.store.traces.get(&r.trace_id) {
                    v += t.valence;
                    a += t.arousal;
                    d += t.disgust;
                }
            }
            self.mood.blend(
                &Mood {
                    valence: v / n,
                    arousal: a / n,
                    disgust: d / n,
                },
                self.profile.mood_blend * 1.4,
            );
        }
        recalled
    }

    pub fn sleep(&mut self) -> DreamReport {
        crate::persist::prune_orphaned_archives(&mut self.store);
        dream::dream(
            &mut self.store,
            &self.profile,
            self.narrator.as_ref(),
            self.embedder.as_ref(),
        )
    }

    pub fn lineage(&self, schema: &str) -> Vec<&IdentityAxiom> {
        let mut xs: Vec<_> = self
            .store
            .axioms
            .values()
            .filter(|a| a.schema.as_deref() == Some(schema))
            .collect();
        xs.sort_by_key(|a| a.created_at);
        xs
    }

    pub fn who_am_i(&self) -> Vec<&IdentityAxiom> {
        let mut axioms = self.store.living_axioms();
        axioms.sort_by(|a, b| {
            let rank = |l| match l {
                crate::core::model::AxiomLayer::Trait => 2,
                crate::core::model::AxiomLayer::Belief => 1,
                crate::core::model::AxiomLayer::Motif => 0,
            };
            rank(b.layer)
                .cmp(&rank(a.layer))
                .then(
                    b.strength
                        .partial_cmp(&a.strength)
                        .unwrap_or(std::cmp::Ordering::Equal),
                )
        });
        axioms
    }

    pub fn speak(&mut self, user: &str) -> String {
        let recalled = self.remember(user);
        let memories: Vec<String> = recalled.into_iter().map(|r| r.narrative).collect();
        let axioms: Vec<String> = self
            .who_am_i()
            .into_iter()
            .map(|a| a.statement.clone())
            .collect();
        self.narrator
            .reply(user, &memories, &axioms, &self.mood)
    }

    pub fn audit(&self, trace_id: &str) -> Option<&str> {
        let trace = self.store.traces.get(trace_id)?;
        let aid = trace.archive_id.as_ref()?;
        self.store.archives.get(aid).map(|a| a.verbatim.as_str())
    }
}

fn is_sqlite(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("db") | Some("sqlite") | Some("sqlite3")
    )
}
