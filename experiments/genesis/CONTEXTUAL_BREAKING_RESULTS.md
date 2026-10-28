# GENESIS — Contextual-memory breaking points (October 28, 2026)

**Status:** Completed exploratory Grok 4.3 pilot; 512/512 recorded choices, 512 unique trial IDs, 512 valid answers. Model request was logged with `reasoning_effort=none`, `temperature=0`; no provider-side mode attestation was archived. Synthetic, two-domain follow-up of [longlife v4](CONTEXTUAL_LONGLIFE_V4.md), **not** a new independent replication or the 12-domain confirmatory experiment.

## Setup and provenance

- Histories: `workspace` and `dining`; two mirrored histories per domain, with conditional action patterns for `company=alone` versus `company=friends`.
- Loads: 240 and 960 unrelated observations per history, in addition to 14 task-related actions; every budget reused the same qualified stored life at its load.
- Context limits: **8, 16, 32, 64 whitespace-delimited words** of delivered memory (caps, not equal actual lengths). All arms saw the same held-out choice formulations and option ordering within the same cell.
- Arms: qualified contextual self-belief (`on`), general self-belief only (`off`), query-independent deterministic frequency summary (`summary`), and query-time lexical raw-record retrieval (`retrieval`). No semantic RAG, LLM summary, no-pattern control or independent judge was included.
- The original 16/16 v4 pilot and the sweep's 512 decisions must **not** be pooled as independent trials; changes to load, wording, and selected controls were made across exploratory iterations.

**Archive:** `breaking-pilot-001.tar.gz` (full run data supplied locally; not bundled in the repository). SHA-256: `43e06482ca027cb36b3d20b53aa98bbac9f5b851167464b7ca8dbee631007eda`. The archive contains `choices.jsonl`, `payloads.jsonl`, `summary.jsonl`, `manifest.json`, `actor_config.json`, `curve.csv`, `comparison.csv`, qualified snapshots and organ diagnostics. The repository includes only compact [manifest](../evidence/contextual/breaking-pilot-001/manifest.json), [actor config](../evidence/contextual/breaking-pilot-001/actor_config.json), [curve.csv](../evidence/contextual/breaking-pilot-001/curve.csv) and [comparison.csv](../evidence/contextual/breaking-pilot-001/comparison.csv). Aggregate files alone do not support a full per-probe audit; supply and verify the separately held archive for that.

## Observed accuracy by memory budget

Totals pool both distractor loads: **32 choices per arm per budget**. Percentages count invalid output as wrong (none occurred). Repeated choices share two semantic domains; 32 rows are **not** 32 statistically independent agents.

| Memory cap (words) | SelMem ON | SelMem OFF | Frequency summary | Lexical retrieval |
| ---: | ---: | ---: | ---: | ---: |
| 8  | 17/32 (53.13%) | 15/32 (46.88%) | 14/32 (43.75%) | 16/32 (50.00%) |
| 16 | 24/32 (75.00%) | 17/32 (53.13%) | 14/32 (43.75%) | 16/32 (50.00%) |
| 32 | **28/32 (87.50%)** | 12/32 (37.50%) | 14/32 (43.75%) | 18/32 (56.25%) |
| 64 | **28/32 (87.50%)** | 12/32 (37.50%) | 14/32 (43.75%) | 18/32 (56.25%) |

At 32/64 words, ON is **+31.25 percentage points** over lexical retrieval and **+43.75 points** over the frequency summary. These are *descriptive gaps*, not confidence bounds or causally identified mechanism effects. Unlike the earlier v4 pilot, ON did **not** score 16/16 per load in this follow-up: it scored **14/16** at 32 and 64 words for each load.

## Results separated by distractor load

Each cell: 16 choices per arm (two domains, mirrored contexts and wording/order cells).

| Distractors | Cap | ON | OFF | Summary | Lexical |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 240 | 8  | 8/16  | 7/16 | 7/16 | 8/16 |
| 240 | 16 | 11/16 | 9/16 | 7/16 | 8/16 |
| 240 | 32 | 14/16 | 6/16 | 7/16 | 9/16 |
| 240 | 64 | 14/16 | 6/16 | 7/16 | 9/16 |
| 960 | 8  | 9/16  | 8/16 | 7/16 | 8/16 |
| 960 | 16 | 13/16 | 8/16 | 7/16 | 8/16 |
| 960 | 32 | 14/16 | 6/16 | 7/16 | 9/16 |
| 960 | 64 | 14/16 | 6/16 | 7/16 | 9/16 |

