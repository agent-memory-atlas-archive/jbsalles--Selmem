use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn new_id(prefix: &str) -> String {
    let n = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id() & 0xffff;
    format!("{prefix}_{pid:04x}_{n:08x}")
}

pub fn set_next_id(n: u64) {
    NEXT_ID.store(n.max(1), Ordering::Relaxed);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    Selfhood,
    World,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceStatus {
    Active,
    Cold,
    Myth,
    Sealed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DriftKind {
    Embellish,
    AmplifyDisgust,
    Fade,
    Merge,
    Weather,
    Rewrite,
    Reinterpret,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AxiomLayer {
    Motif,
    Belief,
    Trait,
}

#[derive(Clone, Debug)]
pub struct DriftEvent {
    pub kind: DriftKind,
    pub at: u64,
    pub note: String,
    pub fidelity_delta: f32,
    pub valence_delta: f32,
    pub disgust_delta: f32,
}

#[derive(Clone, Debug)]
pub struct MemoryTrace {
    pub id: String,
    pub gist: String,
    pub cues: Vec<String>,
    pub valence: f32,
    pub arousal: f32,
    pub disgust: f32,
    pub self_relevance: f32,
    pub schema: Option<String>,
    pub channel: Channel,
    pub archive_id: Option<String>,
    pub created_at: u64,
    pub last_recalled_at: Option<u64>,
    pub last_consolidated_at: Option<u64>,
    pub fidelity: f32,
    pub permanence: f32,
    pub rehearsals: u32,
    pub access: f32,
    pub status: TraceStatus,
    pub drifts: Vec<DriftEvent>,
    pub salience_at_encode: f32,
    pub embedding: Vec<f32>,
    /// Charge sémantique abstraite — ne s'efface pas avec Ebbinghaus.
    pub core: String,
    /// 0..1. Résistance à la dérive (trauma, réussite, valeur).
    pub anchor: f32,
}

impl MemoryTrace {
    pub fn clamp(&mut self) {
        self.valence = self.valence.clamp(-1.0, 1.0);
        self.arousal = self.arousal.clamp(0.0, 1.0);
        self.disgust = self.disgust.clamp(0.0, 1.0);
        self.self_relevance = self.self_relevance.clamp(0.0, 1.0);
        self.fidelity = self.fidelity.clamp(0.0, 1.0);
        self.permanence = self.permanence.clamp(0.0, 1.0);
        self.access = self.access.clamp(0.0, 1.0);
        self.anchor = self.anchor.clamp(0.0, 1.0);
    }
}

#[derive(Clone, Debug)]
pub struct ArchiveRecord {
    pub id: String,
    pub verbatim: String,
    pub source: String,
    pub created_at: u64,
}

#[derive(Clone, Debug)]
pub struct IdentityAxiom {
    pub id: String,
    pub statement: String,
    pub support_trace_ids: Vec<String>,
    pub valence: f32,
    pub strength: f32,
    pub created_at: u64,
    pub superseded_by: Option<String>,
    pub schema: Option<String>,
    pub layer: AxiomLayer,
}

#[derive(Clone, Debug)]
pub struct Mood {
    pub valence: f32,
    pub arousal: f32,
    pub disgust: f32,
}

impl Default for Mood {
    fn default() -> Self {
        Self {
            valence: 0.0,
            arousal: 0.2,
            disgust: 0.0,
        }
    }
}

impl Mood {
    pub fn blend(&mut self, other: &Mood, weight: f32) {
        let w = weight.clamp(0.0, 1.0);
        self.valence = (1.0 - w) * self.valence + w * other.valence;
        self.arousal = (1.0 - w) * self.arousal + w * other.arousal;
        self.disgust = (1.0 - w) * self.disgust + w * other.disgust;
    }
}

#[derive(Clone, Debug)]
pub struct RecalledMemory {
    pub trace_id: String,
    pub narrative: String,
    pub fidelity: f32,
    pub schema: Option<String>,
    pub channel: Channel,
    pub disclaimer: String,
}
