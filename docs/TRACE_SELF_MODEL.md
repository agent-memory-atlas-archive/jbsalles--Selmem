# Trace-based contextual self

Built on `da16605`. This is a functional architecture, not evidence of subjective
consciousness or a demonstration of behavioral individuality.

## State and loop

`MemoryTrace.personal` contains a `SelfMemory`: role (`Episode`, `Belief`,
`Aspiration`, `Commitment`), context, prospective expectation, importance,
epistemic certainty, supporting/counterexample trace IDs, consumed observation
keys, inaccessible-foundation flag and dated appraisal. All are lived traces;
there is no second autobiography. The contextual view is recomputed on demand
and returns at most eight accessible, unsuppressed, non-retired propositions.

1. Interpret the incoming event, then activate the prior contextual view.
2. Compare the event to expectations already in that view. The configured
   `PropositionScorer` supplies entail/contradict/unknown. Without a scorer,
   revision remains unknown. This may add up to eight scorer calls per input.
3. Record that pre-event view's IDs and appraisal time before any belief update.
   Personal relevance raises the normal gate score. Continuity >= 0.8 or
   revision >= 0.8 independently admit a trace, regardless of belief certainty.
4. A deterministic 1/20 content sample admits up to sixteen currently accessible
   **attested observations** via an opening allowance. Unannotated reports keep
   legacy admission. This is not a global cap on the existing memory store.
5. At sleep, consolidate before destructive night passes. Consume distinct
   source/observation keys once. Belief certainty changes at most 0.15 per cycle;
   even capped evidence is consumed, so repeating sleep cannot manufacture proof.

`SelfEvidence::Report` is the default. Only explicit `Observation` episodes can
update beliefs. Generated abstractions are `Interpretation`, never independent
proof. Suppressed, latent, merged and confabulated traces cannot bootstrap a
belief; source text is never recovered from archives or operation journals.

Two distinct internal observations sharing a schema may produce a tentative
contextual belief. `Narrator::distill_self` proposes its statement and a testable
expectation for FUTURE events. The HTTP narrator requests structured JSON; the
rule backend uses its existing generalization and predicts recurrence of one
retained action. Initial certainty is 0.35, controlled by the organ. Schema
quality and semantic witness quality remain empirical limitations.

Contradictions lower certainty and remain explicitly linked. A contradicted
belief retires at certainty <= 0.05, allowing a later hypothesis. Aspirations
and commitments retain their role: an outcome never promotes them into acquired
qualities. Counterexamples can leave an unresolved tension rather than force a
coherent identity. This version does not automatically infer arbitrary context
splits or commitment fulfillment/deadlines; integrations may supply explicit
contextual propositions.

Contradictory episodes receive 24 virtual hours of protection, not permanent
pins. Self abstractions are excluded from episode retrieval, merge,
confabulation and legacy axiom extraction. Accessibility still decays. Missing
support marks foundations inaccessible and reduces certainty once by 0.1;
forgetting does not automatically delete the disposition or restore an archive.

## Rust API and attribution

```rust
use selmem::{Attribution, EncodeInput, EntityProfile, SelectiveMemory};
let mut memory = SelectiveMemory::new(EntityProfile::new("Claire"));
let mut event = EncodeInput::observed(
    "I told Alice about the error despite the cost.", "tool-result-0001"
);
event.attribution = Attribution::Internal;
event.schema = Some("honesty with Alice".into());
memory.live_with(event);
```

Reuse observation IDs for retries. Distinct IDs are the caller's assertion of
distinct observations, not a guarantee of statistical independence. Use `new`
for unverified statements. A user saying "you are a coward" is not an observed
agent action. Actual tool outcomes and executed decisions should use `observed`.
The existing chat adapter records user reports; it does not relabel those as
the agent's actions. UI night recollections are attested internal events.

For an explicit aspiration, commitment or hypothesis, fill `input.personal`
with its role, context, importance, certainty and optionally expectation.
Internal attribution must be explicit and the channel must be `Selfhood`.
This remains an interpretation, not evidence for itself.

Personal propositions must be atomic. Multipart events keep whole-event
relevance but do not assign evidence to an arbitrary fragment. Submit atomic
observed events when evidential revision matters.

## Delivery, interventions and persistence

Structured readout includes `self_model` with role, certainty, importance,
context, missing foundations and counterexample count. It shares the configured
word budget, has at most four entries, and obeys the dispositions component cut.
Abstractions are not exposed as observed episodes. Legacy/non-structured
narrators receive labeled statements.

`store.self_policy` has independent runtime switches:

- `selection`: self-dependent admission, including opening;
- `consolidation`: creation/revision during sleep;
- `opening`: exploratory admission only.

All default true. Reapply overrides after loading. A memory-only control starts
without self traces and disables selection/consolidation. Frozen-self disables
consolidation. An evolving self without selection feedback disables selection.
Match histories, models and budgets when comparing these conditions.

File snapshots gain a versioned `personal-v1` length-prefixed extension; SQLite
gains a `personal TEXT` column via existing migration. Both use the same codec
and reject malformed metadata. Old snapshots default to episode/report without
fabricating prior identity. New extensions require the updated reader.

Tests in `tests/self_model.rs` cover causal selection/cuts, pre-event state,
retry/recall evidence invariants, uncertainty versus importance, provenance,
role separation, opening bounds, suppression, bootstrap, temporary protection,
readout labels, and both persistence backends. Tests establish code invariants;
LLM proposal quality and downstream behavioral effects need experiments.

```sh
cargo test --test self_model
cargo test --test identity_nights --test memory_context_defaults
cargo test --all-targets
```

Validation for this patch: 503 tests passed across the target suite, including
16 self-model tests. The three year-long horizon simulations were run separately
against an optimized library. Patch application was checked on a clean da16605
worktree. The identity-name regression assertion was made case-insensitive to
match the existing prompt's uppercase NOT without changing its behavior.
