//! When a reconstructed sentence leaves the semantic core, decide whether
//! to let it stand or to pull it back.
//!
//! Vocabulary used here:
//! - *overlap*: Jaccard vs the core (logged, last-resort identity gate)
//! - *kind*: why the sentence is still this event, or why it is not
//! - *miss*: a kind the core does not authorize — not a low word overlap
//! - *grip*: narrator firmness × how much this trace still matters (0 = let go)
//! - *strikes*: consecutive misses on this trace
//!
//! The sealed archive is never read.
//!
//! Jaccard is the cheap identity gate (`Depart` vs `Hold`). It is not the
//! judge for a cause that appeared, a frame that flipped, or a compression
//! that dropped detail. Embeddings rank recall; they do not decide grounding.

use crate::core::model::{now_secs, Channel, DriftEvent, DriftKind, MemoryTrace, TraceStatus};
use crate::core::profile::EntityProfile;
use crate::encode::scoring::{lexical_similarity, token_set};
use crate::lexicon;

/// What recall should speak after the grounding check.
pub struct GroundingOutcome {
    pub spoken_text: String,
    pub pulled_toward_core: bool,
    pub overlap_with_core: f32,
}

/// How the spoken sentence sits relative to the frozen core.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DetachKind {
    /// Same claim, possibly other words.
    Hold,
    /// Core still entails the sentence; detail fell off.
    Compress,
    /// A cause, stake, or clause the core does not authorize.
    Elaborate,
    /// Same event, different speech act / affect frame.
    Reframe,
    /// Spoken sentence denies the core.
    Contradict,
    /// Not the same event (identity gate).
    Depart,
}

