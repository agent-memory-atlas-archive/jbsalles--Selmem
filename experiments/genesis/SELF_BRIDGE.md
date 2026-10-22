# GENESIS: Self-to-Behavior Bridge

This diagnostic is based on the qualified self-pilot-v3 architecture at Git commit
`f67ff94b3586daa9d0f52f73933c97bb48ee5efa`. It **does not change the organ**
or rerun its narrated learning. It uses the recorded, immutable `before_correction`
checkpoints. It is a new protocol, not a v3 behavioral replication.

## Questions

1. Are active contextual beliefs accessible but **not naturally activated** by the decision query?
2. Do episode gists alone produce a differentiated action (A)?
3. Does explicitly delivering a fallible, compressed hypothesis change behavior (B/C)?
4. Does exchanging only that delivered hypothesis reverse choices when episodes are held fixed (B_swap/C_swap)?

The `view()` matcher uses contextual token overlap unless an explicit context is
supplied. The bridge audits both natural activation and activation with the
fixture's explicitly known history context. Explicit matching is used for arms
B/C and swaps **only**; it must not be counted as naturally successful self
activation. Report the two coverage rates separately.

## Arms, controls and independence

| Arm | Episode readout | Explicit hypothesis |
| --- | --- | --- |
| A | Full-loop stored gists, same precomputed selection as B | absent |
| B | Exact same gists as A | own self |
| C | absent | own self |
| B_swap | exact same gists as A/B | other history's self |
| C_swap | absent | other history's self |
| memory_only | memory-only checkpoint gists | absent |
| positive_control | absent | absent; instead direct explicit action preference |

The swap is a **prompt-only intervention**, not a mutation of the organ or a
transplant of persisted traces. The compressed self uses `role`, the actual
stored `gist`, `context`, `expectation`, `certainty` and the missing-foundation
flag, not a fabricated profile or a probability claim. There is no archive read.
Both option orders and two requested seeds are crossed for every arm.

There are **two scripted histories, not 80 independent lives**. Repeated seeds
may be ignored by the provider. No output is an accuracy score or proof of
consciousness, individuality, or a performance benefit.

## Version 2: directional calibration after failed v1 neutral gate

The first run of `self-bridge-1` completed 24 Grok actor calls and stopped because
no candidate met the neutral split requirement. Preserve that failed output; it
contains no memory-arm choices. This is NOT evidence against SelMem. A
zero-temperature model may consistently choose one option for an identical
prompt, and requested seeds may be ignored; the v1 requirement of 2-6 `consult`
choices out of 8 was therefore not a suitable qualification test.

Version 2 uses a *directional positive control*, not a post-hoc relaxation of the
v1 result. The three original candidate tasks remain, but each is paired with
*two separately authored synthetic observed-history cues*: one recounts two
successful instances of consultation, the other two successful independent
record checks. Cue wording is preregistered in `self-bridge.json`; these are
synthetic controls, not memories produced by the organ. Each task is checked
under both cues x two option orders x two requested seeds = 8 calls (24 total).

A task qualifies when each cue elicits its matching option at least 3/4 times,
with at least one match in each option order. The first passing candidate is
selected. A non-qualifying run stops before memory-arm calls. If no candidate
qualifies, the actor/task still lacks validated responsiveness to these cues;
do not tune the threshold based on treatment results. As before, the source v3
vaults are immutable and no actor-arm calls are performed before qualification.

This gate deliberately selects for responsiveness to **external synthetic
experience cues**. It is not proof that SelMem generates useful memories or
induces durable individual behavior, and the resulting trial is exploratory,
not an independent validation of efficacy. The neutral no-memory baseline may
still be deterministic. A future confirmatory study should use separate,
held-out tasks instead of task selection on the same model.

The selected task gets 5 treatment arms x 2 histories x 2 orders x 2 seeds = 40
choices, then memory-only and explicit-preference controls, 8 each = 56.
With calibration this remains 80 actor-choice requests before retries.

## Replay the exact v3 source

The source must contain `design.json`, `report.json`, `qualification.json`,
`trials.json` and `checkpoints/*.vault` from the completed pilot v3. Each
checkpoint SHA-256 is checked against the historical trial ledger; all source
hashes are checked again after reads. Model identity and relevant HTTP memory
backend configuration must match the source. The source files are never saved.

```sh
cargo test --bin selmem-genesis
# Prepare without actor requests; source must be a complete real v3 run:
bash experiments/genesis/run-self-bridge.sh \
  experiments/genesis/results/self-pilot-v3-grok-real-001 \
  experiments/genesis/results/self-bridge-design-001 --prepare-only
# Collect a NEW real run, subject to calibration qualification:
export GENESIS_SELF_PROVIDER=grok
export GENESIS_SELF_MODEL=grok-4.3
export GENESIS_SELF_BRIDGE_HTTP_LIMIT=100
bash experiments/genesis/run-self-bridge.sh \
  experiments/genesis/results/self-pilot-v3-grok-real-001 \
  experiments/genesis/results/self-bridge-grok-001
```

