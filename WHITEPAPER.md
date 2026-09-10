# SelMem

Selective reconstructive memory for LLM entities. v1.5

---

## Problem

An LLM produces text by conditioning its next-token distribution on its current context and parameters. External memory can extend that context, but most existing approaches primarily treat memory as information that should remain available for retrieval: conversation histories are summarized, documents are indexed, and relevant items are retrieved when needed.

SelMem explores a different approach: memory as a mechanism for producing persistent individuality and more creative behavior.

The system models memory as a mutable trace of an experience, rather than as a permanent copy of the original event.

In SelMem:

- experiences can be selectively retained or forgotten;
- retained memories can lose surface details while preserving a semantic core;
- recall reconstructs a memory from its current state rather than replaying the original event;
- reconstruction can modify the memory itself;
- repeated experiences can form higher-level patterns, beliefs, and traits;
- these accumulated structures can influence how subsequent experiences are encoded.

The original observations are maintained separately in a sealed archive for auditing and evaluation. They are not used when the system recalls a memory. The model therefore operates on the transformed state of its memory rather than on an immutable record of the past.

This produces a feedback loop:

experience → selection → memory trace → reconstruction → consolidation → identity → future encoding

The hypothesis is that this process can cause otherwise identical LLMs to develop increasingly different internal histories. As those histories influence subsequent encoding and reconstruction, their behavioral trajectories can progressively diverge.

The intended result is not simply greater continuity or access to more information. It is singularity: a model whose future behavior is increasingly shaped by its own accumulated and transformed history.

This singularity may also provide a basis for greater creativity. If a model does not preserve and retrieve identical information in the same way each time, its accumulated history can introduce persistent biases, associations, preferences, and unexpected connections into future generations.

SelMem therefore investigates whether selective, imperfect, and reconstructive memory can transform an otherwise identical LLM into a progressively more singular and potentially more creative system.

This is an experimental hypothesis, not a claim that the mechanism necessarily produces superior intelligence or creativity.

---

## Not this

| Looks like | Is not |
|---|---|
| RAG with a TTL | Recall is reconstruction, not a read |
| Session summary | A summary is still storage. Here the text *and the charge* change |
| Personality prompt | Identity is residue: kept, forgotten, distorted, layered |
| Vector DB | Embedding is only neighborhood. The memory lives in gist, core, affect |
| Audit log for the model | The archive exists. The model never sees it |
| Emotion lexicon as soul | Lexicon is a fallback. Living axioms and optional `interpret` sit above it |
| Bigger unmodified context | More facts, same average path. Singularity is a bent past, not a taller stack |

If you inject verbatim “for accuracy,” you glued a store back on. The design forbids it. When a lived sentence leaves the *core* too often, the organ rewrites itself toward that core. It still never sees the archive.

---

## Rules

1. **Forget by default.** Encoding is a privilege. A per-entity salience gate drops the dull.
2. **Two books.** Each retained experience has two fundamentally different forms:
   - a lived trace containing gist, core, affect and fidelity;
   - a sealed archive containing the original verbatim experience.

   The lived trace is the memory used by the entity. The archive is an audit record and is not available to the model during ordinary recall. Dropped events leave no archive.
3. **Two channels.** `self` is sculpted. `world` stays cold.
4. **Reconstruct.** Recall uses schema, gist or core, mood and affect. It never replays the original sentence.
5. **Dual drift.** Cherished memories gild. Recalled disgust darkens. Neglected disgust extinguishes.
6. **Charge ≠ detail.** The core can remain stable while detail loses fidelity. The precision of remembered detail decreases independently from the persistence of the semantic core.
7. **Anchor.** Trauma, triumph, vow, axiom: thicker skin, not immortality.
8. **Ladder.** Episode → motif (2 traces) → belief (3+) → trait (aligned beliefs). Lineage stays: “I used to believe X.”
9. **Identity loop.** Living axioms paint the next event *before* the gate. Recall can shift meaning under current mood, not only wording.
10. **Singularity.** Founders, traits, contradictions — not only mean valence. A proxy test checks that reconstruction sticks less than the unmodified log, clones are not interchangeable, and `world` facts survive.
11. **Grounding without the journal.** Fading traces (cold, myth, low hold) may warp without a ceiling. On a trace that still matters, misses vs core increment `detach_strikes`. How soon and how hard the narrator pulls back scales with `narrator_firmness` × importance. The rewrite is a blend toward the core, never the archive.
12. **Latent forgetting.** The scene can drop out of recall while schema and affect still color the next event. “I no longer know why I react this way, but the reaction persists.”

---

## Loop

```
experience
    → interpret (lexicon, then identity, then optional LLM on the live sentence)
    → identity paint
    → salience gate
    → lived book  +  sealed archive
              ↓
     remember / speak
     reconstruct → reconsolidate
     (misses vs core increment detach_strikes;
      fading traces skip this; living traces blend back
      in proportion to narrator_firmness × importance)
              ↓
           sleep
     weather · rewrite · merge · extinguish · latent
     motif / belief / trait · re-anchor
              ↓
          who_am_i
              ↓
     next experience is already colored
```

Profiles `tender` / `austere` are an *initial sensitivity*, not a personality file. Same corpus, two gates, two nights → two pasts. After that the history does the rest.

| | tender | austere | status |
|---|---|---|---|
| τ | 0.40 | 0.55 | contrast pair, not a measured gate |
| decay λ | 0.10 | 0.06 | exploratory time constants |
| embellish | 0.18 | 0.05 | contrast pair; reconstructive *direction* is old, the digit is not |
| disgust gain | 0.05 | 0.16 | contrast pair |
| ground_min_overlap | 0.18 | 0.18 | exploratory Jaccard vs core |
| ground_strikes | 3 | 3 | discrete: two free misses, then rewrite |
| narrator_firmness | 0.42 | 0.72 | how hard the organ pulls a living gist back |

