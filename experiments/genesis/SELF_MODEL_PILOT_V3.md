# GENESIS self pilot 3: qualify before collecting choices

This incremental patch targets the supplied `selmem-self-pilot-source.tar.gz`
(SHA-256 `a31c3d3223cbbf89175be294f69e73622272d3e543374005793e1b80e0faca9e`).
Pilot-1 and pilot-2 fixtures remain historical protocols. Engine changes affect
new consolidations in every protocol; do not resume an old lot with this binary.

## Why the real v2 lot was unqualified

`self-pilot-v2-grok-real-001` completed 128 choices in 148 HTTP attempts, with
zero initial self beliefs in both histories. The narrator abstained before the
guard ran at bootstrap. Conditions therefore shared the same self-free states.
All 64 `handoff` choices had empty recall and followed the first offered option;
all 64 `handover` choices selected `check`. This lot establishes no behavioral
effect of self feedback. The separate later assay formed one first belief,
not a demonstrated revision of an initial belief.

## Corrections and limits

Generation now receives only `separate_observations`, including action kind,
bearer and source identities. Blank `statement`/`expectation` fields are removed
from its input. The prompt explains that identical wording at distinct supplied
observation identities can support a tentative repeated-action pattern; it does
not prove a universal trait or independent real-world samples. An abstention
must include `abstention_reason`, retained in the raw HTTP exchange. The core
audit records abstentions and invalid proposals even when no guard is called.
No replacement belief is injected after an abstention. The recorded Grok short
diagnostic qualified both histories in eight HTTP attempts. The subsequent full
lot also qualified and completed 128 choices; [results and limits](SELF_MODEL_RESULTS.md).
This does not guarantee bootstrap success on a fresh collection or isolate the
effect of one prompt change from the other v3 corrections.

After the group guard accepts a proposal, every source is labeled separately
against its proposed expectation. At least two distinct action roots must be
individually supportive. Contradictory sources become `counter_ids`, and Unknown
sources enter neither support nor counter lists. Support and counter roots are
consumed once. This prevents an old opposite action from becoming a foundation
merely because it shared a schema with two recent matching actions. The existing
exact repeated-action fallback needs no semantic inference and retains its
structural checks. Guards and per-source labels remain fallible LLM judgments.

Native `self_consolidation_audit` distinguishes `source_ids` (submitted group),
`support_ids`, `counter_ids`, `unknown_ids`, final `verdict` and `reason`.
Existing persisted beliefs are not retroactively relabeled. These changes use
live traces only; no sealed archives or operation journals are queried.

V3 checks both birth beliefs and recall for all four history/task combinations
before model swaps and actor calls. It writes `qualification.json` and stops on
failure. It also checks every main actor context after checkpoint preparation,
writes `actor-coverage.json`, and stops before actor calls if any is empty.
Coverage checks do not assess whether a nonempty memory is useful or correct.

The second task now explicitly describes an incident response with conflicting
incident records. This vocabulary is shared by both histories, introduces no
preferred action and permits the existing retrieval mechanism to match the
task. Retrieval thresholds and memory budgets are unchanged. This is a new
registered wording: do not pool its choices with v2. Nonempty recall does not
eliminate position bias; both option orders remain fully crossed with both
requested seeds. The 128 choices still represent only two scripted histories.

## Run the short real diagnostic first

Apply the patch to the same local sources, then:

```bash
cargo test --test self_model
cargo test --bin selmem-genesis
bash experiments/genesis/run-self-pilot-v3.sh \
  experiments/genesis/results/self-pilot-v3-mock-001 --mock

export GENESIS_SELF_PROVIDER=grok
export GENESIS_SELF_MODEL=grok-4.3
export GENESIS_SELF_HTTP_LIMIT=24
bash experiments/genesis/run-self-pilot-v3.sh \
  experiments/genesis/results/self-pilot-v3-bootstrap-grok-001 --bootstrap-only
```

The diagnostic calls the real memory narrator/scorer and checks birth recall;
it makes zero actor-choice calls and runs neither correction nor relearning.
Inspect `report.json`, `qualification.json`, `diagnostics.json` and `http.jsonl`.
A qualified status requires two birth beliefs and all four nonempty contexts.
The 24-attempt ceiling includes retries and does not guarantee completion.

If qualified, collect a new full lot in another directory:

```bash
export GENESIS_SELF_HTTP_LIMIT=256
bash experiments/genesis/run-self-pilot-v3.sh \
  experiments/genesis/results/self-pilot-v3-grok-real-001
```

The full lot repeats its own bootstrap and qualification. Success in the short
diagnostic does not bypass those checks or guarantee the next bootstrap.
Every output directory must be fresh. The wrapper archives completed and failed
lots as `OUT_DIR.tar.gz`. A budget is an attempt ceiling, not a monetary cap.

For a no-call design check, use the same wrapper with `--prepare-only`.
Bootstrap-only and prepare-only are mutually exclusive. Historical results and
raw rejected proposals should be retained, including qualified/failed counts
across repeated diagnostic attempts; do not report only successful attempts.

## Development verification

175 tests passed, with one existing GENESIS test ignored. This covers the
library unit tests, `self_model`, GENESIS, `audit_p0` and `integrity_audit`.
The local HTTP abstention test stops after two narrator requests and creates
neither swaps nor actor trials. Mixed-source tests keep supports, counters and
unknowns separate, and reject a group with fewer than two individual supports.
The complete finite mock delivered all 128 choices with zero HTTP attempts and
nonempty recall in all four birth/task combinations and all actor cells.
Mock outputs are technical checks, not evidence of real LLM individuality.
No paid Grok calls were made during development. Compile-time resources absent
from the supplied archive (`data/`, `voices/`, `examples/`) were supplied from
GitHub `b9fd382` for validation and are not changed by this patch.
