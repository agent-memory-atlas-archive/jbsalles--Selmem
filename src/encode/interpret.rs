//! Lexicon, then optional `Narrator::interpret`. Does not paint identity.

use crate::core::model::Mood;
use crate::encode::EncodeInput;
use crate::recall::narrator::Narrator;

/// Fill affect / schema when the caller sent none. Lexicon first, narrator second.
pub fn interpret(
    input: &mut EncodeInput<'_>,
    mood: &Mood,
    axioms: &[String],
    narrator: &dyn Narrator,
) {
    let uninterpreted =
        input.schema.is_none() && input.valence.abs() < 0.08 && input.disgust < 0.08;
    if !uninterpreted {
        return;
    }
    let (v, a, d, s) = crate::encode::affect::guess(input.event);
    input.valence = v;
    input.arousal = a;
    input.disgust = d;
    input.schema = s;
    if let Some(interp) = narrator.interpret(input.event, mood, axioms) {
        input.valence = interp.valence;
        input.arousal = interp.arousal;
        input.disgust = interp.disgust;
        if input.schema.is_none() {
            input.schema = interp.schema;
        }
        input.self_relevance = input.self_relevance.max(interp.self_relevance);
    }
}
