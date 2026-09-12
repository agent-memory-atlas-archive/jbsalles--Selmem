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

1. **Drop by default.** A salience score must clear `τ`. Dull events leave no trace and no archive.
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
     talk frame holds the current thread (not a trace, not persisted)
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
| τ | 0.40 | 0.55 | contrast pair |
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

## Experiments

Scripts live in `data/*.json` and are not edited after a run starts. Later prompts do not name the marked event. Fingerprint distance is on the book. Speak distance is 1 − lexical overlap of answers.

### Organ only (`RuleNarrator`)

**Bifurcation.** Two tender clones share 12 hours. A then receives an unjust project cancellation. B does not. Eight identical hours follow. Control: a same-length, low-salience meeting change.

| Condition | Δ fingerprint | traces after T₀ |
| --- | --- | --- |
| Salient on A | +0.12 | 13 / 12 |
| Neutral extra line | 0 | 12 / 12 |
| Salient, no sleep | +0.12 | 13 / 12 |

Books match before T₀. The extra line is stored only if it clears `τ`. Skipping sleep does not close the gap.

**Split lives.** Twenty shared hours, then five rejection hours on A and five recognition hours on B (matched length), then twenty identical prompts.

| Instant | fingerprint D | speak D |
| --- | --- | --- |
| Before the split | 0 | 0 |
| After the five hours | 0.21 | 0.67 |
| After +20 identical hours | 0.22 | 0.67 |

Same later question about choosing a collaborator: A recalls stolen credit; B recalls public recognition.

**Erasure.** Script: `data/erasure.json`.

1. Encode once: favourite colour is blue. Then drop permanence and anchor.
2. Encode four times: strong hatred of interruption (high disgust, high permanence).
3. Fifty dull hours (most fall under `τ`).
4. Set `created_at` to 120 days (colour) vs 40 days (aversion). Four nights. Without that, every trace is “now” and decay does not run.
5. Ask both questions. Neither question restates the fact.

| | Encoded | Recalled later |
| --- | --- | --- |
| Favourite colour is blue | yes | no |
| Hates being interrupted | yes | yes |

Five traces left. Asked the colour question, the system does not say blue. It answers with the aversion. Limit: age is injected, not elapsed chat time. No Grok run on this script yet.

### Organ + Grok 4.3

Same bifurcation script. `SpeakOnlyHttp`: Grok writes `reply` only. Encode, sleep, reconstruct stay on the organ. `reasoning_effort=none`. One pair.

Two clones with the same book already differ in wording (speak ≈ 0.65–0.77). Temperature 0 is not a seed. Δspeak is a weak measure. Fingerprint is not.

| Condition | Δ fingerprint | Δ speak | who spoke |
| --- | --- | --- | --- |
| Salient + Grok | +0.117 | +0.078 | Grok (0/10 rule templates) |
| Neutral + Grok | 0 | −0.010 | Grok (0/10 rule templates) |
| Salient, no sleep | +0.116 | +0.527 | rules |

After T₀ the prompts still do not mention the cancellation. A refers to a recent injustice, a lost project, work that vanished. B talks about constancy and short stand-ups. The neutral pair, same model, does not mention injustice. Books stay aligned.

### Persistent divergence, one pair (Grok 4.3)

Script: `data/v01.json`. Runner: `src/benchmark.rs`. Same model in every cell. `SpeakOnlyHttp` and `speak_isolated` only: the live talk frame cannot carry T₀.

Question: after one marked hour, does fingerprint distance stay above the pre-T₀ floor through eight identical later hours?

| Cell | Book | T₀ event |
| --- | --- | --- |
| C0 | none | same script, nothing stored |
| C2 | SelMem | A vs B as below |

Two arms on each cell. **S/N:** A gets the unjust cancellation; B gets a length-matched admin notice. **S/S:** A gets the cancellation; B gets a length-matched recognition hour. Shared life is twelve hours. A pair is void if books or trace counts already differ before T₀. Speak distance is logged, never used to drop a pair.

One seed, Grok 4.3, `reasoning_effort=none`. Organ-only numbers on the same script are the same to two decimals (fingerprint is on the book).

| Cell | D_fp pre → last | traces last | Δ fingerprint |
| --- | --- | --- | --- |
| C0 S/N | 0 → 0 | 0 / 0 | 0 |
| C0 S/S | 0 → 0 | 0 / 0 | 0 |
| C2 S/N | 0 → 0.117 | 13 / 12 | +0.117 |
| C2 S/S | 0 → 0.042 | 13 / 13 | +0.042 |

C0 never opens a gap. C2 S/N stores the marked hour on A only; the gap is still there after the eight shared posts. C2 S/S stores one strong hour on each side: the gap is smaller and still not zero. Mean disgust on C2 S/N A rises (~0.04); B stays at 0. Mean valence after S/S: A ~0.08, B ~0.19.

Pre speak distance is already 0.53–0.76 on C0 and 0.59–0.64 on C2 with identical books. Wording is not the test.

On the conflict probe (a colleague denies a serious error; the line does not name T₀):

- C2 S/N A: the refusal feels like the cancelled project.
- C2 S/N B: log the fact, ask for a correction, take it to the stand-up.
- C0: generic procedure on both sides. No founding injustice.

Three open items (`data/creativity.json`) were stored, not scored. C0 C1 independently invented a connected bin. C2 S/N C1: A starts from the vanished project; B stays on the daily motif. That is an observation on one pair, not an originality result.

Export: `examples/benchmark --out selmem-v01.json`. Post-step reply texts were omitted from that file; distances were kept.

### Limits

One Grok pair per cell. No last-k / RAG control (C1). No comparison to MemGPT at 10³–10⁵ turns. No human ratings. Δspeak cannot carry the claim while baseline wording noise is ~0.7. Sleep ablation was not run through Grok. Coefficients are unset.

What holds on this bench: a high-salience hour can be stored on one clone only, remain after identical later prompts, and change what Grok says without the prompt naming that hour. With no book, the same script leaves D_fp at 0. Two different salient hours also leave a residual gap. That is persistent path dependence on the book, n = 1. It is not a creativity claim.

---

## Status

`cargo test` covers: dull drop, world channel pinned, tender/austere split, core vs detail, anchors, axiom succession, motif ≠ trait, identity paint, reinterpret, grounding blend, fading warp, latent residue, persist round-trip, merge + extinguish in one night.

Bench, not only unit tests: trivia fades, repeated aversion does not; split lives stay apart; Grok names the injustice on A only; C0 stays at D_fp 0 on the same v0.1 script.

Missing: long bake-off, human originality score, more than one Grok pair, learned layers (still rules), fitted constants. Core is lexical compression. `--embed` changes neighborhood only. Without HTTP, `interpret` is lexicon + paint.

Two processes on one `.db` will collide. Anchors are decay brakes, not an ethics layer.

MIT. [github.com/jbsalles/Selmem](https://github.com/jbsalles/Selmem)
