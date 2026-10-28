# GENESIS V4 — contextual-memory breaking-point map

**Status: original prospective protocol, now accompanied by a completed exploratory pilot (October 28, 2026).** See [512-decision results](CONTEXTUAL_BREAKING_RESULTS.md). This extends the already recorded [V4](CONTEXTUAL_LONGLIFE_V4.md) synthetic test. It is **not** an independent confirmation of V4.

## Question and falsification

Does context-qualified memory preserve held-out choice accuracy as the **actor-visible memory word ceiling** shrinks, compared with query-independent frequency summaries and query-time lexical retrieval? Is the effect primarily a memory-budget threshold, a distractor-load effect, or their interaction?

Precommit to publishing the **entire score-versus-budget curve**, including failed and tied cells, for each distractor load. Ties at any one budget (especially generous budgets) are not by themselves evidence against a benefit *under tighter budgets*. Conversely, a gain limited to one arbitrarily selected cell is not sufficient to claim a robust effect.

Predeclared exploratory axes (modifiable only for a new, labeled experiment):

- Word ceilings: **8, 16, 24, 32, 48, 64, 96, 128**. These are *whitespace-separated words of model-visible memory*, not tokens, input processing cost, or equally filled contexts.
- Distractors per history: **240, 480, 960**, plus 14 relevant decisions. Each load builds new ON/OFF memories, but **every budget reuses the same saved memories for that load**.
- 12 fixed V4 domains, two mirrored histories per domain, two contexts per history, two paraphrases/option orders per context, all actors and controls asked identical held-out questions.
- Arms by default: contextual ON, contextual OFF, frequency summary, lexical retrieval. Optional `--sweep-arms on,off,episodes,summary,retrieval,bare` restores all six V4 arms. The source episode observations are identical between ON and OFF. Summary and retrieval see the full stream, without gold IDs.

All histories use the **original V4** synthetic contextual company dimension (`alone` vs `friends`), action templates, structured observation attestations, scripted proposition scorer, qualification gates, background distractors, baseline selectors and actor prompt. This tests the existing organ, **not** a new algorithm for each budget.

## Primary reporting

1. **Breaking-point surface:** correct held-out choices / total responses by budget and distractor load, for each arm. Plot as score-vs-budget lines (one group per load); also report a table of `ON − summary` and `ON − retrieval` at *every* cell.
2. **Effective memory exposure:** delivered word count and fraction truncated per arm and cell. V4 limits the *maximum*, not the exact equal delivered length. Extremely short limits can truncate the contextual statement mid-sentence; treat the resulting failure as **serialization / budget** failure, not necessarily lost memory.
3. **Robustness:** per-domain results with reversed histories and both option orders. Independent unit is the **semantic domain**, not the repeated actor calls. Do not give response-level binomial intervals as if all calls were independent.
4. **Failure points:** earliest ceiling (in the descending sweep) where ON loses concordance, and earliest where a comparator ties or wins. Do not filter out failed contexts or failed qualifications.

There is **no new preregistered significance threshold** here. The 12-domain V4 confirmation criterion cannot be inherited by this new adaptive follow-up. Freeze a separate criterion and run independent, held-out domain replications before a confirmatory claim.

## Running (Rust only)

The sweep is a new mode of the **existing** `selmem-contextual-longlife` binary. There are no dependencies and no Python scripts. The offline step makes **zero model calls**, creates all qualified history snapshots, validates ON/OFF stream equality, and writes all *model-visible payloads* before inference.

```bash
./run.sh test --bin selmem-contextual-longlife

# Cheap smoke / preflight: 2 domains × 2 distractor loads × 4 budgets × 4 arms
./run.sh run --release --bin selmem-contextual-longlife -- \
  --out experiments/genesis/results/contextual-breaking-pilot-001 \
  --pairs 2 --sweep-loads 240,960 --sweep-budgets 8,16,32,64

# Inspect manifest.json, snapshots/load-*/organ.jsonl, payloads.jsonl.
# Then resume the *same frozen plan* with the actor. Each invocation is capped.
export GROK_API_KEY='...'
./run.sh run --release --bin selmem-contextual-longlife -- \
  --out experiments/genesis/results/contextual-breaking-pilot-001 \
  --pairs 2 --sweep-loads 240,960 --sweep-budgets 8,16,32,64 \
  --actor --provider grok --model grok-4.3 --reasoning none --max-http-calls 128 --resume

# Repeat the same command until the run reports completion.
```

The pilot plans **512 actor calls** (2 domains × 8 probes × 4 arms × 4 ceilings × 2 loads). It makes at most 128 API requests per invocation, **not** a fixed currency expenditure. A failed HTTP attempt may not have a saved response; validate provider usage before retrying.

Full offline map (12 domains × 3 loads × 8 ceilings × 4 arms = **9,216 model calls** for a complete actor replay):

```bash
./run.sh run --release --bin selmem-contextual-longlife -- \
  --out experiments/genesis/results/contextual-breaking-full-001 \
  --pairs 12 --sweep-loads 240,480,960 \
  --sweep-budgets 8,16,24,32,48,64,96,128
```

Use `--actor --max-http-calls N --resume` only when explicitly budgeting for the paid run. **Do not launch the complete paid grid casually**: run the pilot and inspect word counts, clipping and control behavior first.

## Artifacts and invariants

- `manifest.json` identifies grids, actor model, arms, and planned calls. On resume it must match **byte for byte**.
- `snapshots/load-N/*.vault`, `snapshots/load-N/organ.jsonl`: persist a single generated ON/OFF pair per history and load. Histories are not rebuilt per budget, nor on resume.
- `payloads.jsonl`: exact actor-visible history strings, selected IDs, held-out questions, word caps, *actual* words, uncapped words, clipping and trial IDs. It is re-derived and verified exactly on resume.
- `choices.jsonl`: append-only, one entry per complete actor response, persisted after each call. Invalid (`not 1 or 2`) responses remain scored as incorrect. Duplicates or unknown trial IDs prevent resume.
- `summary.jsonl`: domain and aggregate score rows (only once every response is recorded).
- `curve.csv`: aggregate scores, mean words and clipped fraction by budget × load × arm.
- `comparison.csv`: ON/summary/retrieval score and differences per cell, including ties and losses.

`--resume` is intentionally strict: no changing loads, budgets, actors, seed, arm selection, or source code's generated payloads mid-run. When the per-invocation HTTP ceiling is hit, the command stops with partial `choices.jsonl` and **does not write final curves**.

## Limitations and next controls

- Synthetic action histories, simulator-provided scope facets, fixed company values, and a scripted witness do **not** demonstrate autonomous scope discovery, subjective preferences, or creativity.
- Summary and lexical arms are deliberately weak **deterministic V4 baselines**, not embedding RAG, LLM summarizers or full-history access. These remain necessary comparisons, ideally matched for total compute and delivered information.
- The lexical selector's tie-breaking and source-ID ordering can create order effects; test independently permuted life streams under fixed marginal counts before causal attribution.
- Add a **no-pattern** history with identical marginal action, context and exposure totals, plus a confounded company/time history: ON must abstain on unsupported exceptions rather than fabricate a plausible pattern. These are next experiments, **not included in the current grid**.
- Shortening an existing English-language belief by blind word truncation can favor early tokens. A later experiment should compare **minimal complete structured encodings** or balanced compression, instead of silently changing V4 during this replication.
