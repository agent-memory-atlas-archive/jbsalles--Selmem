# Memory-influence diagnostic

This diagnostic tests whether two controlled histories change open-task answers and candidate selection. Greater history sensitivity is not automatically greater creativity.

Two histories, six tasks, three conditions and two repeats produce 72 final rows:
direct generation with memory, three-candidate selection with memory, and three-candidate selection without memory. The full collection has 144 nominal model calls before retries. Candidate generation, anonymous task-only quality and anonymous memory grounding are separate stages. Runtime behavior is described in [response selection](../../docs/RESPONSE_SELECTION.md).

## Fresh collection

Run from the repository root with the existing private configuration:

```bash
bash experiments/genesis/run-memory-influence.sh collect \
  --out experiments/genesis/results/memory-influence-new \
  --config .selmem --llm grok --model grok-4.3 \
  --max-http-calls 180 --prepare-only
bash experiments/genesis/run-memory-influence.sh collect \
  --out experiments/genesis/results/memory-influence-new \
  --config .selmem --llm grok --model grok-4.3 \
  --max-http-calls 180 --resume
```

Preparation creates the controlled source lives and frozen contexts without paid calls. After an interruption, repeat the second command. All HTTP attempts count across resumes. Input cases, settings and implementation must match; a ceiling is not a completion or price guarantee. Use `--mock` in a separate fresh directory for a technical rehearsal.

## Fixed-pool reevaluation

To reproduce the intervention represented by historical 005, the complete original 004 source archive is required:

```bash
bash experiments/genesis/run-memory-influence.sh reevaluate \
  --cases-from experiments/genesis/results/memory-influence-004 \
  --out experiments/genesis/results/memory-influence-grounding-new \
  --reuse-quality --config .selmem --llm grok --model grok-4.3 \
  --max-http-calls 48 --prepare-only
# Repeat with --resume instead of --prepare-only to make paid calls.
```

This reuses validated source candidate pools, direct answers and 48 task-only quality results. It plans 24 new grounding calls. The recorded 005 run used 33 attempts including validation retries. It does not represent a fresh full-pipeline replication. Without `--reuse-quality`, reevaluation also recollects quality judgments. Changed/missing/inconsistent reuse evidence is refused before fallback to paid work.

The [reproduction policy](../REPRODUCIBILITY.md) explains recovery with the original source revision. Legacy upgrade switches have been removed. Do not use old patch numbers or force-edited hashes as a fresh-run recipe.

## Independent rating

```bash
bash experiments/genesis/run-memory-influence.sh template \
  --out experiments/genesis/results/memory-influence-new
bash experiments/genesis/run-memory-influence.sh analyze \
  --out experiments/genesis/results/memory-influence-new \
  --ratings /path/to/judge-a.json --ratings /path/to/judge-b.json
```

Give each judge only `blind-proposals.json` and an independent copy of the blank template. Rate utility, feasibility and originality 0–4, task violations, rationale and the template's five behavioral dimensions. Judge all answers before exchanging notes. Two names do not prove actual independence. The analyzer requires complete coverage and refuses mock evidence.

The model's quality/grounding outputs are pipeline diagnostics, not independent creativity ratings. The weight sweep holds candidate generation fixed and measures selection only. The 005 default moderate-weight sweep changes 3 of 24 memory selections; two changes lose utility under the same model's quality ratings and one ties. Mean originality across those three changes is unchanged. Full findings and exported audits are in [CREATIVITY_RESULTS.md](../CREATIVITY_RESULTS.md).
