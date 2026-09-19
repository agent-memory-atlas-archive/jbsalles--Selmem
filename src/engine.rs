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
use crate::core::talk::WorkingTalk;
use crate::recall::narrator::{Narrator, RuleNarrator};
use crate::persist;
use crate::core::profile::EntityProfile;
use crate::recall;
use crate::core::store::MemoryStore;

pub struct SelectiveMemory {
    pub profile: EntityProfile,
    pub store: MemoryStore,
    pub mood: Mood,
    /// Live thread. Not a trace. Not persisted. Sleep commits then clears it.
    pub talk: WorkingTalk,
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
            talk: WorkingTalk::default(),
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
                talk: WorkingTalk::default(),
                path: Some(path),
                narrator: Box::new(RuleNarrator),
                embedder: Box::new(HashEmbedder),
            })
        } else {
            Ok(Self {
                profile,
                store: MemoryStore::new(),
                mood: Mood::default(),
                talk: WorkingTalk::default(),
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

    pub fn live_with(&mut self, input: EncodeInput<'_>) -> EncodeDecision {
        self.ingest(input, true)
    }

    /// Same gate as `live_with`. `hold` writes the live thread; sleep commit does not.
    ///
    /// Order is the organ: interpret → paint → maybe hear → split → gate →
    /// maybe accept_core → blend mood. `commit_talk` calls this with `hold = false`.
    fn ingest(&mut self, mut input: EncodeInput<'_>, hold: bool) -> EncodeDecision {
        let axioms: Vec<String> = self
            .store
            .living_axioms()
            .into_iter()
            .map(|ax| ax.statement.clone())
            .collect();
        encode::interpret(&mut input, &self.mood, &axioms, self.narrator.as_ref());
        encode::paint(&mut self.store, &self.mood, &mut input);
        if hold {
            self.talk.hear(input.event, input.schema.as_deref());
        }
        let valence = input.valence;
        let arousal = input.arousal;
        let disgust = input.disgust;
        let input_source = input.source;
        let event_owned = input.event.to_string();
        let proposed = encode::propose_split(&event_owned, self.narrator.as_ref());
        let decision = encode::encode_with_parts(
            &mut self.store,
            &self.profile,
            input,
            self.embedder.as_ref(),
            proposed.as_deref(),
        );
        encode::maybe_set_core(
            &mut self.store,
            self.narrator.as_ref(),
            &decision,
            input_source,
            &event_owned,
        );
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

    /// The sitting becomes hours, then the frame dies.
    ///
    /// Sleep right after a chat must not wipe the conversation: each recorded
    /// turn is one live event (affect → paint → gate). Topic-only is still
    /// not an episode — experiments live then sleep and must not grow extra traces.
    fn commit_talk(&mut self) {
        self.talk.refresh();
        let schema = self.talk.schema.clone();
        let turns = std::mem::take(&mut self.talk.turns);
        self.talk.clear();
        for t in turns {
            // The sitting is the other voice. Glueing the reply in made the
            // entity's words count as what happened to it.
            let event = t.user.trim();
            if event.is_empty() {
                continue;
            }
            let (v, a, d, guessed) = encode::affect::guess(event);
            let mut ev = EncodeInput::new(event);
            ev.source = "talk";
            ev.valence = v;
            ev.arousal = a;
            ev.disgust = d;
            ev.self_relevance = 0.35;
            ev.utility = 0.45;
            ev.permanence = 0.40;
            ev.schema = schema.clone().or(guessed);
            let _ = self.ingest(ev, false);
        }
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
        self.commit_talk();
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
        self.speak_inner(user, true)
    }

    /// Probe / unit test path. Does not read or write the live thread.
    pub fn speak_isolated(&mut self, user: &str) -> String {
        self.speak_inner(user, false)
    }

    pub fn clear_talk(&mut self) {
        self.talk.clear();
    }

    /// Chat only. Pin the sitting's content lines so sleep can clear the
    /// frame without dropping what was just said. Does not change the gate.
    pub fn keep_sitting(&mut self) -> (usize, Option<String>) {
        self.talk.refresh();
        let lines: Vec<String> = self
            .talk
            .turns
            .iter()
            .map(|t| t.user.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let cue = self.talk.topic.clone();
        let mut chunks: Vec<String> = Vec::new();
        for line in lines {
            let short = line.split_whitespace().count() <= 2;
            if short {
                if let Some(prev) = chunks.last_mut() {
                    prev.push('\n');
                    prev.push_str(&line);
                    continue;
                }
            }
            chunks.push(line);
        }
        let mut kept = 0usize;
        for chunk in &chunks {
            if chunk.split_whitespace().count() <= 2 {
                continue;
            }
            if self
                .store
                .archives
                .values()
                .any(|a| a.source == "talk" && a.verbatim == *chunk)
            {
                continue;
            }
            let (v, a, d, schema) = encode::affect::guess(chunk);
            let mut ev = EncodeInput::new(chunk);
            ev.source = "talk";
            ev.channel = crate::core::model::Channel::World;
            ev.valence = v;
            ev.arousal = a;
            ev.disgust = d;
            ev.self_relevance = 0.55;
            ev.utility = 0.55;
            ev.permanence = 0.35;
            ev.schema = schema;
            if self.ingest(ev, false).kept {
                kept += 1;
            }
        }
        (kept, cue)
    }

    /// Sitting hours are not vows. Each chat-night they lose a little;
    /// after about 20, recall no longer finds them.
    pub fn fade_sitting(&mut self) {
        let talk_ids: Vec<String> = self
            .store
            .traces
            .values()
            .filter(|t| {
                t.permanence < 0.80
                    && t.archive_id
                        .as_ref()
                        .and_then(|id| self.store.archives.get(id))
                        .map(|a| a.source == "talk")
                        .unwrap_or(false)
            })
            .map(|t| t.id.clone())
            .collect();
        for id in talk_ids {
            let Some(t) = self.store.traces.get_mut(&id) else {
                continue;
            };
            t.fidelity = (t.fidelity - 0.04).max(0.15);
            t.access = (t.access - 0.05).max(0.0);
            t.permanence = (t.permanence - 0.01).max(0.0);
        }
    }

    /// Empty the book, the seal, axioms, mood and the sitting. Profile stays.
    pub fn reset(&mut self) {
        self.store = MemoryStore::new();
        self.mood = Mood::default();
        self.talk.clear();
    }

    /// Drop the frame if the conversation went idle (10 min) or hit 2 h.
    pub fn refresh_talk(&mut self) {
        self.talk.refresh();
    }

    fn speak_inner(&mut self, user: &str, hold: bool) -> String {
        if hold {
            self.talk.hear(user, None);
        }
        let query = if hold {
            self.talk.recall_query(user)
        } else {
            user.to_string()
        };
        let recalled = self.remember(&query);
        let empty = WorkingTalk::default();
        let talk = if hold { &self.talk } else { &empty };
        // Sitting: answer the human. The book stays a book — sleep, /who,
        // isolated probes. A thin greeting must not recite axioms.
        let (memories, axioms) = if hold {
            let memories: Vec<String> = recalled
                .into_iter()
                .take(1)
                .map(|r| r.narrative)
                .collect();
            (
                memories,
                vec![format!("Your name is {}.", self.profile.name)],
            )
        } else {
            (
                recalled.into_iter().map(|r| r.narrative).collect(),
                self.who_am_i()
                    .into_iter()
                    .map(|a| a.statement.clone())
                    .collect(),
            )
        };
        let reply = self
            .narrator
            .reply(user, &memories, &axioms, &self.mood, talk);
        if hold {
            self.talk.record(user, &reply);
        }
        reply
    }

    pub fn audit(&self, trace_id: &str) -> Option<&str> {
        let trace = self.store.traces.get(trace_id)?;
        let aid = trace.archive_id.as_ref()?;
        self.store.archives.get(aid).map(|a| a.verbatim.as_str())
    }

    /// Keep this hour. Does not open the archive. Next nights decay it more slowly.
    pub fn pin(&mut self, trace_id: &str) -> bool {
        let Some(t) = self.store.traces.get_mut(trace_id) else {
            return false;
        };
        t.permanence = t.permanence.max(0.92);
        t.self_relevance = t.self_relevance.max(0.9);
        t.anchor = t.anchor.max(0.85);
        t.status = crate::core::model::TraceStatus::Active;
        t.access = t.access.max(0.7);
        true
    }
}

fn is_sqlite(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("db") | Some("sqlite") | Some("sqlite3")
    )
}
