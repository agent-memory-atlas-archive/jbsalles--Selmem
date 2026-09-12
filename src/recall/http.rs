use crate::net::httpx::{extract_json_string, json_esc, post_json};
use crate::core::model::{MemoryTrace, Mood};
use crate::core::talk::WorkingTalk;
use crate::recall::narrator::{Narrator, RuleNarrator};

pub struct HttpNarrator {
    pub url: String,
    pub model: String,
    pub api_key: Option<String>,
    fallback: RuleNarrator,
}

impl HttpNarrator {
    pub fn parse(endpoint: &str, model: impl Into<String>, api_key: Option<String>) -> Option<Self> {
        if !(endpoint.starts_with("http://") || endpoint.starts_with("https://")) {
            return None;
        }
        Some(Self {
            url: endpoint.to_string(),
            model: model.into(),
            api_key,
            fallback: RuleNarrator,
        })
    }

    fn chat(&self, system: &str, user: &str) -> Result<String, String> {
        let cfg = crate::config::Config::get();
        let temp = cfg.temp();
        let effort = cfg.reasoning();
        let extra = if effort.is_empty() || effort == "off" {
            String::new()
        } else {
            format!(",\"reasoning_effort\":\"{}\"", json_esc(&effort))
        };
        let body = format!(
            "{{\"model\":\"{}\",\"temperature\":{},\"max_tokens\":280{} ,\"messages\":[{{\"role\":\"system\",\"content\":\"{}\"}},{{\"role\":\"user\",\"content\":\"{}\"}}]}}",
            json_esc(&self.model),
            temp,
            extra,
            json_esc(system),
            json_esc(user)
        );
        let raw = post_json(&self.url, self.api_key.as_deref(), &body)?;
        if let Some(msg) = api_error(&raw) {
            return Err(msg);
        }
        extract_json_string(&raw, "content")
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| {
                let clip: String = raw.chars().take(240).collect();
                format!("réponse LLM illisible: {clip}")
            })
    }
}

impl Narrator for HttpNarrator {
    fn reconstruct(&self, trace: &MemoryTrace, mood: &Mood, query: &str) -> String {
        if trace.status == crate::core::model::TraceStatus::Latent {
            return crate::lexicon::rule().latent.clone();
        }
        let system = &crate::lexicon::prompts().reconstruct;
        let user = format!(
            "gist: {}\nschema: {}\nvalence: {:.2} arousal: {:.2} disgust: {:.2} fidelity: {:.2}\nhumeur actuelle: v={:.2} a={:.2} d={:.2}\nindice de rappel: {}",
            trace.gist,
            trace.schema.as_deref().unwrap_or("-"),
            trace.valence,
            trace.arousal,
            trace.disgust,
            trace.fidelity,
            mood.valence,
            mood.arousal,
            mood.disgust,
            query
        );
        self.chat(system, &user)
            .unwrap_or_else(|_| self.fallback.reconstruct(trace, mood, query))
    }

    fn distill_axiom(&self, traces: &[&MemoryTrace]) -> Option<String> {
        let mut block = String::new();
        for t in traces {
            block.push_str(&format!(
                "- [{}] v={:.2} d={:.2} {}\n",
                t.schema.as_deref().unwrap_or("-"),
                t.valence,
                t.disgust,
                t.gist
            ));
        }
        let system = &crate::lexicon::prompts().distill;
        match self.chat(system, &block) {
            Ok(s) => {
                let s = s.trim().to_string();
                if s.is_empty() {
                    self.fallback.distill_axiom(traces)
                } else {
                    Some(s)
                }
            }
            Err(_) => self.fallback.distill_axiom(traces),
        }
    }

    fn interpret(
        &self,
        event: &str,
        mood: &Mood,
        axioms: &[String],
    ) -> Option<crate::recall::narrator::Interpretation> {
        let system = &crate::lexicon::prompts().interpret;
        let user = format!(
            "événement: {}\nhumeur: v={:.2} a={:.2} d={:.2}\naxiomes: {}",
            event,
            mood.valence,
            mood.arousal,
            mood.disgust,
            axioms.iter().take(5).cloned().collect::<Vec<_>>().join(" | ")
        );
        match self.chat(system, &user) {
            Ok(raw) => parse_interp(&raw).or_else(|| self.fallback.interpret(event, mood, axioms)),
            Err(_) => self.fallback.interpret(event, mood, axioms),
        }
    }

