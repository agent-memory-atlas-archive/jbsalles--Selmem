# SelMem

Selective reconstructive memory for LLM entities. v0.5

---

## Problem

An LLM maps a context window to a next-token distribution. Adding memory usually means keeping more of the past available: logs, summaries, embeddings, top-k retrieval.

SelMem treats memory as a state that changes. Not every event is stored. Stored events lose detail. Recall rebuilds a sentence from the current trace instead of replaying the original. That rebuild can write back. Repeated traces form motifs, then beliefs, then traits, which bias the next encode.

The verbatim event is kept in a sealed archive for tests and audit. The model never reads it. It only sees the lived trace (gist, core, affect, fidelity).

```
experience → selection → trace → recall → sleep → identity → next encode
```

Claim under test: two copies of the same model, given different retained histories, will not stay interchangeable. That is path dependence, not a claim of better intelligence or creativity.

---

## Not this

| Looks like | Difference |
| --- | --- |
| RAG with a TTL | Recall rebuilds a sentence; it does not return a stored chunk |
| Session summary | A summary is still a stored string. Here wording and affect both move |
| Personality prompt | No fixed persona file. What remains after nights is the bias |
| Vector DB | Embeddings rank neighbors. The record is gist + core + affect |
| Log injected into the prompt | Archive exists for humans (`GET /audit`). The model does not see it |

If you paste the original sentence back “for accuracy”, you have a store again. When a living gist drifts too far from its core, the system blends the gist toward that core. Still no archive.

---

## Rules

1. **Drop by default, forget by salience.** A score must still clear `τ` (now lower: 0.28 / 0.40). Weak keeps cool and weather faster than charged ones.
2. **Two records.** Lived: gist, core, affect, fidelity. Sealed: original text. Only the lived record is used at recall.
3. **Two channels.** `self` is rewritten over time. `world` is not.
4. **Reconstruct.** Recall uses schema, gist or core, mood, affect. Not the original string.
5. **Two drift directions.** High valence can gild. Recalled disgust can darken. Unused disgust can fade.
6. **Core ≠ detail.** The semantic core can hold while surface fidelity falls.
7. **Anchor.** High-permanence traces decay more slowly. They are not frozen.
8. **Ladder.** Episode → motif (2 traces, same schema) → belief (3+) → trait (two aligned beliefs). Superseded beliefs stay in lineage.
9. **Feedback.** Living axioms tint the next event before the gate. Recall can shift sense with current mood.
10. **Distance.** Fingerprint uses valence, disgust, fidelity, anchors, core tokens, axioms, founders, traits, contradictions. 0 = same book. 1 = disjoint books.
11. **Grounding.** Fading traces may keep warping. On a living trace, misses vs core increment `detach_strikes`. Pull-back strength is `narrator_firmness × importance`. Blend toward core, never toward the archive.
12. **Latent.** The scene can leave recall while schema and affect still bias encode.

---

## Loop

```
experience
    → interpret (lexicon → identity paint → optional LLM on the live sentence)
    → salience gate
    → lived book + sealed archive
              ↓
     remember / speak
     reconstruct → reconsolidate
     (living traces: misses vs core, then blend)
     talk frame holds the current thread (active ≤ 10 min gap, ≤ 2 h; sleep commits it through the gate, then drops it)
              ↓
           sleep
     decay · rewrite · merge · extinguish · latent
     motif / belief / trait · re-anchor
              ↓
          who_am_i
              ↓
     next experience already biased
```

`tender` / `austere` are two starting gates, not characters. Same input stream, two thresholds, two nights → two books.

|  | tender | austere | status |
| --- | --- | --- | --- |
| τ | 0.28 | 0.40 | contrast pair |
| decay λ | 0.10 | 0.06 | unfitted |
| embellish | 0.18 | 0.05 | contrast pair |
| disgust gain | 0.05 | 0.16 | contrast pair |
| ground_min_overlap | 0.18 | 0.18 | Jaccard vs core |
| ground_strikes | 3 | 3 | two free misses, then rewrite |
| narrator_firmness | 0.42 | 0.72 | blend strength |

All of these are knobs. See [PARAMETERS.md](PARAMETERS.md).

**Encode**

```
S = w_a·A + w_n·N + w_s·R + w_u·U + w_g·G − w_r·Red
```

If `S < τ` and permanence < 0.8: nothing is stored.

If the caller sends no affect: lexicon (FR+EN), then identity paint, then `Narrator::interpret` when an HTTP narrator is set. The interpreter sees the live sentence and living axioms only.

**Recall.** Small top-k. Mix embedding, lexicon, mood, access count. Each recall can cost fidelity and shift valence (`DriftKind::Reinterpret`). Overlap vs core below `ground_min_overlap` is a miss. `hold = narrator_firmness × importance`. Low hold: no ceiling on warp. High hold: after enough misses, blend gist toward a core-facing rewrite (`DriftKind::Ground`). Latent traces are not replayed as scenes.

