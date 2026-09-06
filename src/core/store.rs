use std::collections::{HashMap, HashSet};

use crate::core::model::{ArchiveRecord, IdentityAxiom, MemoryTrace, TraceStatus};

#[derive(Default)]
pub struct MemoryStore {
    pub traces: HashMap<String, MemoryTrace>,
    pub archives: HashMap<String, ArchiveRecord>,
    pub axioms: HashMap<String, IdentityAxiom>,
    pub edges: HashMap<String, HashSet<String>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_archive(&mut self, record: ArchiveRecord) -> String {
        let id = record.id.clone();
        self.archives.insert(id.clone(), record);
        id
    }

    pub fn add_trace(&mut self, trace: MemoryTrace) -> String {
        let id = trace.id.clone();
        self.traces.insert(id.clone(), trace);
        id
    }

    pub fn add_axiom(&mut self, axiom: IdentityAxiom) -> String {
        let id = axiom.id.clone();
        self.axioms.insert(id.clone(), axiom);
        id
    }

    pub fn link(&mut self, a: &str, b: &str) {
        if a == b {
            return;
        }
        self.edges.entry(a.to_string()).or_default().insert(b.to_string());
        self.edges.entry(b.to_string()).or_default().insert(a.to_string());
    }

    pub fn active_ids(&self) -> Vec<String> {
        self.traces
            .values()
            .filter(|t| t.status != TraceStatus::Sealed)
            .map(|t| t.id.clone())
            .collect()
    }

    pub fn living_axioms(&self) -> Vec<&IdentityAxiom> {
        self.axioms
            .values()
            .filter(|a| a.superseded_by.is_none())
            .collect()
    }
}
