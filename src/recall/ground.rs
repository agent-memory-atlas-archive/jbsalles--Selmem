//! Drift is cheap on fading traces. Return to the core is proportional
//! to narrator firmness × how much the trace still matters.
//! The sealed archive is never used.
use crate::core::model::{now_secs, Channel, DriftEvent, DriftKind, MemoryTrace, TraceStatus};
use crate::core::profile::EntityProfile;
use crate::encode::scoring::lexical_similarity;

pub struct Check {
    pub text: String,
    pub corrected: bool,
    pub overlap: f32,
}

pub fn core_charge(trace: &MemoryTrace) -> String {
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

/// Cold, myth, or already slipping out of reach: allowed to warp forever.
pub fn fading(trace: &MemoryTrace) -> bool {
    matches!(trace.status, TraceStatus::Cold | TraceStatus::Myth)
        || (trace.fidelity < 0.38 && trace.access < 0.28 && trace.permanence < 0.35 && trace.anchor < 0.40)
}

/// How tightly this narrator holds *this* trace. 0 = let it go.
pub fn hold(trace: &MemoryTrace, profile: &EntityProfile) -> f32 {
    if fading(trace) {
        return 0.0;
    }
    let importance = (0.35 * trace.permanence
        + 0.25 * trace.anchor
        + 0.20 * trace.access
        + 0.20 * trace.fidelity)
        .clamp(0.0, 1.0);
    (profile.narrator_firmness.clamp(0.0, 1.0) * importance).clamp(0.0, 1.0)
}

pub fn strikes_needed(trace: &MemoryTrace, profile: &EntityProfile) -> usize {
    let h = hold(trace, profile);
    if h < 0.12 {
        return usize::MAX;
    }
    let base = profile.ground_strikes.max(1) as f32;
    (base / h).round().clamp(1.0, 24.0) as usize
}

pub fn will_correct(trace: &MemoryTrace, profile: &EntityProfile, generated: &str, core: &str) -> bool {
    if fading(trace) || hold(trace, profile) < 0.12 {
        return false;
    }
    overlap_with_core(generated, core) < profile.ground_min_overlap
        && (trace.detach_strikes as usize) + 1 >= strikes_needed(trace, profile)
}

/// Mix drifted text with a core-facing rewrite. High hold → more core.
pub fn blend(drifted: &str, toward: &str, mix: f32) -> String {
    let mix = mix.clamp(0.0, 1.0);
    if mix < 0.18 || toward.trim().is_empty() {
        return drifted.to_string();
    }
    if mix >= 0.82 {
        return toward.chars().take(280).collect();
    }
    let old: Vec<&str> = drifted.split_whitespace().collect();
    let neu: Vec<&str> = toward.split_whitespace().collect();
    if old.is_empty() {
        return toward.to_string();
    }
    let keep_n = ((old.len() as f32) * (1.0 - mix)).round() as usize;
    let take_n = ((neu.len() as f32) * mix).round().max(1.0) as usize;
    let mut out = Vec::new();
    out.extend(old.into_iter().take(keep_n.max(1)));
    out.extend(neu.into_iter().take(take_n));
    let s = out.join(" ");
    s.chars().take(280).collect()
}

pub fn recontextualize_rule(core: &str, _gist: &str, profile: &EntityProfile, valence: f32, disgust: f32) -> String {
    crate::dream::retell(core, profile, valence, disgust)
}

pub fn note(
    trace: &mut MemoryTrace,
    profile: &EntityProfile,
    generated: &str,
    core: &str,
    rewritten: Option<String>,
) -> Check {
    if trace.channel == Channel::World {
        return Check {
            text: if generated.trim().is_empty() {
                trace.gist.clone()
            } else {
                generated.to_string()
            },
            corrected: false,
            overlap: 1.0,
        };
    }
    let overlap = overlap_with_core(generated, core);
    if overlap >= profile.ground_min_overlap {
        if trace.detach_strikes > 0 {
            trace.detach_strikes -= 1;
        }
        return Check {
            text: generated.to_string(),
            corrected: false,
            overlap,
        };
    }
    let h = hold(trace, profile);
    if h < 0.12 {
        return Check {
            text: generated.to_string(),
            corrected: false,
            overlap,
        };
    }
    trace.detach_strikes = trace.detach_strikes.saturating_add(1);
    let need = strikes_needed(trace, profile);
    if (trace.detach_strikes as usize) < need {
        return Check {
            text: generated.to_string(),
            corrected: false,
            overlap,
        };
    }
    let toward = rewritten
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| {
            recontextualize_rule(core, &trace.gist, profile, trace.valence, trace.disgust)
        });
    let next = blend(generated, &toward, h);
    if next.trim() == generated.trim() {
        return Check {
            text: generated.to_string(),
            corrected: false,
            overlap,
        };
    }
    trace.gist = next.clone();
    trace.detach_strikes = 0;
    trace.drifts.push(DriftEvent {
        kind: DriftKind::Ground,
        at: now_secs(),
        note: format!("reprise hold={h:.2} need={need} overlap={overlap:.2}"),
        fidelity_delta: 0.0,
        valence_delta: 0.0,
        disgust_delta: 0.0,
    });
    trace.clamp();
    Check {
        text: next,
        corrected: true,
        overlap,
    }
}
