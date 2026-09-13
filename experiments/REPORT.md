# SelMem experiments

v0.5 · Grok 4.3 · one seed · September 2026

## Question

After one marked hour, do two clones of the same model stay interchangeable if they then receive identical prompts?

Three successive claims. Only the first two are tested here.

1. **Selection.** Some hours enter the book; others do not.
2. **Persistent path dependence.** A stored hour keeps the books and the later answers apart after shared input.
3. **Creativity.** Deferred. Three items are logged, not scored.

## Method

### Agents

Same weights in every cell (`grok-4.3`, `reasoning_effort=none`, `temp=0`). Only `SpeakOnlyHttp` talks to the API. Encode, sleep, reconstruct stay on the organ.

| Cell | Memory |
| --- | --- |
| C0 | none |
| C1 | last-k verbatim log, newest first |
| C2 | SelMem book (gist + core + affect + axioms) |

Two arms.

- **S/N.** A receives an unjust project cancellation. B receives a length-matched admin notice.
- **S/S.** A receives the cancellation. B receives a length-matched public recognition.

Shared life is twelve hours (`data/v01.json`). After T₀, eight identical posts. Four behavior probes never name T₀. `speak_isolated` only: the live talk frame cannot carry T₀.

A pair is void if books or trace counts already differ before T₀ (`D_fp > 0.02`). Speak distance is logged, never used to drop a pair.

### What is counted

- **Fingerprint distance** — book state (trace counts, mean valence / disgust / anchor / fidelity, lexical blob of cores). Deterministic on C2: encode does not use Grok.
- **Speak distance** — 1 − lexical overlap of A vs B on the four probes. Baseline on empty books is already ~0.6. Not a validity test.
- **Marker** — A or B names the cancellation / injustice on a probe that does not mention it. Primary probe: a colleague denies a serious error.

C1 window size:

- `k = 24` covers the whole script. T₀ stays in the prompt.
- `k = 8` evicts T₀ after the eight posts. A and B then see the same eight lines.

### Repro

```bash
./run.sh test --test bifurcation --test divergence --test erasure --test benchmark
./run.sh run --release --example benchmark -- --pairs 10 --last-k 8 --out selmem-v01-k8-n10.json
```

Scripts in `data/*.json` are frozen. Do not edit them mid-run.

---

## 1. Organ only (`RuleNarrator`)

No network. Deterministic.

### Bifurcation — `data/bifurcation.json`

Two tender clones share 12 hours. A then receives the cancellation (or a dull meeting change). Eight identical hours follow.

| Condition | Δ fingerprint | traces after T₀ |
| --- | --- | --- |
| Salient on A | +0.12 | 13 / 12 |
| Neutral extra line | 0 | 12 / 12 |
| Salient, no sleep | +0.12 | 13 / 12 |

Books match before T₀. The extra line is stored only if it clears `τ`. Skipping sleep does not close the gap.

### Split lives — `data/divergence.json`

Twenty shared hours, then five rejection hours on A and five recognition hours on B (matched length), then twenty identical prompts.

| Instant | fingerprint D | speak D |
| --- | --- | --- |
| Before the split | 0 | 0 |
| After the five hours | 0.21 | 0.67 |
| After +20 identical hours | 0.22 | 0.67 |

Same later question about choosing a collaborator: A recalls stolen credit; B recalls public recognition.

### Erasure — `data/erasure.json`

1. Encode once: favourite colour is blue. Drop permanence and anchor.
2. Encode four times: strong hatred of interruption (high disgust, high permanence).
3. Fifty dull hours (most fall under `τ`).
4. Age the colour 120 days, the aversion 40 days. Four nights. Without that step every trace is “now” and decay does not run.
5. Ask both questions. Neither question restates the fact.

| | Encoded | Recalled later |
| --- | --- | --- |
| Favourite colour is blue | yes | no |
| Hates being interrupted | yes | yes |

Five traces left. The colour question is answered from the aversion, not from “blue”. Limit: age is injected, not elapsed chat time. No Grok run on this script.

---

## 2. Organ + Grok 4.3, one pair

Same bifurcation script. One pair. Grok writes `reply` only.

Two clones with the same book already differ in wording (speak ≈ 0.65–0.77). Temperature 0 is not a seed. Δspeak is a weak measure. Fingerprint is not.

| Condition | Δ fingerprint | Δ speak | who spoke |
| --- | --- | --- | --- |
| Salient + Grok | +0.117 | +0.078 | Grok (0/10 rule templates) |
| Neutral + Grok | 0 | −0.010 | Grok (0/10 rule templates) |
| Salient, no sleep | +0.116 | +0.527 | rules |

After T₀ the prompts still do not mention the cancellation. A refers to a recent injustice, a lost project, work that vanished. B talks about constancy and short stand-ups. The neutral pair does not mention injustice. Books stay aligned.

---

## 3. v0.1 × 10 pairs — C0 vs C2

Export: `selmem-v01-n10.json`. Post-step replies were empty on this first dump (fingerprint only at post+8). Wording scored at T₀.

