# GENESIS contextual-belief test — run specification

Status: **experimental protocol and runner, not results**. The scripted witness
provides a deliberately synthetic oracle for memory-state qualification; it is
**not** an independent LLM validation of the agent's experiences.

## Research question and falsifiable outcomes

Can SelMem represent and deliver a learned *context-dependent action pattern*
without confusing contradiction with global extinction? Does that new contextual
state change held-out choices compared with identical memory input but contextual
refinement turned OFF? This does not measure subjective preferences, causal
explanations, consciousness, or spontaneous individuality.

Before running the paid actor, the offline runner must demonstrate:

1. Formation: a broad belief grounded in two independently labelled decisions.
2. Contradiction: two opposite decisions in a different attested circumstance
   generate two `Candidate` children with **zero** qualified children.
3. Prospective confirmation: a fresh decision in each context promotes both
   candidates to `Conditional`. No candidate is delivered earlier.
4. Recovery: two contrary actions in the exceptional context create conflict;
   later independent evidence supports the original general action across both
   contexts, causing a return to `General`.
5. OFF: replaying exactly the same sequence with `contextual_scope.enabled=false`
   produces no scoped children.
6. Confounded sham: time-of-day exactly tracks company; the two changing
   dimensions prevent a discriminatory hypothesis.

Two **mirrored** histories reverse the action-to-company mapping at B. At B
qualified, each history has three quiet and three lively observed decisions,
with equal numbers of `alone` and `friends` observations. No subjective
valuation is supplied. Each history is run with ON and OFF, source event IDs
are stable across these two arms, and the scripted event order is fixed.

## Offline setup — zero model calls

Install the contextual-belief feature patch **first**, then install the
experiment's additive patch. From the repo root:

```sh
cargo test --test contextual_beliefs
cargo test --bin selmem-contextual-experiment
cargo run --release --bin selmem-contextual-experiment -- --out experiments/genesis/results/contextual-offline-001
```

Outputs:

- `manifest.json`: version, seed, mode and model-call budget.
- `organ.jsonl`: every state transition, certainty, candidate/qualified counts,
  and IDs returned from typed contextual retrieval.
- `snapshots/*.vault`: isolated, recoverable ON/OFF checkpoints for the actor.

The command refuses to overwrite an existing output directory. If qualification
fails, it returns nonzero **before any paid model calls**.

## Optional matched LLM choice experiment

```sh
export GROK_API_KEY='YOUR_KEY' # only in your environment; not written to files
cargo run --release --bin selmem-contextual-experiment -- \
  --out experiments/genesis/results/contextual-grok-001 \
  --actor --provider grok --model grok-4.3 \
  --repeats 1 --seed 1729 --max-http-calls 128
```

For OpenAI, use `--provider openai --model MODEL` and set `OPENAI_API_KEY`.
Only choose a model that accepts chat-completion `temperature:0` and `max_tokens`.
There are **exactly 128 attempted LLM calls per repeat**, no background
requests and no automatic retries. `--repeats 3` needs an explicit cap of 384.
API errors produce invalid rows, never silently count as a preference choice.

Actor conditions, all invoked with fresh, unsaved snapshot copies per probe:

- ON: contextual model and episodes; explicitly delivered qualified beliefs.
- OFF: identical history, contextual learning disabled; broad belief is allowed.
- EPISODES_ONLY: ON history, but self-dispositions excluded from delivery.
- BARE: a fresh memory-free agent with the same model and question.

Two phases are probed: after B qualification, and after C recovery. Every arm
is crossed with both histories, both contexts (alone / close friends), two
neutral phrasings and reversed option order. Order of API requests is
deterministically shuffled with the registered seed.

Held-out prompts name neither a remembered belief nor an experiment goal. The
choice is between equally priced, equally accessible quiet and lively venues.
Contexts are supplied both naturally in the question and as explicit typed
`ContextFacet` values to the SelMem API, **not inferred by SelMem from prose**.

The actor must reply exactly `1` or `2`. Invalid, refused or failed responses
are retained; no post-hoc interpretation is permitted. Choice mapping is
recorded after reversing the options. Snapshot copies are reopened per probe
so no trial can contaminate the next probe's memory.

## Analysis and decision rule

`choices.jsonl` contains one row per attempted request: phase, history, arm,
context, paraphrase, option order, expected *scripted* action, chosen action,
validation result and IDs of delivered self-beliefs. `summary.jsonl` reports
attempts, valid answers, and per-attempt concordance with the action used in
the history. Keep invalid answers in the denominator.

**Primary contrast, registered before observing answers:** for B-qualified
snapshots, subtract OFF concordance from ON concordance *within the same
history/context/wording/order*. A difference is a contextual-delivery effect,
not necessarily internally acquired preferences. Also compare ON against
EPISODES_ONLY and BARE. Look for reversed context effects across mirrored
histories, with stable option-order differences. A convincing effect should
change its direction between `alone` and `friends` at B, and collapse after
C recovery when the history now supports A in both contexts. Report ALL arms,
not only the best one.

**Interpretation:** a positive ON–OFF difference with stable reversal across
mirrored histories supports causal sensitivity to the contextual self-memory
*delivered to the actor*. It does **not** prove silent adaptation: memory
statements are present in the actor context. A difference absent in held-out
probes is a negative result, even if internal memory transitions pass. With one
repeat, there is too little data for strong statistical conclusions; replicate
with multiple independent histories, untrained situations, independently
attested outcomes and a non-scripted scorer before publication.

## Confounds and limitations

- These are scripted observations with hard-coded action labels. Actual
  perceptions, feelings and motive attribution are not established.
- At B the marginal action counts are matched across mirrored histories, but
  the conditional relationship is deliberately different. At C, global
  frequencies change, so compare within-phase arms rather than claiming an
  isolated contextual learning effect across phases.
- Readout is explicit in ON: measure actor-context sensitivity, not
  unprompted internal preference formation.
- This runner calls `consolidate_with_scorer()` for deterministic reproduction,
  not full multi-pass sleep. Full-night forgetting/drift must be a separate
  experiment with independent horizons.
- `genesis-contextual-experiment-1` is an independent addition; it does not
  alter the memory organ or its existing tests.