Every coefficient is an **exploratory knob**. The numbers are engineering choices, not psychological constants. See [PARAMETERS.md](PARAMETERS.md).

**Encode.**

```
S = w_a·A + w_n·N + w_s·R + w_u·U + w_g·G − w_r·Red
```

Below `τ` and permanence < 0.8: no lived trace, no archive.

If the caller sends no affect: lexical guess (FR+EN, weighted), then `identity::paint`, then `Narrator::interpret` when an HTTP narrator is wired. The interpreter sees the live sentence and living axioms. It does not see the sealed book.

**Recall.** Small top-k. Mix embedding, lexicon, mood congruence, access. Narrator never sees verbatim. Each recall costs fidelity and may move valence toward present mood (`DriftKind::Reinterpret`), slowed by anchor. Overlap vs *core* below `ground_min_overlap` is a miss. `hold = narrator_firmness × importance`. Hold near 0 (fading trace): unlimited warp. Hold high: fewer misses, then a *blend* of drifted gist and a core-facing rewrite (`DriftKind::Ground`). The journal stays sealed. A `Latent` trace is not reconstructed as a scene; its charge still paints encoding.

**Sleep.** Anchor → detail decay → extinguish unused disgust → sculpt → rewrite (bounded) → merge (two heavy anchors do not merge) → ladder → re-anchor. Scene-gone traces with remaining charge become latent. No LLM required.

---

## Five layers

```
IDENTITY     who_am_i: traits first, then beliefs, then motifs
    ↑
TRAITS       two aligned living beliefs
    ↑
BELIEFS      three or more traces under one schema
    ↑
MOTIFS       two traces under one schema
    ↑
TRACES       reconstructive episodes (gist + core + affect)
             or latent residue (charge without scene)
    ↑
EXPERIENCE   live sentence, never the archive
```

A motif does not overwrite a strong belief. A weak belief (strength < 0.36) can be replaced by a new motif when the story turns. Traits are not minted from two anecdotes.

---

## Three levers

**Detail decay.** Core is established at encode and provides a stable semantic reference for the lived trace. Remembered detail gradually loses precision according to the trace's decay state, while the semantic core can remain comparatively stable.

The distinction is intentional: the entity does not simply retain or delete an entire memory. A trace can become less precise while remaining meaningful. The surviving core can then serve as the reference against which later reconstructions are compared.

```
semantic core → comparatively stable
detail        → progressively less precise
fidelity      → tracks the state of the reconstruction
```

The decay function and its coefficients are implementation choices. They describe the behaviour of the SelMem memory mechanism, not a claim that human memory follows a particular mathematical law.

**Rewrite.** Neighbors by schema or cosine. Keep the core, punch one detail, drop the rest. High anchor (`≥ 0.88`) is not rewritten.

**Anchor.** Seed from trauma / triumph / vow / intense self. Raised again if the trace supports a living axiom. Slows decay, rumination, and reinterpretation.

Fingerprint = valence, disgust, fidelity, anchor, core tokens, axioms, founder traces, trait count, same-schema contradictions. Distance 0 = clones. 1 = disjoint lives. A lab instrument, not a soul.

---

## Surface

Rust 1.75. Zero Cargo crates. SQLite (prepared statements, `BEGIN IMMEDIATE`, busy timeout) or flat `SELMEM1`. HTTP daemon + UI. One thread per connection; `/health` does not take the organ lock. Auth: `Authorization: Bearer` only.

```
GET  /health /who /lineage /mood /profile /audit
POST /live /remember /speak /turn /sleep /save /profile
```

```bash
./run.sh run --release --bin selmemd -- \
  --bind 0.0.0.0:7420 --path claire.db --name Claire \
  --profile tender --ground-overlap 0.18 --ground-strikes 3 --narrator-firmness 0.42 --token secret \
  --llm https://api.openai.com/v1/chat/completions \
  --model gpt-4o-mini --api-key "$SELMEM_API_KEY"
```

No endpoint: rules + hashed vectors. Still runs.

Ollama: `--llm http://127.0.0.1:11434/v1/chat/completions --model llama3`.

The model sees gist/core, schema, affect, fidelity, mood, living axioms.

It never sees archive verbatim. `GET /audit` is for humans.

---

## Honest status

Held under `cargo test`: dull events drop with no archive, world channel stays clean, tender/austere diverge, semantic core survives detail decay, anchors preserve important traces, axioms supersede, two traces are a motif not a trait, identity colors a related event, recall can reinterpret, a living gist is allowed a few misses then blended toward the core without exposing the archive, a fading gist may keep warping, a latent trace forgets the scene and keeps the reaction, reconstruction sticks less than the unmodified log and clones are not interchangeable while `world` facts survive, persist round-trips, merge and extinction share a night.

Not held: no bake-off vs MemGPT or RAG at 10³–10⁵ turns. No human originality score — the test is a proxy for path-dependence, not a proof of better thought. No proof a human feels a presence. Layers are still rules, not learned features. Coefficients are exploratory knobs ([PARAMETERS.md](PARAMETERS.md)), not fitted constants. Core is lexical compression. `--embed` changes neighborhood, not truth. `interpret` without an HTTP narrator is still a lexicon plus identity paint.

Rumination can be unjust; anchors are brakes, not ethics. Two processes on one `.db` remain a bad idea.

A selective memory is not judged on compile day. It is judged by what it forgot without noticing, by what it can no longer betray, and by whether the next hour arrives already someone.

MIT. [github.com/jbsalles/Selmem](https://github.com/jbsalles/Selmem)
