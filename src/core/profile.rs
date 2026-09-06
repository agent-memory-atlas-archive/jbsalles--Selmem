#[derive(Clone, Debug)]
pub struct EntityProfile {
    pub name: String,
    pub encode_threshold: f32,
    pub w_arousal: f32,
    pub w_novelty: f32,
    pub w_self: f32,
    pub w_utility: f32,
    pub w_goal: f32,
    pub w_redundancy: f32,
    pub decay_lambda: f32,
    pub rehearsal_boost: f32,
    pub embellish_gain: f32,
    pub disgust_gain: f32,
    pub disgust_cap: f32,
    pub fidelity_loss_on_recall: f32,
    pub reconsolidation_eta: f32,
    pub mood_blend: f32,
    pub cold_access: f32,
    pub myth_access: f32,
    pub max_recall: usize,
    pub extinction_rate: f32,
    pub merge_similarity: f32,
}

impl EntityProfile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            encode_threshold: 0.45,
            w_arousal: 0.25,
            w_novelty: 0.15,
            w_self: 0.25,
            w_utility: 0.15,
            w_goal: 0.10,
            w_redundancy: 0.20,
            decay_lambda: 0.08,
            rehearsal_boost: 0.18,
            embellish_gain: 0.12,
            disgust_gain: 0.10,
            disgust_cap: 0.92,
            fidelity_loss_on_recall: 0.04,
            reconsolidation_eta: 0.25,
            mood_blend: 0.08,
            cold_access: 0.12,
            myth_access: 0.04,
            max_recall: 4,
            extinction_rate: 0.06,
            merge_similarity: 0.32,
        }
    }

    pub fn tender(name: impl Into<String>) -> Self {
        Self {
            encode_threshold: 0.40,
            embellish_gain: 0.18,
            disgust_gain: 0.05,
            decay_lambda: 0.10,
            ..Self::new(name)
        }
    }

    pub fn austere(name: impl Into<String>) -> Self {
        Self {
            encode_threshold: 0.55,
            embellish_gain: 0.05,
            disgust_gain: 0.16,
            decay_lambda: 0.06,
            w_self: 0.30,
            ..Self::new(name)
        }
    }
}
