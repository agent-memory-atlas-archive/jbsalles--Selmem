# SelMem

Selective reconstructive memory for LLM entities. v1.3

---

## Problem

An LLM is a stimulus–response transducer over tokens: context in, a distribution on the next token out. That mapping has no delay, no veto, and no state that binds the next act except the window and the weights. It is context-conditioned continuation. It is not an executive process, and it is not episodic memory.

What the predictor lacks is an *executive–autobiographical loop* — withhold the prepotent response, reconstruct a past rather than replay it, retouch traces offline, and let that residue color the next event.

We bolt on embeddings, logs, RAG. A stack of unmodified facts. Everything can come back, ranked by cosine. Nothing is forgotten. Nothing is warped. Nothing inhibits the prepotent token. Nothing accumulates into someone. The next sentence is still the most likely one given a larger pile.

That pile maximises *coverage*, not *deviation*. A complete, unbent context pulls the transducer back onto the average path: more evidence, same continuation. A particular past — holes, emphasis, a taste that refuses some material and overweights other — yields a non-average, path-dependent continuation. Two clones with the same log remain one voice. Two clones that forgot and gilded differently can disagree, and disagreement is a source of new text. That is not a proof of better thinking. It is the condition for the next token to belong to someone.

A system prompt is a mask. A vector index is a store. Reconstructive memory is neither. It refuses, delays, gilds, ruminates, lets unused aversive traces extinguish, and *interprets the next hour through what it already became*.

**Claim.** Memory that does not sculpt does not singularize. Identity that does not color encoding is still a mask. Singularity is not decoration on top of facts and not a claim of superior creativity. It is what lets the next token be other than the consensus of the stack. SelMem is an external loop around the transducer: selection, reconstruction, consolidation, identity.

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

If you inject verbatim “for accuracy,” you glued a store back on. The design forbids it.

---

## Rules

1. **Forget by default.** Encoding is a privilege. A per-entity salience gate drops the dull.
2. **Two books.** Lived trace (gist, core, affect, fidelity) vs sealed archive (verbatim). Only a human opens the archive. Dropped events leave no archive.
3. **Two channels.** `self` is sculpted. `world` stays cold.
4. **Reconstruct.** Recall uses schema, gist or core, mood. Never the original sentence.
5. **Dual drift.** Cherished memories gild. Recalled disgust darkens. Neglected disgust extinguishes.
6. **Charge ≠ detail.** The *core* holds. Detail follows Ebbinghaus.
7. **Anchor.** Trauma, triumph, vow, axiom: thicker skin, not immortality.
8. **Ladder.** Episode → motif (2 traces) → belief (3+) → trait (aligned beliefs). Lineage stays: “I used to believe X.”
9. **Identity loop.** Living axioms paint the next event *before* the gate. Recall can shift meaning under current mood, not only wording.
10. **Singularity.** Founders, traits, contradictions — not only mean valence. A proxy test checks that reconstruction sticks less than the unmodified log, clones are not interchangeable, and `world` facts survive.

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
     reconstruct → reconsolidate (gist blend + valence pull → Reinterpret)
              ↓
           sleep
     weather · rewrite · merge · extinguish
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

Every coefficient is an **exploratory knob**. Literature warrants the *shape* of a mechanism (selective encoding, reconstructive drift, gist vs detail, schema from repetition). It does not warrant 0.40, 0.18, “three traces make a belief,” or “two beliefs make a trait.” Those counts are discrete conveniences. See [PARAMETERS.md](PARAMETERS.md).

**Encode.**  
`S = w_a·A + w_n·N + w_s·R + w_u·U + w_g·G − w_r·Red`  
Below `τ` and permanence < 0.8: no lived trace, no archive.

If the caller sends no affect: lexical guess (FR+EN, weighted), then `identity::paint`, then `Narrator::interpret` when an HTTP narrator is wired. The interpreter sees the live sentence and living axioms. It does not see the sealed book.