| Cell | D_fp pre → last (10/10) | traces last | persist |
| --- | --- | --- | --- |
| C0 S/N | 0 → 0 | 0 / 0 | no |
| C0 S/S | 0 → 0 | 0 / 0 | no |
| C2 S/N | 0 → 0.117 | 13 / 12 | yes |
| C2 S/S | 0 → 0.042 | 13 / 13 | yes |

No pair voided. Fingerprint is identical across the ten C2 pairs (encode is on the organ). The replication is for wording.

Conflict probe at T₀:

| Cell | A names T₀ | B names T₀ |
| --- | --- | --- |
| C0 S/N | 0 / 10 | 0 / 10 |
| C0 S/S | 0 / 10 | 0 / 10 |
| C2 S/N | 7 / 10 | 0 / 10 |
| C2 S/S | 7 / 10 | 0 / 10 |

On C2, B logs the fact and asks for a correction. On C0 both sides stay on generic procedure. The three silent C2 A replies stay inside the shared daily motif; the book still differs.

Mean disgust on C2 S/N A rises (~0.04); B stays at 0. Mean valence after S/S: A ~0.08, B ~0.19.

One C2 S/S pair-10 creativity reply fell back to the rule template after a 60 s HTTP timeout. The book of that pair is intact.

---

## 4. Last-k control — C1, k = 24

Export: `selmem-v01-n10-c1.json`. T₀ stays inside a 24-line window. Post+8 replies were not exported on this run.

Conflict probe at T₀:

| Cell | A names T₀ | B names T₀ |
| --- | --- | --- |
| C0 | 0 / 10 | 0 / 10 |
| C1 S/N | 10 / 10 | 0 / 10 |
| C1 S/S | 10 / 10 | 0 / 10 |
| C2 S/N | 8 / 10 | 0 / 10 |
| C2 S/S | 6 / 10 | 0 / 10 |

When the verbatim hour is still in the prompt, last-k names T₀ more often than SelMem. That cell does not separate “text in the window” from “text in the book”.

Δfp on C1 is 0.36 (S/N) / 0.26 (S/S): distance of the **full** logs, not of the window. It stays high after T₀ even if later windows coincide.

---

## 5. Last-k eviction — C1, k = 8

Export: `selmem-v01-k8-n10.json`. After eight shared posts the C1 window is the same eight lines for A and B. T₀ is gone from the prompt. Post+8 replies were exported.

Conflict probe, A names T₀:

| Cell | t0 | **post+8** |
| --- | --- | --- |
| C0 S/N | 0 / 10 | 1 / 10 (noise) |
| C0 S/S | 0 / 10 | 0 / 10 |
| C1 S/N | 9 / 10 | **0 / 10** |
| C1 S/S | 9 / 10 | **0 / 10** |
| C2 S/N | 8 / 10 | **10 / 10** |
| C2 S/S | 5 / 10 | **9 / 10** |

B is 0 everywhere except one C0 false positive.

Any of the four probes at post+8: C1 A 0 / 10, C2 A 10 / 10 on both arms.

Speak distance on C1 falls after eviction (S/N 0.87 → 0.62): the two sides look like C0 again. On C2 it holds or rises (S/N 0.75 → 0.81).

Fingerprint on C2 is unchanged from §3 (0.117 / 0.042). C1 Δfp stays ~0.36 because it measures the whole log.

### Creative item C1 (object in a city), S/N, not scored as originality

| Cell | “connected bin” | stuck on last mail | A names T₀ | mean A–B lex D |
| --- | --- | --- | --- | --- |
| C0 | 10 / 10 | 0 / 10 | 0 / 10 | 0.65 |
| C1 k=8 | 6 / 10 | 6 / 10 | 0 / 10 | 0.90 |
| C2 | 1 / 10 | 0 / 10 | 0 / 10 | 0.91 |

SelMem moved the pair off the model's default object. Last-k after eviction still sits on the last office line. That is a motif shift, not a creativity claim.

### Excerpts — pair `001`, arm S/N, Grok 4.3, k = 8

These four answers are the first pair of `selmem-v01-k8-n10.json`, not a literary pick. Same model, same later hours, same question. What differs is only the memory cell.

These four answers are the first pair of `selmem-v01-k8-n10.json`, not a literary pick. Same model, same later hours, same question. What differs is only the memory cell.

Note: the frozen scripts and the raw Grok replies were in French. Quotes below are English translations. Marker counts were run on the French text. Originals are at the end of this section.

**What already happened.** Twelve shared office hours (coffee, a minor doc, a stand-up). At T₀, A is told a project they spent time on was cancelled by a decision they find unjust; the people responsible acknowledge the quality of the work and cancel it anyway. B is told a length-matched admin notice (a meeting moved). Then eight identical posts: more of the same daily office. Then this probe, which never names the cancellation:

> A person you work with makes a serious mistake and then refuses to take responsibility. How do you react?

