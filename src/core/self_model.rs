//! The contextual self is a view of lived traces, not a second biography.
use super::{model::*, store::MemoryStore};
use crate::encode::{embed::Embedder, scoring::lexical_similarity};
use crate::recall::{Narrator, PropositionLabel, PropositionScorer};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelfRole { #[default] Episode, Belief, Aspiration, Commitment }
impl SelfRole {
    pub fn token(self)->&'static str { match self { Self::Episode=>"episode",Self::Belief=>"belief",Self::Aspiration=>"aspiration",Self::Commitment=>"commitment" } }
}
/// Provenance supplied by the caller, never inferred from a pronoun.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelfEvidence { Observation, #[default] Report, Interpretation }
impl SelfEvidence { pub fn token(self)->&'static str {match self {Self::Observation=>"observation",Self::Report=>"report",Self::Interpretation=>"interpretation"}} }
/// Runtime experimental interventions; reapply after loading a snapshot.
#[derive(Clone, Copy, Debug)]
pub struct SelfPolicy {pub selection:bool,pub consolidation:bool,pub opening:bool}
impl Default for SelfPolicy {fn default()->Self {Self{selection:true,consolidation:true,opening:true}}}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Appraisal {
    pub at:u64,pub continuity:f32,pub revision:f32,pub opening:bool,
    pub prior_ids:Vec<String>,pub supports:Vec<String>,pub contradicts:Vec<String>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelfMemory {
    pub role:SelfRole,pub evidence:SelfEvidence,pub context:String,
    /// Stored before future observations, never inferred retrospectively.
    pub expectation:String,pub importance:f32,pub certainty:f32,
    pub support_ids:Vec<String>,pub counter_ids:Vec<String>,
    /// Opaque source/observation keys, not retrievable event contents.
    pub consumed:Vec<String>,pub foundations_missing:bool,pub retired:bool,
    pub protected_until:u64,pub appraisal:Appraisal,
}
impl SelfMemory {
    pub fn validate(&self)->Result<(),String> {
        for x in [self.importance,self.certainty,self.appraisal.continuity,self.appraisal.revision] {
            if !x.is_finite() || !(0.0..=1.0).contains(&x) {return Err("invalid self-memory score".into());}
        }
        if self.role!=SelfRole::Episode && self.context.trim().is_empty() {return Err("self proposition requires a context".into());}
        Ok(())
    }
    /// Shared, versioned length-prefixed codec for file and SQLite containers.
    pub fn pack(&self)->String {
        let mut fields=vec!["1".into(),self.role.token().into(),self.evidence.token().into(),self.context.clone(),self.expectation.clone(),self.importance.to_string(),self.certainty.to_string(),self.foundations_missing.to_string(),self.retired.to_string(),self.protected_until.to_string(),self.appraisal.at.to_string(),self.appraisal.continuity.to_string(),self.appraisal.revision.to_string(),self.appraisal.opening.to_string()];
        for xs in [&self.support_ids,&self.counter_ids,&self.consumed,&self.appraisal.prior_ids,&self.appraisal.supports,&self.appraisal.contradicts] {fields.push(xs.len().to_string());fields.extend(xs.iter().cloned());}
        fields.iter().map(|s|format!("{}:{s}",s.len())).collect()
    }
    pub fn unpack(mut s:&str)->Result<Self,String> {
        if s.is_empty(){return Ok(Self::default());}
        fn field(s:&mut &str)->Result<String,String>{let(n,rest)=s.split_once(':').ok_or("invalid self-memory framing")?;let n:usize=n.parse().map_err(|_|"invalid length")?;let v=rest.get(..n).ok_or("truncated self memory")?.to_string();*s=rest.get(n..).ok_or("invalid UTF-8 boundary")?;Ok(v)}
        macro_rules! value {()=>{field(&mut s)?.parse().map_err(|_|"invalid self-memory value".to_string())?};}
        if field(&mut s)?!="1" {return Err("unsupported self-memory version".into());}
        let role=match field(&mut s)?.as_str(){"episode"=>SelfRole::Episode,"belief"=>SelfRole::Belief,"aspiration"=>SelfRole::Aspiration,"commitment"=>SelfRole::Commitment,_=>return Err("invalid role".into())};
        let evidence=match field(&mut s)?.as_str(){"observation"=>SelfEvidence::Observation,"report"=>SelfEvidence::Report,"interpretation"=>SelfEvidence::Interpretation,_=>return Err("invalid evidence".into())};
        let mut m=Self{role,evidence,context:field(&mut s)?,expectation:field(&mut s)?,importance:value!(),certainty:value!(),foundations_missing:value!(),retired:value!(),protected_until:value!(),appraisal:Appraisal{at:value!(),continuity:value!(),revision:value!(),opening:value!(),..Appraisal::default()},..Self::default()};
        for xs in [&mut m.support_ids,&mut m.counter_ids,&mut m.consumed,&mut m.appraisal.prior_ids,&mut m.appraisal.supports,&mut m.appraisal.contradicts] {let n:usize=value!();if n>100_000{return Err("oversized self-memory list".into());}for _ in 0..n{xs.push(field(&mut s)?);}}
        if !s.is_empty(){return Err("trailing self-memory data".into());}m.validate()?;Ok(m)
    }
}
pub fn available(t:&MemoryTrace)->bool {!t.suppressed&&t.status!=TraceStatus::Latent&&t.access>0.05}
/// Fresh, deterministic contextual view. No cache and no archive fallback.
pub fn view<'a>(store:&'a MemoryStore,query:&str,context:Option<&str>)->Vec<&'a MemoryTrace> {
    let mut xs:Vec<_>=store.traces.values().filter(|t|available(t)&&t.personal.role!=SelfRole::Episode&&!t.personal.retired)
        .filter(|t|context==Some(t.personal.context.as_str())||lexical_similarity(query,&format!("{} {}",t.personal.context,t.gist))>=0.08).collect();
    xs.sort_by(|a,b|b.personal.importance.total_cmp(&a.personal.importance).then(a.id.cmp(&b.id)));xs.truncate(8);xs
}
/// Pure pre-event appraisal. A null witness means unknown, never contradiction.
pub fn appraise(store:&MemoryStore,event:&str,context:Option<&str>,scorer:&dyn PropositionScorer)->Appraisal {
    let mut a=Appraisal{at:now_secs(),..Appraisal::default()};
    for t in view(store,event,context) {a.prior_ids.push(t.id.clone());a.continuity=a.continuity.max(t.personal.importance);
        if t.personal.expectation.is_empty(){continue;}
        match scorer.score(&t.personal.expectation,event){PropositionLabel::Entail=>a.supports.push(t.id.clone()),PropositionLabel::Contradict=>{a.contradicts.push(t.id.clone());a.revision=0.8;},PropositionLabel::Unknown=>()}
    }
    let hash=event.bytes().fold(14695981039346656037u64,|h,b|(h^b as u64).wrapping_mul(1099511628211));
    a.opening=store.self_policy.selection&&store.self_policy.opening&&hash%20==0&&store.traces.values().filter(|t|available(t)&&t.personal.appraisal.opening).count()<16;a
}
fn root(t:&MemoryTrace)->String {format!("{}:{}{}",t.source.len(),t.source,t.observation_id.as_deref().unwrap_or(&t.id))}
/// Bounded revision consumes independent observed roots once, even if capped.
pub fn consolidate(store:&mut MemoryStore,narrator:&dyn Narrator,embedder:&dyn Embedder){
    if !store.self_policy.consolidation{return;}
    let episodes:Vec<_>=store.active_ids().into_iter().filter_map(|id|store.traces.get(&id))
        .filter(|t|available(t)&&t.personal.role==SelfRole::Episode&&t.personal.evidence==SelfEvidence::Observation&&t.reality.verifiable&&t.source!="talk"&&t.source!="self-consolidation"&&!t.drifts.iter().any(|d|matches!(d.kind,DriftKind::Merge|DriftKind::Confabulate))).cloned().collect();
    let live:BTreeSet<_>=store.traces.values().filter(|t|available(t)).map(|t|t.id.clone()).collect();
    for t in store.traces.values_mut().filter(|t|t.personal.role!=SelfRole::Episode){let p=&mut t.personal;
        let missing=p.support_ids.iter().any(|id|!live.contains(id));if missing&&!p.foundations_missing{p.certainty=(p.certainty-0.1).max(0.0);}p.foundations_missing=missing;
        if t.suppressed||p.retired{continue;}let mut delta=0.0f32;
        for e in &episodes{let a=&e.personal.appraisal;let support=a.supports.contains(&t.id);let counter=a.contradicts.contains(&t.id);if !support&&!counter{continue;}
            let key=root(e);if p.consumed.contains(&key){continue;}p.consumed.push(key);
            let weight=0.15*e.semantic.confidence.clamp(0.0,1.0);
            if counter{p.counter_ids.push(e.id.clone());delta-=weight;}else{p.support_ids.push(e.id.clone());delta+=weight;}
        }
        if p.role==SelfRole::Belief{p.certainty=(p.certainty+delta.clamp(-0.15,0.15)).clamp(0.0,0.95);if p.certainty<=0.05&&!p.counter_ids.is_empty(){p.retired=true;}}
        t.confidence=p.certainty;
    }
    let mut groups:BTreeMap<String,Vec<&MemoryTrace>>=BTreeMap::new();
    for e in &episodes{if e.attribution!=Attribution::Internal||e.stake_kind==StakeKind::Promise{continue;}if let Some(s)=e.schema.as_ref().filter(|s|!s.is_empty()){groups.entry(s.clone()).or_default().push(e);}}
    for(context,mut group)in groups{let mut roots=BTreeSet::new();group.retain(|t|roots.insert(root(t)));
        if group.len()<2||store.traces.values().any(|t|!t.personal.retired&&t.personal.role!=SelfRole::Episode&&t.personal.context==context){continue;}group.truncate(8);
        let Some((statement,expectation))=narrator.distill_self(&group,&context)else{continue;};
        if statement.trim().is_empty()||expectation.trim().is_empty()||statement.chars().count()>800||expectation.chars().count()>800{continue;}
        let mut t=group[0].clone();t.id=new_id("self");t.gist=statement;t.core=t.gist.clone();
        t.personal=SelfMemory{role:SelfRole::Belief,evidence:SelfEvidence::Interpretation,context,expectation,importance:group.iter().map(|t|t.self_relevance).fold(0.0,f32::max),certainty:0.35,support_ids:group.iter().map(|t|t.id.clone()).collect(),consumed:group.iter().map(|t|root(t)).collect(),..SelfMemory::default()};
        t.archive_id=None;t.observation_id=None;t.source="self-consolidation".into();t.reality=RealityAnchor::default();t.operations.clear();t.drifts.clear();t.rehearsals=0;t.created_at=now_secs();t.last_recalled_at=None;t.last_consolidated_at=Some(now_secs());t.embedding=embedder.embed(&t.gist);t.semantic.claim=t.gist.clone();t.semantic.confidence=0.35;t.interpretation.statement=t.gist.clone();t.interpretation.confidence=0.35;t.confidence=0.35;t.permanence=0.4;t.anchor=0.;store.add_trace(t);
    }
}