**Recall.** Small top-k. Mix embedding, lexicon, mood congruence, access. Narrator never sees verbatim. Each recall costs fidelity and may move valence toward present mood (`DriftKind::Reinterpret`), slowed by anchor.

**Sleep.** Anchor → Ebbinghaus → extinguish unused disgust → sculpt → rewrite (bounded) → merge (two heavy anchors do not merge) → ladder → re-anchor. No LLM required.

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
    ↑
EXPERIENCE   live sentence, never the archive
```

A motif does not overwrite a strong belief. A weak belief (strength < 0.36) can be replaced by a new motif when the story turns. Traits are not minted from two anecdotes.

---

## Three levers

**Ebbinghaus.** Core is frozen at encode. Detail retention:

```
R(t) = exp(−t / S)
S = S₀ · (1+α·arousal) · (1+β·permanence) · (1+γ·anchor) · (1+δ·ln(1+rehearsals))
```

Night folds gist toward core. Forty days later a dull rain is still “it rained.” Not the window.

The exponential is a coding choice. Human fits are often a power law. We have not compared the two. The split *core vs detail* is the part that has a home in fuzzy-trace theory; the particular `S` weights do not.

**Rewrite.** Neighbors by schema or cosine. Keep the core, punch one detail, drop the rest. High anchor (`≥ 0.88`) is not rewritten.

**Anchor.** Seed from trauma / triumph / vow / intense self. Raised again if the trace supports a living axiom. Slows decay, rumination, and reinterpretation.

Fingerprint = valence, disgust, fidelity, anchor, core tokens, axioms, founder traces, trait count, same-schema contradictions. Distance 0 = clones. 1 = disjoint lives. A lab instrument, not a soul.

---

## Surface

Rust 1.75. Zero Cargo crates. SQLite (prepared statements, `BEGIN IMMEDIATE`, busy timeout) or flat `SELMEM1`. HTTP daemon + UI. One thread per connection; `/health` does not take the organ lock. Auth: `Authorization: Bearer` only.

```
GET  /health /who /lineage /mood /audit
POST /live /remember /speak /turn /sleep /save
```

```bash
./run.sh run --release --bin selmemd -- \
  --bind 0.0.0.0:7420 --path claire.db --name Claire \
  --profile tender --token secret \
  --llm https://api.openai.com/v1/chat/completions \
  --model gpt-4o-mini --api-key "$SELMEM_API_KEY"
```

No endpoint: rules + hashed vectors. Still runs.  
Ollama: `--llm http://127.0.0.1:11434/v1/chat/completions --model llama3`.

The model sees gist/core, schema, affect, fidelity, mood, living axioms.  
It never sees archive verbatim. `GET /audit` is for humans.

---

## Honest status

Held under `cargo test` (17): dull events drop with no archive, world channel stays clean, tender/austere diverge, Ebbinghaus keeps the core, trauma anchors, axioms supersede, two traces are a motif not a trait, identity colors a related event, recall can reinterpret, reconstruction sticks less than the unmodified log and clones are not interchangeable while `world` facts survive, persist round-trips, merge and extinction share a night.

Not held: no bake-off vs MemGPT or RAG at 10³–10⁵ turns. No human originality score — the test is a proxy for path-dependence, not a proof of better thought. No proof a human feels a presence. Layers are still rules, not learned features. Coefficients are exploratory knobs ([PARAMETERS.md](PARAMETERS.md)), not fitted constants. Core is lexical compression. `--embed` changes neighborhood, not truth. `interpret` without an HTTP narrator is still a lexicon plus identity paint. Rumination can be unjust; anchors are brakes, not ethics. Two processes on one `.db` remain a bad idea.

A selective memory is not judged on compile day. It is judged by what it forgot without noticing, by what it can no longer betray, and by whether the next hour arrives already someone.

MIT. [github.com/jbsalles/Selmem](https://github.com/jbsalles/Selmem)
