# GENESIS self-model pilot (self-pilot-1)

Historical protocol. New collections should use [pilot 2](SELF_MODEL_PILOT_V2.md).
The validation below describes the original implementation.

This is a new diagnostic, not a continuation of a creativity collection. Its
primary endpoint is a causal change in **memory admission**, with revision when
credible counterexamples arrive. Decision readout is exploratory. It does not
measure creativity, consciousness, or population-level individuality.

## Design

The compiled `self-pilot.json` fixture contains two simulated incident histories:
coordination with a crew and independent verification of records. Each starts
with two distinct scripted action outcomes, explicitly observed and attributed
to the agent. These are imposed history interventions, not claims that an LLM
freely selected or actually performed real-world actions. Existing human
conversation datasets remain reports and are not relabeled as autobiography.

A contextual belief is consolidated from the actions, with no personality
sheet or self proposition injected at birth. Each resulting state is copied
into four conditions, preserving identical episode history within each family:

| Condition | Self influences admission | Self can learn/revise |
|---|---|---|
| `memory_only` | No; self traces removed after common bootstrap | No |
| `frozen_self` | Yes | No |
| `evolving_no_feedback` | No | Yes |
| `full_loop` | Yes | Yes |

This is an intervention **after common bootstrap**, not an end-to-end comparison
of four independently lived histories. Legacy axiom creation, co-recall and
reconstructive readout are disabled equally. The opening allowance is disabled
in every condition to isolate self feedback. Profile and salience parameters,
virtual clocks, admission opportunities and counterexamples are matched.

The test uses deliberately calibrated, externally annotated salience. Bootstrap
and counterexample actions are normally admissible in all conditions; routine
notes are below the ordinary gate. Such a circuit check establishes neither
naturalistic salience learning nor an autonomous identity.

Before and after three credible contrary outcomes, all conditions receive the
same two low-salience routine-note opportunities. Logs contain gate scores,
prior self IDs, appraisals, source/observation identities and resulting certainty.
Contrary events are evaluated before each corresponding night's revision.

Two additional surgical swaps keep each history's episodes while replacing only
its self traces with the other history's traces. Their missing source foundations
are explicitly marked. The swap is tested immediately, without consolidation or
archive access. It isolates the causal effect on the admission gate.

## Readout and endpoints

There are 64 choices: 2 histories × 4 conditions × 2 phases × 2 neutral decision
problems × 2 option orders. Choices are shuffled across cells. Matched cells use
matched requested seeds; option order reverses between repeats.

**Self-model contents are excluded from all actor prompts.** Only selected lived
episodes, capped at 320 words, accompany the same decision tasks. The self view
is exported separately for audit. This avoids mistaking direct personality
prompting for an effect mediated by memory selection. The complete source state,
legacy reading profile, certainty metadata and condition labels are not actor
inputs. Checkpoints are checksummed before each generation.

Inspect in this order:

1. `report.json`: completion/error, HTTP attempts, bootstrap qualification.
2. `measures.json`: matched admission counts before/after correction.
3. `diagnostics.json`: bootstrap, model swaps, states and lifecycle audits.
4. `trials.json` / `trials/`: delivered prompts, readouts, requested seeds, choices.
5. `blind-choices.json`: choices without condition/history labels;
   `blind-key.audit.json` is the separate mapping.

The real backend infers beliefs and expectations and labels outcomes with the
configured LLM. A missing bootstrap belief is recorded and makes the feedback
comparison unqualified; it is not silently replaced by the mock witness. HTTP
failures/budget exhaustion produce an incomplete report, never a valid result.
The HTTP log includes every request/response and retry. Source checkpoints and
completed diagnostic stages are retained on failure. There is no collection
resume mode in this short pilot: preserve a failed lot and use a fresh directory.

No threshold asserts the desired real result. A useful outcome would combine
history-dependent admission, a change after swapping the self view, and
credible-counterexample revision in evolving conditions. No admission effect,
failed revision or absent downstream choice difference are meaningful negative
findings. The proposed distribution of choices is not an oracle accuracy score.

There are only two scripted histories. Repeats, conditions and checkpoints share
sources; 64 choices are **not 64 independent lives**. No automatic significance,
individuality or creativity claim is emitted. If the causal mechanism qualifies,
then register a larger, independently varied creativity protocol and collect
fresh histories from encoding onward.

## Running