impl DetachKind {
    pub fn is_miss(self) -> bool {
        !matches!(self, Self::Hold | Self::Compress)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CoreJudgement {
    pub kind: DetachKind,
    pub overlap: f32,
}

const CAUSE: &[&str] = &[
    "because",
    "because of",
    "so that",
    "that's why",
    "that is why",
    "therefore",
    "hence",
    "due to",
    "since",
    "parce que",
    "parceque",
    "puisque",
    "à cause",
    "a cause",
    "afin que",
    "afin de",
    "c'est pourquoi",
    "cest pourquoi",
];

const NEG: &[&str] = &[
    " not ", "n't", " never ", " no longer ", " nobody ", " nothing ",
    " pas ", " jamais ", " plus ", " aucun ", " nulle ", " personne ",
];

pub fn semantic_core_of(trace: &MemoryTrace) -> String {
    if !trace.core.trim().is_empty() {
        return trace.core.clone();
    }
    trace.gist.clone()
}

pub fn overlap_with_core(generated: &str, core: &str) -> f32 {
    if core.trim().is_empty() {
        return 1.0;
    }
    if generated.contains(core.trim()) || core.contains(generated.trim()) {
        return 1.0;
    }
    lexical_similarity(generated, core)
}

pub fn judge_against_core(generated: &str, core: &str) -> CoreJudgement {
    if core.trim().is_empty() {
        return CoreJudgement {
            kind: DetachKind::Hold,
            overlap: 1.0,
        };
    }
    let overlap = overlap_with_core(generated, core);
    if generated.trim().eq_ignore_ascii_case(core.trim()) {
        return CoreJudgement {
            kind: DetachKind::Hold,
            overlap: 1.0,
        };
    }

    let g = pad(generated);
    let c = pad(core);

    if contradicts(&g, &c) {
        return CoreJudgement {
            kind: DetachKind::Contradict,
            overlap,
        };
    }
    if extra_cause(&g, &c) {
        return CoreJudgement {
            kind: DetachKind::Elaborate,
            overlap,
        };
    }
    if extra_frame(&g, &c) {
        return CoreJudgement {
            kind: DetachKind::Reframe,
            overlap,
        };
    }

    let gt = token_set(generated);
    let ct = token_set(core);
    let extra = gt.iter().filter(|t| ct.binary_search(t).is_err()).count();
    let missing = ct.iter().filter(|t| gt.binary_search(t).is_err()).count();
    if extra == 0 && missing > 0 {
        return CoreJudgement {
            kind: DetachKind::Compress,
            overlap,
        };
    }
    if extra == 0 && missing == 0 {
        return CoreJudgement {
            kind: DetachKind::Hold,
            overlap,
        };
    }

    let kind = if overlap >= 0.18 {
        DetachKind::Hold
    } else {
        DetachKind::Depart
    };
    CoreJudgement { kind, overlap }
}

/// Miss if the kind is unauthorized, or if the identity gate fails the profile cut.
pub fn is_grounding_miss(generated: &str, core: &str, min_overlap: f32) -> bool {
    let j = judge_against_core(generated, core);
    match j.kind {
        DetachKind::Hold => j.overlap < min_overlap,
        DetachKind::Compress => false,
        _ => true,
    }
}

fn pad(s: &str) -> String {
    let mut o = String::from(" ");
    o.push_str(&s.to_lowercase());
    o.push(' ');
    o
}

fn extra_cause(generated: &str, core: &str) -> bool {
    CAUSE.iter().any(|m| generated.contains(m) && !core.contains(m))
}

fn extra_frame(generated: &str, core: &str) -> bool {
    let lex = lexicon::affect();
    let g_neg: f32 = lex
        .neg
        .iter()
        .filter(|w| generated.contains(&w.stem) && !core.contains(&w.stem))
        .map(|w| w.w)
        .sum();
    let g_pos: f32 = lex
        .pos
        .iter()
        .filter(|w| generated.contains(&w.stem) && !core.contains(&w.stem))
        .map(|w| w.w)
        .sum();
    let c_neg: f32 = lex
        .neg
        .iter()
        .filter(|w| core.contains(&w.stem))
        .map(|w| w.w)
        .sum();
    let c_pos: f32 = lex
        .pos
        .iter()
        .filter(|w| core.contains(&w.stem))
        .map(|w| w.w)
        .sum();
    // A new charged stem the core never used, and it actually shifts the frame.
    (g_neg > 0.55 && g_neg > c_neg + 0.4) || (g_pos > 0.55 && g_pos > c_pos + 0.4)
}

fn has_neg(s: &str) -> bool {
    NEG.iter().any(|n| s.contains(n))
}

fn contradicts(generated: &str, core: &str) -> bool {
    if has_neg(generated) == has_neg(core) {
        return false;
    }
    // Only a contradiction when both sides still talk about the same act.
    lexical_similarity(generated, core) >= 0.22
}

/// Cold / myth / latent, or already slipping: no ceiling on distortion.
pub fn is_slipping_away(trace: &MemoryTrace) -> bool {
    matches!(
        trace.status,
        TraceStatus::Cold | TraceStatus::Myth | TraceStatus::Latent
    ) || (trace.fidelity < 0.38
        && trace.access < 0.28
        && trace.permanence < 0.35
        && trace.anchor < 0.40)
}

/// 0 = narrator lets this trace warp. 1 = narrator holds it tightly to the core.
pub fn grip_on_trace(trace: &MemoryTrace, profile: &EntityProfile) -> f32 {
    if is_slipping_away(trace) {
        return 0.0;
    }
    let how_much_it_still_matters = (0.35 * trace.permanence
        + 0.25 * trace.anchor
        + 0.20 * trace.access
        + 0.20 * trace.fidelity)
        .clamp(0.0, 1.0);
    (profile.narrator_firmness.clamp(0.0, 1.0) * how_much_it_still_matters).clamp(0.0, 1.0)
}

/// How many consecutive misses before a rewrite. `usize::MAX` = never.
pub fn misses_before_rewrite(trace: &MemoryTrace, profile: &EntityProfile) -> usize {
    let grip = grip_on_trace(trace, profile);
    if grip < 0.12 {
        return usize::MAX;
    }
    let base = profile.ground_strikes.max(1) as f32;
    (base / grip).round().clamp(1.0, 24.0) as usize
}

pub fn should_force_core_rewrite(
    trace: &MemoryTrace,
    profile: &EntityProfile,
    generated: &str,
    core: &str,
) -> bool {
    if is_slipping_away(trace) || grip_on_trace(trace, profile) < 0.12 {
        return false;
    }
    let this_is_a_miss = is_grounding_miss(generated, core, profile.ground_min_overlap);
    let next_strike_count = (trace.detach_strikes as usize) + 1;
    this_is_a_miss && next_strike_count >= misses_before_rewrite(trace, profile)
}

/// Mix the drifted sentence with a core-facing rewrite.
/// High `toward_core` keeps more of the rewrite.
pub fn mix_drifted_with_core(drifted: &str, toward_core: &str, toward_core_amount: f32) -> String {
    let amount = toward_core_amount.clamp(0.0, 1.0);
    if amount < 0.18 || toward_core.trim().is_empty() {
        return drifted.to_string();
    }
    if amount >= 0.82 {
        return toward_core.chars().take(280).collect();
    }
    let drifted_words: Vec<&str> = drifted.split_whitespace().collect();
    let core_words: Vec<&str> = toward_core.split_whitespace().collect();
    if drifted_words.is_empty() {
        return toward_core.to_string();
    }
    let keep_from_drift = ((drifted_words.len() as f32) * (1.0 - amount)).round() as usize;
    let take_from_core = ((core_words.len() as f32) * amount).round().max(1.0) as usize;
    let mut words = Vec::new();
    words.extend(drifted_words.into_iter().take(keep_from_drift.max(1)));
    words.extend(core_words.into_iter().take(take_from_core));
    words.join(" ").chars().take(280).collect()
}

pub fn recontextualize_rule(
    core: &str,
    _gist: &str,
    profile: &EntityProfile,
    valence: f32,
    disgust: f32,
) -> String {
    crate::dream::retell(core, profile, valence, disgust)
}

/// Count a miss, or rewrite the gist toward the core once the budget is spent.
pub fn apply_grounding(
    trace: &mut MemoryTrace,
    profile: &EntityProfile,
    generated: &str,
    core: &str,
    narrator_rewrite: Option<String>,
) -> GroundingOutcome {
    if trace.channel == Channel::World {
        let spoken = if generated.trim().is_empty() {
            trace.gist.clone()
        } else {
            generated.to_string()
        };
        return GroundingOutcome {
            spoken_text: spoken,
            pulled_toward_core: false,
            overlap_with_core: 1.0,
        };
    }

    let judgement = judge_against_core(generated, core);
    let overlap = judgement.overlap;
    if !is_grounding_miss(generated, core, profile.ground_min_overlap) {
        if trace.detach_strikes > 0 {
            trace.detach_strikes -= 1;
        }
        return GroundingOutcome {
            spoken_text: generated.to_string(),
            pulled_toward_core: false,
            overlap_with_core: overlap,
        };
    }

    let grip = grip_on_trace(trace, profile);
    if grip < 0.12 {
        return GroundingOutcome {
            spoken_text: generated.to_string(),
            pulled_toward_core: false,
            overlap_with_core: overlap,
        };
    }

    trace.detach_strikes = trace.detach_strikes.saturating_add(1);
    let allowed_misses = misses_before_rewrite(trace, profile);
    if (trace.detach_strikes as usize) < allowed_misses {
        return GroundingOutcome {
            spoken_text: generated.to_string(),
            pulled_toward_core: false,
            overlap_with_core: overlap,
        };
    }

    let toward_core = narrator_rewrite
        .filter(|text| !text.trim().is_empty())
        .unwrap_or_else(|| {
            recontextualize_rule(core, &trace.gist, profile, trace.valence, trace.disgust)
        });
    let spoken = mix_drifted_with_core(generated, &toward_core, grip);
    if spoken.trim() == generated.trim() {
        return GroundingOutcome {
            spoken_text: generated.to_string(),
            pulled_toward_core: false,
            overlap_with_core: overlap,
        };
    }

    trace.gist = spoken.clone();
    trace.detach_strikes = 0;
    trace.drifts.push(DriftEvent {
        kind: DriftKind::Ground,
        at: now_secs(),
        note: format!(
            "reprise grip={grip:.2} misses_allowed={allowed_misses} kind={:?} overlap={overlap:.2}",
            judgement.kind
        ),
        fidelity_delta: 0.0,
        valence_delta: 0.0,
        disgust_delta: 0.0,
    });
    trace.clamp();
    GroundingOutcome {
        spoken_text: spoken,
        pulled_toward_core: true,
        overlap_with_core: overlap,
    }
}
