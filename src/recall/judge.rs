//! Pure check: does the spoken sentence still sit on the frozen core?
//!
//! No `MemoryTrace`, no profile, no I/O. A float cut may be passed in
//! (`is_grounding_miss`). Embeddings rank recall; they do not decide this.

use crate::encode::scoring::{lexical_similarity, token_set};
use crate::lexicon;

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
    /// Unauthorized *claim* (write-back). `Reframe` is tone, not a miss.
    pub fn is_miss(self) -> bool {
        matches!(self, Self::Elaborate | Self::Contradict | Self::Depart)
    }

    /// Same event, other speech act. Speak it; do not pull the book.
    pub fn is_color(self) -> bool {
        matches!(self, Self::Reframe)
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
        // Same event + new affect frame → color. Low overlap is another scene.
        let kind = if overlap >= 0.18 {
            DetachKind::Reframe
        } else {
            DetachKind::Depart
        };
        return CoreJudgement { kind, overlap };
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

/// Miss if the kind is unauthorized, or if the identity gate fails the cut.
pub fn is_grounding_miss(generated: &str, core: &str, min_overlap: f32) -> bool {
    let j = judge_against_core(generated, core);
    match j.kind {
        DetachKind::Hold => j.overlap < min_overlap,
        DetachKind::Compress | DetachKind::Reframe => false,
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