    fn rewrite(
        &self,
        trace: &MemoryTrace,
        neighbors: &[&MemoryTrace],
        profile: &crate::core::profile::EntityProfile,
    ) -> Option<String> {
        let gild = profile.embellish_gain - profile.disgust_gain;
        let p = crate::lexicon::prompts();
        let voice = if gild >= 0.04 {
            p.voice_tender.as_str()
        } else if gild <= -0.04 {
            p.voice_austere.as_str()
        } else {
            p.voice_neutral.as_str()
        };
        let mut block = format!(
            "core: {}\ngist: {}\nvalence: {:.2} disgust: {:.2} fidelity: {:.2} schema: {}\nvoix: {}\n",
            trace.core,
            trace.gist,
            trace.valence,
            trace.disgust,
            trace.fidelity,
            trace.schema.as_deref().unwrap_or("-"),
            voice
        );
        for n in neighbors.iter().take(3) {
            block.push_str(&format!("proche: {}\n", n.gist));
        }
        let system = &crate::lexicon::prompts().rewrite;
        match self.chat(system, &block) {
            Ok(s) => {
                let s = s.trim().to_string();
                if s.is_empty() {
                    self.fallback.rewrite(trace, neighbors, profile)
                } else {
                    Some(s)
                }
            }
            Err(_) => self.fallback.rewrite(trace, neighbors, profile),
        }
    }

    fn recontextualize(
        &self,
        trace: &MemoryTrace,
        core: &str,
        profile: &crate::core::profile::EntityProfile,
    ) -> String {
        let system = &crate::lexicon::prompts().recontextualize;
        let user = format!(
            "core: {}\ngist actuel: {}\nvalence: {:.2} disgust: {:.2} schema: {}",
            core,
            trace.gist,
            trace.valence,
            trace.disgust,
            trace.schema.as_deref().unwrap_or("-")
        );
        match self.chat(system, &user) {
            Ok(s) if !s.trim().is_empty() => s.trim().chars().take(280).collect(),
            _ => self.fallback.recontextualize(trace, core, profile),
        }
    }

    fn reply(
        &self,
        user: &str,
        memories: &[String],
        axioms: &[String],
        mood: &Mood,
        talk: &WorkingTalk,
    ) -> String {
        let mut ctx = String::new();
        ctx.push_str(&talk.render());
        for a in axioms.iter().take(4) {
            ctx.push_str("- axiome: ");
            ctx.push_str(a);
            ctx.push('\n');
        }
        for m in memories.iter().take(4) {
            ctx.push_str("- souvenir: ");
            ctx.push_str(m);
            ctx.push('\n');
        }
        let system = &crate::lexicon::prompts().reply;
        let user_p = format!(
            "humeur v={:.2} a={:.2} d={:.2}\n{ctx}\nhumain: {user}",
            mood.valence, mood.arousal, mood.disgust
        );
        match self.chat(system, &user_p) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("selmem LLM reply failed: {e}");
                self.fallback.reply(user, memories, axioms, mood, talk)
            }
        }
    }
}

fn api_error(raw: &str) -> Option<String> {
    if !raw.contains("\"error\"") || raw.contains("\"choices\"") {
        return None;
    }
    crate::net::httpx::first_string_field(raw, "message")
        .or_else(|| crate::net::httpx::first_string_field(raw, "error"))
        .or_else(|| Some(raw.chars().take(200).collect()))
}

/// LLM only writes the spoken answer. Reconstruction stays on the rules
/// so a bifurcation run does not spend a call per trace.
pub struct SpeakOnlyHttp {
    pub http: HttpNarrator,
    rules: RuleNarrator,
}

impl SpeakOnlyHttp {
    pub fn parse(endpoint: &str, model: impl Into<String>, api_key: Option<String>) -> Option<Self> {
        Some(Self {
            http: HttpNarrator::parse(endpoint, model, api_key)?,
            rules: RuleNarrator,
        })
    }
}

impl Narrator for SpeakOnlyHttp {
    fn reconstruct(&self, trace: &MemoryTrace, mood: &Mood, query: &str) -> String {
        self.rules.reconstruct(trace, mood, query)
    }
    fn distill_axiom(&self, traces: &[&MemoryTrace]) -> Option<String> {
        self.rules.distill_axiom(traces)
    }
    fn reply(
        &self,
        user: &str,
        memories: &[String],
        axioms: &[String],
        mood: &Mood,
        talk: &WorkingTalk,
    ) -> String {
        self.http.reply(user, memories, axioms, mood, talk)
    }
}

fn parse_interp(raw: &str) -> Option<crate::recall::narrator::Interpretation> {
    let t = raw.to_lowercase();
    let grab = |k: &str| {
        t.split_whitespace().find_map(|w| {
            w.strip_prefix(&format!("{k}="))
                .and_then(|x| x.trim_matches(|c| c == ';' || c == ',').parse::<f32>().ok())
        })
    };
    let v = grab("v")?;
    let a = grab("a").unwrap_or(0.35);
    let d = grab("d").unwrap_or(0.0);
    let r = grab("r").unwrap_or(0.55);
    let schema = t.split_whitespace().find_map(|w| {
        w.strip_prefix("s=")
            .map(|s| s.trim_matches(|c| c == ';' || c == ',').to_string())
    });
    let schema = schema.filter(|s| s != "-" && s.len() > 1);
    Some(crate::recall::narrator::Interpretation {
        valence: v.clamp(-1.0, 1.0),
        arousal: a.clamp(0.0, 1.0),
        disgust: d.clamp(0.0, 1.0),
        schema,
        self_relevance: r.clamp(0.0, 1.0),
    })
}