### Interpretation — what the curve actually measures

1. **Serialization breakpoint, not necessarily an intrinsic memory breakpoint.** The ON memo is **22 words when complete**. At 8 words it consists only of a prefix such as `Fallible past self-hypothesis: Observed action pattern under company=alone:`; the predicted action is not delivered. At 16 words the action starts to appear. From 32 to 64 words the actual ON payload does not grow. The higher score at 32 words may largely reflect complete delivery of a short prediction, not an emergent compression threshold.
2. **The distractor manipulation does not perturb most selected payloads.** Across **64/64 matched load comparisons for each arm**, ON, OFF and lexical memory texts were byte-for-byte identical between 240 and 960 distractors. Summary text changed in all 64. This is evidence of selector stability against *these distractors*, not a model-visible 720-record distraction experiment. Despite `temperature=0`, ON actor answers agreed between loads for 61/64 matched cells, OFF 62/64, lexical and summary 64/64: non-deterministic response variation should not be mistaken for a distractor effect.
3. **Caps are not equal exposure or equal information.** At 32/64-word limits the effective mean words at 32/64 were ON 22/22, OFF 29/29, retrieval 32/50, summary 32/64. ON received a directly predicted context-qualified action, while deterministic baselines received observations or frequencies. The contrast includes a representation and task-alignment advantage; reconstructive distortion was not isolated.
4. **Substantive errors remain.** At 32/64, the two distinct SelMem ON failures occur in the `dining` domain (alternative wording, one option order): the stored rule favors a lively crowded option, but the actor chooses a peaceful bistro. They repeat across the two loads and both budgets. Workspace scores 16/16 across loads at 32 and 64; dining scores 12/16.
5. **Experimental scope is limited.** Only two semantic domains, synthetic observations and simulator-supplied contextual facets, scripted proposition witness, deterministic summary and lexical comparators, no-pattern absent, no independent judge, and no matched semantic RAG or LLM summarizer. The previously preregistered v4 12-domain requirement is **not met**; it is not inherited by this adaptive grid.

## Next tests

- **Normalize serialization**: short neutral wrapper equal for each arm; budget the semantic content, retain complete statements where possible, and record the facts actually delivered. Extend budgets around the observed cliff (12, 16, 20, 24, 32, 48, 64, 128).
- **No-pattern and confounded histories**: balance marginal counts without a stable condition–action association and measure false qualification/abstention and choices. Include a perfectly confounded company/time history.
- **Order and distractor interventions**: independently permute arrival order and add near-target distractors so selection and delivered memory can genuinely change; evaluate whether the system learns spurious scopes.
- **Strong, budget-matched controls**: semantic/vector RAG with the same scope metadata, rolling and task-aware LLM summaries, and a scope-stripped hypothesis of equal length; track build cost, delivered words and actor calls.
- **Freeze a confirmatory protocol** across 12 domains and fresh independently generated histories. Analyze domains as the unit, publish ties, invalid outputs and all failed qualifications before making general claims.

## Reproduction

With the sweep extension installed in the Rust binary (see [protocol](CONTEXTUAL_BREAKING_POINTS.md)), an **offline replay** of the pilot uses no model calls:

```bash
./run.sh run --release --bin selmem-contextual-longlife -- \
  --out experiments/genesis/results/breaking-pilot-replay-new \
  --pairs 2 --sweep-loads 240,960 --sweep-budgets 8,16,32,64
```

A separate *new* paid collection additionally requires `--actor --provider grok --model grok-4.3 --reasoning none --max-http-calls N --resume` and a valid API key, applied after offline preparation. Model-side numerical reproducibility is not guaranteed; compare per-cell outcomes rather than replacing the recorded pilot. Always use a new output directory for replications. The full source snapshots and choice payloads were retained in the locally supplied archive, not checked into this documentation patch.

**Claim boundary:** This pilot supports a *specific advantage of an explicitly delivered contextual action hypothesis over the tested deterministic controls* under a short memory ceiling. It does **not** establish superiority to semantic RAG, an intrinsic 32-word memory threshold, a causal effect of more distractors, autonomous preference formation, durable individuality, or greater creativity.
