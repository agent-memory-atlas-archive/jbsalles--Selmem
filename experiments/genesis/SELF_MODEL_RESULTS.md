# Contextual self-model results

Analysis of the supplied Grok archives on 22 October 2026. The qualified v3 lot
supports a functional effect of the contextual self on memory admission and
revision in two scripted histories. It does not establish durable behavioral
individuality, consciousness, a performance gain or a creativity advantage.

## Recorded lots

| Lot | Recorded outcome | Interpretation |
| --- | --- | --- |
| `self-pilot-v2-grok-real-001` | 128 choices, 148 HTTP attempts; 0/2 birth beliefs; all 64 `handoff` contexts empty | Unqualified for the feedback comparison. Retain as a failed bootstrap diagnostic; do not pool its choices with v3. |
| `self-pilot-v3-bootstrap-grok-001` | 2/2 birth beliefs, 4/4 nonempty history/task readouts, 8 HTTP attempts, zero actor choices | Qualified short diagnostic, not a behavioral replication. |
| `self-pilot-v3-grok-real-001` | 128/128 choices, 180 HTTP attempts, 2/2 birth beliefs; all 128 actor contexts nonempty | Qualified full pilot. No HTTP errors recorded; all 128 recorded checkpoint hashes match the supplied vaults. |

The full v3 lot used `grok-4.3`, temperature 0, reasoning `none`, two requested
seeds and both option orders for every matched cell. Its 180 HTTP attempts
comprise 6 narrator requests, 6 group guards, 40 expectation/outcome labels and
128 actor-choice requests. [Protocol and commands](SELF_MODEL_PILOT_V3.md).

## Admission follows the contextual self

Each history imposes two observed action outcomes, either coordinating with a
crew or independently checking records. These are simulated interventions with
explicit semantic attribution and calibrated salience, not autonomous actions
or human conversation relabeled as lived agent experience. The common birth
state is cloned into four conditions. Each condition receives the same routine
note opportunities and three contrary action outcomes.

| Condition | Own-context notes admitted before correction | Own-context notes admitted after correction |
| --- | --- | --- |
| `memory_only` | 0/2 | 0/2 |
| `frozen_self` | 2/2 | 2/2 |
| `evolving_no_feedback` | 0/2 | 0/2 |
| `full_loop` | 2/2 | 0/2 |

The denominator is two histories, one own-context note opportunity per history
and phase. All 16 other-context note opportunities in these main cells were
rejected. Two surgical swaps preserved each episode history but transplanted
the other history's self traces, marking their foundations inaccessible. In
both directions, only the note matching the transplanted self context was
admitted. This is a causal circuit check of self-dependent admission under the
specified intervention, not evidence of naturalistic salience learning.

## Retirement and later learning

All four evolving branches (two histories times two evolving conditions)
retired their initial belief after two recognized counterexamples. Certainty
fell from 0.35 to approximately 0.05; the third contrary action arrived after
retirement. Both frozen beliefs remained active at 0.35. The decrement and
retirement threshold are implementation rules, not empirically calibrated
probabilities. Contradiction is the scorer's operational label against a
tentative expectation; this does not logically falsify the possibility that an
action "may" recur.

The separate `relearning_only` assay copied the after-correction checkpoints and
supplied two new contrary actions before one sleep. All four evolving branches
formed a new belief about the opposite action pattern. Every new belief had
exactly two supports, both from the new `fresh-action` observations, with no old
opposite action listed as a support. The old beliefs remained retired. Frozen
branches retained their old belief; memory-only branches created none.

These copies did not contribute to the 128 actor choices. The assay establishes
later model formation in this fixture; it does not measure behavior after that
new model or the persistence of a new disposition across a long life.

## Decision effects remain small and localized

Self propositions were excluded from actor prompts. The actor received only
selected stored episode gists, capped at 320 words. Delivered contexts matched
the recorded gists in all 128 trials. Differences therefore concern this
episode-mediated readout, rather than direct self-proposition prompting.

| Matched comparison | Different choices / pairs |
| --- | --- |
| `full_loop` vs `memory_only` | 2/32 |
| `full_loop` vs `frozen_self` | 1/32 |
| `full_loop` vs `evolving_no_feedback` | 0/32 |
| `frozen_self` vs `memory_only` | 1/32 |
| `evolving_no_feedback` vs `memory_only` | 2/32 |
| Before vs after correction, all conditions | 5/64 |
| Forward vs reverse option order | 5/64 |
| First vs second requested generation seed | 1/64 |

Overall, Grok chose `check` 123 times and `consult` 5 times. All five consultation
choices occurred in the coordination history after correction, on `handoff`,
with consultation offered first: twice in `full_loop`, twice in
`evolving_no_feedback`, and once in `frozen_self`. The two full-loop differences
from memory-only repeat that same cell across the two requested seeds; reversing
the options removes the difference. The full loop and evolution without
admission feedback produced identical choices across all 32 matched pairs,
even though some delivered episode contexts differed.

These are descriptive sensitivity counts, not independent replications or
accuracy scores. No oracle ranks the offered strategies. The lot does not
demonstrate an added behavioral effect of admission feedback over evolution
without that feedback. Neither the small differences nor their absence
establish a general advantage or ineffectiveness of SelMem.

## Scope and next test

There are two scripted histories, not 128 independent lives. Actions repeat
identical wording; attribution and observation identities are supplied by the
adapter. Narration and semantic labels use the same model family and remain
fallible. A qualified v3 run after a failed v2 run also changes the prompt and
one task wording; their difference cannot isolate one correction's effect.

The next informative study needs more independently varied histories and tasks
that do not overwhelmingly favor one strategy. Register its comparisons before
collection, retain both option orders and all qualification failures, and test
transfer and persistence. Repeating only these same frozen decision cells
would add little independent evidence. Creativity needs its own useful-output
evaluation and equally budgeted controls.

## Provenance

Original archives remain separate audit artifacts; this document does not
publish the raw vaults or HTTP exchanges. Preserve the exact files and hashes:

| Archive | SHA-256 |
| --- | --- |
| `self-pilot-v2-grok-real-001.tar.gz` | `fd5fda7a9c427a01b20db5b00adff3271278a4c920e9f603509678ffde1210cc` |
| `self-pilot-v3-bootstrap-grok-001.tar.gz` | `1fb9d03f462c53d1ba6ab7772a5ddaedafcca602c39927619f24cb1c29b1cbd2` |
| `self-pilot-v3-grok-real-001.tar.gz` | `78a48318fbaf48329914a8c7229f476a8936717630a6902b882575c51a84a96f` |

Full v3 `design.json` records code/binary SHA-256
`ed543e43638e742225cbe6b17a051da0ddc1b5f5bc66e107f10ca57129266aa8`
and fixture SHA-256
`11c963f9d2ec5e07e897288ea5cdad71b1542ff36b407204b20ab0645072e29d`.
These are recorded hashes, not Git commit identifiers. Audit `report.json`,
`qualification.json`, `actor-coverage.json`, `measures.json`,
`paired-measures.json`, `diagnostics.json`, `trials.json`, checkpoints and
`http.jsonl`. A fresh build or later documentation edit does not change the
identity of these historical outputs.
