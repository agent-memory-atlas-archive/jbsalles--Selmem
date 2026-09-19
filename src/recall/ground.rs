//! Compatibility façade. The judge lives in `judge`. The pull lives in `pull`.

pub use crate::recall::judge::{
    is_grounding_miss, judge_against_core, overlap_with_core, CoreJudgement, DetachKind,
};
pub use crate::recall::pull::{
    apply_grounding, grip_on_trace, is_slipping_away, misses_before_rewrite, mix_drifted_with_core,
    recontextualize_rule, semantic_core_of, should_force_core_rewrite, GroundingOutcome,
};
