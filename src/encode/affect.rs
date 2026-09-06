/// Cheap lexical affect for HTTP/CLI when the caller sends no valence.
/// Weighted stems, FR + EN. Not a classifier.
pub fn guess(text: &str) -> (f32, f32, f32, Option<String>) {
    let t = text.to_lowercase();
    let neg: &[(&str, f32)] = &[
        ("trahi", 1.0),
        ("betray", 1.0),
        ("humili", 1.0),
        ("ridicul", 0.9),
        ("ri de", 0.8),
        ("laughed at", 0.8),
        ("mensonge", 0.8),
        ("lied", 0.8),
        ("dégoût", 0.9),
        ("disgust", 0.9),
        ("détest", 0.8),
        ("detest", 0.8),
        ("hate", 0.8),
        ("annul", 0.7),
        ("cancel", 0.6),
        ("abandon", 0.9),
        ("left me", 0.8),
        ("seul", 0.5),
        ("alone", 0.5),
        ("colère", 0.7),
        ("anger", 0.7),
        ("angry", 0.7),
        ("peur", 0.6),
        ("afraid", 0.6),
        ("fear", 0.6),
        ("honte", 0.8),
        ("shame", 0.8),
        ("insult", 0.7),
        ("cruel", 0.7),
        ("blessure", 0.7),
        ("hurt", 0.6),
        ("pain", 0.5),
        ("douleur", 0.6),
    ];
    let pos: &[(&str, f32)] = &[
        ("merci", 0.7),
        ("thank", 0.7),
        ("resté", 0.8),
        ("stayed", 0.8),
        ("fidèle", 0.8),
        ("faithful", 0.8),
        ("loyal", 0.7),
        ("aime", 0.8),
        ("love", 0.8),
        ("ador", 0.7),
        ("présent", 0.6),
        ("present", 0.5),
        ("doux", 0.6),
        ("tender", 0.6),
        ("pluie", 0.4),
        ("rain", 0.3),
        ("confiance", 0.7),
        ("trust", 0.7),
        ("sourire", 0.6),
        ("smile", 0.6),
        ("ensemble", 0.6),
        ("together", 0.6),
        ("pardon", 0.5),
        ("forgiv", 0.6),
        ("espoir", 0.5),
        ("hope", 0.5),
        ("fierté", 0.6),
        ("proud", 0.6),
        ("joie", 0.7),
        ("joy", 0.7),
        ("heureux", 0.7),
        ("happy", 0.7),
    ];

    let n: f32 = neg.iter().filter(|(w, _)| t.contains(w)).map(|(_, w)| *w).sum();
    let p: f32 = pos.iter().filter(|(w, _)| t.contains(w)).map(|(_, w)| *w).sum();
    let bang = t.matches('!').count() as f32 * 0.08;
    let intens = if t.contains("très") || t.contains("trop") || t.contains("so ") || t.contains("never") {
        0.12
    } else {
        0.0
    };

    if n > p + 0.15 {
        let arousal = (0.45 + 0.15 * n + bang + intens).min(0.95);
        let disgust = (0.22 + 0.18 * n).min(0.85);
        (-(0.35 + 0.2 * n).min(0.95), arousal, disgust, Some("blessure".into()))
    } else if p > n + 0.15 {
        let arousal = (0.32 + 0.12 * p + bang).min(0.9);
        ((0.35 + 0.18 * p).min(0.95), arousal, 0.0, Some("lien".into()))
    } else if n > 0.0 && p > 0.0 {
        ((p - n) * 0.25, 0.4 + bang, (0.15 * n).min(0.4), Some("ambivalence".into()))
    } else {
        (0.0, (0.18 + bang).min(0.5), 0.0, None)
    }
}