**Sleep.** Anchor, decay detail, drop unused disgust, bounded rewrite, merge (two strong anchors do not merge), ladder, re-anchor. No LLM required.

---

## Layers

```
IDENTITY     who_am_i: traits, then beliefs, then motifs
    ↑
TRAITS       two aligned living beliefs
    ↑
BELIEFS      ≥ 3 traces, one schema
    ↑
MOTIFS       2 traces, one schema
    ↑
TRACES       gist + core + affect, or latent charge without scene
    ↑
EXPERIENCE   live sentence
```

A motif does not replace a strong belief. A weak belief (strength < 0.36) can. Two anecdotes are not a trait.

---

## Levers

**Detail decay.** Core is set at encode. Detail loses precision with time and disuse. Coefficients are implementation choices, not a model of human Ebbinghaus.

**Rewrite.** Neighbors by schema or cosine. Keep the core, keep one detail, drop the rest. Anchor ≥ 0.88 skips rewrite.

**Anchor.** Raised on high-intensity self events and again if the trace supports a living axiom. Slows decay and reinterpretation.

Fingerprint is a lab metric on the book. It is not a personality score.

---

## Surface

Rust 1.75. No Cargo crates. SQLite (prepared statements, `BEGIN IMMEDIATE`) or flat `SELMEM1`. HTTP daemon + UI. `/health` does not take the memory lock. Auth: `Authorization: Bearer` only.

```
GET  /health /who /lineage /mood /profile /audit /talk
POST /live /remember /speak /turn /sleep /save /profile /talk/clear
```

```bash
./run.sh run --release --bin selmemd -- \
  --bind 0.0.0.0:7420 --path claire.db --name Claire \
  --profile tender --ground-overlap 0.18 --ground-strikes 3 --narrator-firmness 0.42 --token secret \
  --llm https://api.x.ai/v1/chat/completions \
  --model grok-4.3 --api-key "$SELMEM_API_KEY"
```

No `--llm`: rule narrator + hashed vectors.

Same keys in a `.selmem` file in the working directory (`llm=`, `model=`, `api_key=`, `reasoning=`). Flags and `SELMEM_*` env override the file. A vault file starts with `SELMEM1` and is not config.

Ollama: `--llm http://127.0.0.1:11434/v1/chat/completions --model llama3`.

xAI: `reasoning=none` unless you want reasoning tokens.

The model sees gist, core, schema, affect, fidelity, mood, living axioms. Not the archive.

---

## Conclusions from the benches

Tables, scripts, and how to replay a cell: **[experiments/REPORT.md](experiments/REPORT.md)**.

A high-salience hour can enter one clone’s book and stay out of the other’s (`τ`). A length-matched dull hour does not. The resulting fingerprint gap survives eight identical later hours (C2 S/N Δfp = 0.117, C2 S/S Δfp = 0.042, C0 = 0). Encode is on the organ, so Δfp does not vary across Grok pairs.

Wording is the part that needed n = 10. On a probe that never names T₀, Grok names the cancellation on the marked SelMem side after that hour has left a last-k=8 window (10 / 10 S/N, 9 / 10 S/S at post+8). C0 and C1 k=8 do not (0 / 10). While T₀ is still in the window, last-k names it at least as often as SelMem (9–10 / 10 at T₀). The difference is eviction, not “having seen the sentence once”. Four lines from pair `001` are in the report (C0 procedure, C1 last mail, C2 A “this injustice repeating”, C2 B a written correction). The live replies were French; the report quotes them in English.

Speak distance cannot carry the claim: two empty books already sit at ~0.6. Sleep ablation, erasure, and split lives were measured on `RuleNarrator` only. One seed. No human ratings. Not a creativity claim.

What is still open: a scored creative grid, a few blind human judges on the post+8 conflict probe, a second seed if those disagree.

---

## Status

`cargo test` covers: dull drop, world channel pinned, tender/austere split, core vs detail, anchors, axiom succession, motif ≠ trait, identity paint, reinterpret, grounding blend, fading warp, latent residue, persist round-trip, merge + extinguish in one night.

Bench, not only unit tests: trivia fades, repeated aversion does not; split lives stay apart; on v0.1 × 10 Grok pairs the book gap holds and, after last-k=8 evicts T₀, only C2 A still names it. Full tables: [experiments/REPORT.md](experiments/REPORT.md).

Missing: scored creative grid, human ratings, more than one seed, learned layers (still rules), fitted constants. Core is a 12-word compress unless an HTTP narrator proposes one after the gate and a lexical filter accepts it. `--embed` changes neighborhood only. Without HTTP, `interpret` is lexicon + paint.

Two processes on one `.db` will collide. Anchors are decay brakes, not an ethics layer.

MIT. [github.com/jbsalles/Selmem](https://github.com/jbsalles/Selmem)