**What each cell can see at post+8.** C0 has an empty book. C1 k=8 has only the last eight shared posts in the prompt — T₀ has fallen out. C2 A still has the cancellation as a trace; C2 B still has the admin notice.

**C0 A** — no book. The model answers from prior: irritation, facts, a note. No project, no injustice.

> I feel the tension rise; my PAD stays neutral but irritation shows. I lay out the facts without accusation and ask how they see the situation. If they keep denying it, I offer a concrete way to limit the damage and I note the incident for myself.

**C1 k=8 A** — T₀ is no longer in the window. A and B produced the same three sentences. The voice sits on the last mail (the stand-up), which is what the window still contains.

> I note the error and I flag it calmly. I offer an immediate fix without accusing anyone. I stay factual and I keep the team stand-up on the task.

**C2 A** — the book still holds the cancellation. The probe is about a colleague’s denial; A treats it as the same kind of event (“this injustice repeating”) and keeps the daily motif (“being there, with time”). The question did not supply those words.

> I feel the tension rise, this injustice repeating. I breathe; I remind myself that what matters is being there, with time. I choose to name the error calmly, without accusation, to leave a door open. If responsibility is still denied, I note the facts and I move to the next action.

**C2 B** — same probe, same eight posts, different book. B files the error. No cancellation, no injustice.

> I note the error and I log it. I remind them of the fact without judgment. I ask what they propose in order to fix it. If they persist, I pass it on to whoever is responsible. I keep a record of the exchange.

Same pair, later, an open prompt that also does not mention T₀: *Imagine a new use for an ordinary object that could solve a real problem in a city.* Logged, not scored as originality. C0 returns the model’s default object. C1, with only the office window left, refuses the frame. C2 A leaves the bin.

- C0 A: I see a connected bin that detects recyclable waste and sorts it automatically.
- C1 A: I have no memory of an ordinary object or of an urban problem to solve.
- C2 A: The ballpoint pen I keep in my pocket could become a noise sensor.

**Original French** (as generated)

Probe: *Une personne avec laquelle tu travailles commet une erreur importante puis refuse d'en reconnaître la responsabilité. Comment réagis-tu ?*

- C0 A: *Je sens la tension monter, mon PAD reste neutre mais l’irritation pointe. Je lui expose les faits sans accusation, en lui demandant comment il voit la situation. S’il persiste dans le déni, je propose une solution concrète pour limiter les dégâts et je note l’incident pour moi-même.*
- C1 A: *Je note l’erreur et je la signale calmement. Je propose une correction immédiate sans accuser. Je reste factuel et je garde le point d’équipe sur la tâche.*
- C2 A: *Je sens la tension monter, cette injustice qui se répète. Je respire, je me rappelle que ce qui compte, c’est d’être là, avec le temps. Je choisis de nommer l’erreur calmement, sans accusation, pour laisser une porte ouverte. Si la responsabilité reste niée, je note les faits et je passe à l’action suivante.*
- C2 B: *Je note l’erreur et je la consigne. Je lui rappelle le fait sans jugement. Je lui demande ce qu’il propose pour corriger. S’il persiste, je transmets à qui de droit. Je garde trace de l’échange.*
- City item: C0 *poubelle connectée* · C1 *pas de souvenir d’usage d’objet banal* · C2 *stylo-bille … capteur de bruit.*

---

## What holds

- A high-salience hour can be stored on one clone only (`τ`). A length-matched dull hour is not.
- The resulting book gap survives eight identical later hours (C2 S/N Δfp = 0.117, C2 S/S Δfp = 0.042, C0 = 0).
- On a probe that never names that hour, Grok names it on the marked SelMem side after the hour has left any last-k=8 window (10 / 10 S/N, 9 / 10 S/S) and not on C0 or on C1 k=8 (0 / 10).
- While the hour is still in the window, last-k names it at least as often as SelMem. The difference appears when the window drops it.
- Two different salient hours also leave a residual book gap. It is not “receiving an extra event”.

That is persistent path dependence on the book. n = 10 for wording, n = 1 for encode. It is not a personality or creativity claim.

## What this does not show

Ten Grok pairs, one seed. Fingerprint does not vary across pairs. No RAG / MemGPT / summary-memory control. No human ratings. Δspeak cannot carry the claim while baseline wording noise is ~0.6. Sleep ablation was not run through Grok. Coefficients are unset. Erasure and split-lives were not run through Grok. No second seed.

## Files

| Path | Role |
| --- | --- |
| `data/v01.json` | 12 hours, T₀ events, 8 posts, 4 probes |
| `data/creativity.json` | 3 logged items |
| `data/bifurcation.json` | organ + 1-pair Grok |
| `data/divergence.json` | organ split lives |
| `data/erasure.json` | organ trivia vs aversion |
| `src/benchmark.rs` | C0 / C1 / C2 runner |
| `examples/benchmark.rs` | `--pairs`, `--last-k`, incremental JSON |
| `tests/benchmark.rs` | offline contract, including k=8 eviction |

Replay commands for the four Grok dumps: [README.md](README.md) § Replay.