Offline technical rehearsal (no config or API key, no HTTP calls):

```bash
bash experiments/genesis/run-self-pilot.sh \
  experiments/genesis/results/self-pilot-mock-001 --mock
```

Here the narrator and witness recognize only the finite published fixture. The
mock actor selects the first offered option. Its choices cannot show behavioral
individuality; the rehearsal tests wiring, interventions, provenance and output
integrity only.

For real collection, configure your existing `.selmem` provider and select the
model explicitly. `GENESIS_SELF_MODEL` is the actual provider model identifier:

```bash
export GENESIS_SELF_PROVIDER=gpt
export GENESIS_SELF_MODEL='your-model-identifier'
export GENESIS_SELF_HTTP_LIMIT=256
bash experiments/genesis/run-self-pilot.sh \
  experiments/genesis/results/self-pilot-plan --prepare-only
# Inspect design.json, then use a NEW directory for collection:
bash experiments/genesis/run-self-pilot.sh \
  experiments/genesis/results/self-pilot-real-001
```

The script uses memory temperature 0, configurable reasoning (default none),
the existing provider key routing, and archives the output directory on success
or error. The 256-attempt ceiling includes 64 actor requests, self appraisal,
consolidation and all retries. It is neither a monetary cap nor a completion
guarantee. Model support, output validity and budget adequacy must be checked
from the prepared design and actual logs; no paid requests are made by prepare.

## Native GENESIS API

`memory_config.self_model_mode` accepts the four conditions above plus `legacy`
(default). Policies are reapplied on **every** reopen; runtime switches are not
assumed to survive a snapshot. `self_model_opening` is an explicit boolean;
it is off by default and is rejected when enabled for legacy/memory-only.

Native events default to `evidence: "report"`. An `"observation"` needs a stable
`observation_id`; `agency` explicitly distinguishes internal/external actors.
`semantic_annotation`, when supplied, uses the existing strict 15-field semantic
format. Invalid types or conflicting attribution are rejected before ingestion.
Annotations and distinct IDs are assertions by the simulation adapter, not proof
of independent real-world observations.

Native inspection includes the personal trace role, evidence, context,
expectation, importance, certainty, support/counter IDs, consumed roots,
retirement and prior appraisals. Probe output exposes `self_model_audit` and a
structured `memory_context`; `self_context: "included"` explicitly adds the self
view to that packet. Default `"audit_only"` keeps it out. Existing creativity
protocols retain their own readout rules and do not automatically adopt this new
packet or turn into a self-model experiment.

The `beliefs` lesion now removes new belief traces as well as legacy belief
axioms. `self_model` removes all self-role traces. Legacy schema-center lesions
still mean legacy schemas, not every possible derivative of an event.

A separate `score_expectation` contract distinguishes observed outcomes testing
prior expectations from historical reconstruction auditing. GENESIS uses a
separate prompt and `self_appraisal` HTTP category for this task. The existing
fact-preservation prompt stays unchanged. Its checked narrator forwards
`distill_self`, so the HTTP backend actually supplies a prospective expectation.

The pilot also fixes re-creation of a retired belief from the same consumed
observation roots. Genuinely new roots can support a later hypothesis; repeating
sleep cannot manufacture new evidence.

## Compatibility and checks

No historical fixture, archived result or creativity claim is overwritten.
New source/binary hashes differ, so do not resume a pre-patch collection with
this binary. Keep older results tied to their original commit and protocol.

```bash
cargo test --test self_model
cargo test --bin selmem-genesis
cargo test --all-targets
```

## Technical validation performed

On the finite mock fixture, both histories produced a contextual self trace.
Across the two matched histories, routine-note admissions in their own context
were 0/2 for memory-only, 2/2 for frozen-self, 0/2 for evolving-without-feedback
and 2/2 for the full loop before correction. After correction they were 0/2,
2/2, 0/2 and 0/2 respectively. Both surgical self swaps shifted admission to the
other context. All 64 mock choices completed with zero HTTP attempts. These are
programmed circuit checks, not independent observations of LLM individuality.

The target suite passed 509 tests (the three long `horizon` simulations were
excluded from this verification). This includes 17 self-model tests and the
GENESIS protocol, local HTTP adapter and regression tests. Prepare-only was
checked separately and made zero HTTP attempts. No paid LLM collection was run
in this development environment; runtime `.selmem` credentials were unavailable.