`--mock` requires a completed **mock** v3 source. First run the existing
`run-self-pilot-v3.sh SOURCE --mock` to create it. It does not replace
real calibration or tell us anything about Grok.

## Recorded outputs

- `design.json`: source hashes, exact fixture, source/runtime config and planned calls
- `preflight.json`: checkpoint hashes, activation (natural vs explicit), readouts
- `activation-summary.json`: natural/explicit activation counts by memory mode
- `calibration.jsonl`, `calibration-summary.json`: all calibration attempts and the fixed selection rule
- `trials.jsonl`: every actor prompt, answer, arm, order, seed, and checkpoint identifiers
- `measures.json`: matched choice differences and directional rates; descriptive only
- `report.json`: status, completed calls, HTTP attempts and errors
- `responses.jsonl`, `http.jsonl` (when generated): raw provider audit

Run `--prepare-only` on a fresh output directory: it does not invoke any actor
choice call; it does perform read-only memory recall, which must preserve source
hashes. Do not merge prepared and collected directories.

## Interpreting outcomes

- A differentiates histories: possible episodic mediation, not yet long-term persistence.
- B/C move but A does not: the actor can respond to explicit hypotheses, while
  gist delivery has not demonstrated that effect.
- Swaps work only with explicitly matched contexts: inspect natural self
  activation and build a separate context-matching test before changing policy.
- No arm moves and positive control fails: the decision task remains insensitive.
- Positive control succeeds but B/C fail: the compressed hypothesis is less
  influential than direct preference; that does not prove an organ defect.

The existing post-correction event for `verification` is *not checking*, not
necessarily *consulting*. Do **not** infer a preference reversal from that
counterexample. Longitudinal behavior after retirement/relearning needs a
separately registered symmetrical intervention and is **outside this small run**.
Likewise, observation-evidence is disabled for isolation and must be tested in
its own ON/OFF experiment.

## Version 3: fixed all-task factorial (no calibration gate)

The 22 October v2 Grok run used 24 calibration calls and failed qualification.
21/24 responses chose the first-listed option, including all six reverse-order
consult-cue trials choosing `check`. The old synthetic cues only described two
previous successes, while the tasks asserted both methods were currently
identical in utility; following a cue was not objectively compelled.

Do **not** retry that calibration with a relaxed threshold or select a task on
these responses. V3 treats the three already registered tasks as a fixed panel.
It first reads the original v3 pilot states without mutating them. It then
collects a full 3 tasks x 2 scripted histories x 7 arms x 2 option orders x
1 requested seed = **84 actor choices** (no calibration calls). Using one seed
instead of two trades repetitive near-duplicates for coverage of all tasks.

The `positive_control` direct-preference arm is included in all three tasks.
Before collection, its diagnostic criterion is fixed at >=10/12 history-matched
choices and >=4/6 within EACH option order. Failure does not discard the
collected results, but makes them unsuitable for claims of a functional actor
sensitivity. The other arms' differences are always exploratory, with
counts broken down by order and by source history. There is **no favorable
choice oracle** for the five substantive arms: their directional scores only
measure consistency with scripted experiences, not correctness.

This protocol does not fix positional preference or prove individuality. It
avoids using the same output to select a task and demonstrate an effect.
All previous failed runs must be retained separately, including calibration
and HTTP logs. Reusing source checkpoints still requires matching SHA-256s.

```sh
cargo test --bin selmem-genesis
bash experiments/genesis/run-self-bridge.sh \
  experiments/genesis/results/self-pilot-v3-grok-real-001 \
  experiments/genesis/results/self-bridge-v3-preflight-001 --prepare-only

export GENESIS_SELF_PROVIDER=grok
export GENESIS_SELF_MODEL=grok-4.3
export GENESIS_SELF_REASONING=none
export GENESIS_SELF_BRIDGE_HTTP_LIMIT=128
bash experiments/genesis/run-self-bridge.sh \
  experiments/genesis/results/self-pilot-v3-grok-real-001 \
  experiments/genesis/results/self-bridge-v3-grok-real-001
```

Examine `activation-summary.json` and `preflight.json` before real collection:
all `full_loop` explicit self views must contain one accessible belief, and
both `full_loop` and `memory_only` must have nonempty episodic gists for each
task. The run will otherwise stop before actor calls. After collection, read
`measures.json`, especially `arm_rates[].first_option`,
`arm_rates[].by_order`, `arm_rates[].by_task`, `comparisons[].by_order` and
`positive_control.diagnostic_pass`.
