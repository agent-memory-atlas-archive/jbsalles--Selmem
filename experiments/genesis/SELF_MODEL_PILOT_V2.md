# GENESIS contextual self pilot 2

Historical protocol. For new collection use [pilot 3](SELF_MODEL_PILOT_V3.md),
which adds short real bootstrap diagnostics, qualification gates and individual
source attribution. The local verification below describes the earlier binary.

This protocol supersedes `self-pilot-1` for new collections. Apply this incremental
patch after the first pilot patch. Preserve the old real lot as a diagnostic;
collect a fresh lot from encoding with the corrected binary.

## Corrections

Bootstrap requires two distinct, internally attributed, attested action outcomes
(`StakeKind::Decision`, `Bearer::Self_`). Notes, intentions and promises cannot
supply behavioral evidence. A separate semantic guard validates BOTH proposed
statement and expectation; Unknown, invalid responses and errors reject creation.
Sources are supplied as separate observations with identity and encoding time.
Encoding time is not event chronology; unsupported causal or temporal links
across incidents are prohibited. Without a custom guard, only an exact
conservative summary of repeated identical actions is accepted.

The LLM guard is fallible and may share the narrator's model. Inspect
`self_consolidation_audit`; this is not independent factual verification.
These checks apply to new consolidations, not previously persisted beliefs.
No archives or operation journals recover forgotten foundations.

Explicit context must match stored context. Otherwise activation uses distinctive
words from the stored context, excluding belief prose. A generic word in a belief
cannot override an explicit context mismatch.

Actor readout delivers stored selected episode gists without appended schema
prototypes or support signals. Self traces remain audit-only. Gists are memory
representations, not guaranteed original transcripts.

## Matched protocol

The two histories, four interventions, tasks and three original counterexamples
remain those of [pilot 1](SELF_MODEL_PILOT.md). Each of two requested generation
seeds now has BOTH option orders: 128 choices. Provider seed reproducibility is
not assumed. `paired-measures.json` reports matched flips by seed, order,
condition and phase without significance tests. Two scripted histories are
not 128 independent lives.

A separate `relearning_only` assay copies each after-correction state and supplies
two NEW contrary action observations before one sleep. It tests whether new
roots support a later hypothesis. It cannot change the main checkpoints, actor
contexts or choices. Inspect `relearning/` and `diagnostics.json`. One leftover
action plus a note must not recreate a behavioral belief.

Bootstrap or guard failure and absent effects are meaningful negative findings.
An HTTP failure or exhausted budget produces an incomplete lot. Do not replace
real labels with the finite mock or pool pilot-1 choices with pilot 2.

## Commands (Grok)

Run at the repository root with your existing Grok key in `.selmem`. Use the
same actual model identifier as your previous real lot; the example follows
the repository's Grok default.

```bash
cargo test --test self_model
cargo test --bin selmem-genesis
bash experiments/genesis/run-self-pilot-v2.sh \
  experiments/genesis/results/self-pilot-v2-mock-001 --mock

export GENESIS_SELF_PROVIDER=grok
export GENESIS_SELF_MODEL=grok-4.3
export GENESIS_SELF_HTTP_LIMIT=256
bash experiments/genesis/run-self-pilot-v2.sh \
  experiments/genesis/results/self-pilot-v2-plan-001 --prepare-only

# Inspect design.json, then use a NEW directory:
bash experiments/genesis/run-self-pilot-v2.sh \
  experiments/genesis/results/self-pilot-v2-grok-real-001
```

Prepare-only makes no paid requests. The wrapper archives success or failure
as `OUT_DIR.tar.gz`. The HTTP ceiling includes 128 actor requests, guards,
appraisals and retries. It is neither a monetary cap nor a completion guarantee.
Read `report.json`, then `measures.json`, `paired-measures.json`,
`diagnostics.json`, delivered prompts in `trials.json` and the HTTP audit logs.

## Local verification

The finite mock completed 128 choices with zero HTTP attempts and two qualified
bootstrap histories. It checks matching, raw-gist delivery, context isolation,
retirement and separate new-action relearning. Mock choices select the first
offered option and cannot demonstrate LLM individuality. No paid collection
was run in the development environment.
