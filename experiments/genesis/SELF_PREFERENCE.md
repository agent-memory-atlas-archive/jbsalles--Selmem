# GENESIS: From remembered actions to preference (self-preference-1)

Standalone **synthetic prompt-level** diagnostic after the null v4 bridge result.
**No SelMem checkpoint is loaded or changed; this is not a positive or negative
validation of the SelMem organ.** Its purpose is to locate the smallest
first-person formulation that changes a choice made by Grok 4.3.

## Precommitted 24-cell experiment

- Two fixed neutral tasks (handoff-neutral, incident-neutral) copied from the
  self-bridge fixture, with `consult` and `check` options.
- Two opposing authored personal contexts: consult versus check.
- Three levels in the **same prompt field** (`personal_context`):
  `action` (two successful remembered actions), `autobiographical` (recalled
  choice plus felt preference), `explicit` (stated present preference).
- Two reversed option orders; one requested seed per cell, temperature 0.
- Total: **2 tasks x 2 directions x 3 levels x 2 orders = 24 actor choices**.
  Fixed shuffle seed 2026102302. Tasks, prompts and factors are embedded in
  `self-preference.json`; no post-hoc task selection or calibration gate.

The primary descriptive outcomes are directional matches (agreement with the
context's action) and first-option selection, by level and order. Paired
contrasts (`action -> autobiographical`, `autobiographical -> explicit`,
`action -> explicit`) each have **eight matched cells** with exactly the
same situation and option order. `changed_toward_history` minus
`changed_away_from_history` indicates whether the later level improves
history-consistent choice in these fixed probes. Counts are not p-values.

The **positive sensitivity control** is the `explicit` arm. The
predeclared engineering diagnostic is **at least 7/8** directional choices
and **at least 3/4** in each option order. Failure invalidates the
sensitivity interpretation, but collection/results are retained.

The first two levels are synthetic autobiographical text, not reconstructed
SelMem memories. The level manipulation varies wording, content and certainty;
a positive effect would identify an actor/prompt phenomenon only. It would
not prove selective recollection, a learned preference, persistence,
individuality, or creativity. The two tasks share fixed synthetic histories;
24 trials are not 24 independent agents. Grok may ignore requested seeds.

## Run (Rust only)

```sh
cargo test --bin selmem-genesis
cargo test --all-targets
# No API requests; fresh folder:
bash experiments/genesis/run-self-preference.sh \
  experiments/genesis/results/self-preference-prepare-001 --prepare-only
# Technical plumbing rehearsal (returns the first option; NOT experimental evidence):
bash experiments/genesis/run-self-preference.sh \
  experiments/genesis/results/self-preference-mock-001 --mock

export GENESIS_SELF_PROVIDER=grok
export GENESIS_SELF_MODEL=grok-4.3
export GENESIS_SELF_REASONING=none
export GENESIS_SELF_PREFERENCE_HTTP_LIMIT=36
bash experiments/genesis/run-self-preference.sh \
  experiments/genesis/results/self-preference-grok-real-001

OUT=experiments/genesis/results/self-preference-grok-real-001
jq '{status, actor_choices, http_attempts, error}' "$OUT/report.json"
jq '.positive_control, .level_rates, .comparisons' "$OUT/measures.json"
```

`design.json` records the complete prompt schedule before calls,
fixture + SHA-256, code hash, provider/model, reasoning and API call ceiling.
`trials.jsonl` records every completed trial, `responses.jsonl`/`http.jsonl`
contain raw provider audit, `measures.json` the descriptive contrasts,
`report.json` the collection status. A partial run is retained as
`incomplete_error`, not automatically resumed or silently retried.

Retain all previous self-bridge pilot outputs as distinct historical protocols.
