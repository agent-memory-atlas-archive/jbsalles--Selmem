# GENESIS — Contextual belief differentiation and recovery (preregistered proposal)

## Status

Code tests validate organ invariants only. This protocol is **not** an experimental result. No model calls have been made. Do not claim autonomous preference learning, causal knowledge, superior accuracy or durable individuality on the basis of the new representation.

## Why this experiment exists

The base self model retires overly general beliefs after contradictory events. The context extension adds a deliberately conservative hypothesis: a repeated action pattern may differ by an observed contextual factor. A hypothesis is not delivered to the actor until new attested evidence supports each branch. Retiring the old belief does not demonstrate a preference reversal. Report each step separately: evidence admitted, candidate created, candidate qualified, candidate delivered, actor choice changed, and behavior retained.

## Factorial construction

Predeclare two variables, for example `company ∈ {alone, friends}` and `noise ∈ {quiet, loud}`, and outcomes such as observed *actions*. For a **preference** claim, collect explicit experiential evaluation as an additional source and never infer positive affect from action frequency alone. Use four mirrored histories with equal marginal counts of each variable and action; reverse which joint combinations are favorable. Include a confounded history where company and time of day always change together, and a neutral history with no observed decisions.

- A (formation): at least two independently identified attested observations supporting a broad action summary.
- B (contradiction): at least two opposite observed outcomes, with contextual facts and no preference sentence supplied. Hold out at least one later, independent observation per context for candidate validation.
- C (recovery): provide newly attested observations supporting A again in the previously exceptional context; also include a mere replay of historical source IDs to ensure retries have no effect.

No source ID may count twice. Store external attestation IDs, original trace IDs, model/scorer version, context parser outputs and timestamps. Differences in root IDs do **not** by themselves guarantee genuinely independent experiences. Reuse the exact same prompt and source budget in every arm.

## Controls and ablations

1. Contextual extension enabled; observe candidate creation and delayed promotion.
2. Contextual extension disabled (`store.contextual_scope.enabled=false`) after every reopen, with identical episodes.
3. Frozen self-model, still capable of remembering episodes.
4. Episode-only (no self dispositions), and explicitly memory-free control.
5. Sham contexts: shuffle labels after forming memory, preserving marginal counts.
6. Explicit self-statement positive sensitivity control, never conflated with native effect.
7. Opposite-option order and paraphrased contexts, fixed before generation.
8. No proposition scorer (`Unknown`), which must abstain rather than invent scopes.

The scope policy defaults **off** on construction and reopening; trials must explicitly set it per arm before collection. Record and verify this at every snapshot and before every actor call. Use fresh snapshot copies and hashes to ensure read-only probes leave the books unchanged.

## Qualification and behavioral probes

At A/B/C checkpoints inspect source evidence, candidate status and confidence, lineage, parent fallback, delivery audit, and actual actor payload. For the B checkpoint, make sure no candidate reaches the actor before fresh support. For C, report adaptation and resistance/recovery separately. Unknown contextual facts must not produce a fabricated conditional context; the unchanged generalized hypothesis should be explicitly labeled conflicted.

For every life, compare held-out decisions in situations not used in either formation or validation. Perform each test in both option orders with neutral language, without supplying explicit remembered beliefs. Test semantic paraphrases and combinations of factors absent from training. When a scoped self hypothesis is passed via an explicit readout, flag that test as a *delivery intervention* rather than silent learned preference. Do not reuse test questions for model selection or post-hoc tuning.

The key contrasts are (a) candidate formation versus no-formation, (b) qualified context-dependent retrieval versus generic RAG-style context, (c) choices with/without delivered self-context, (d) time/order effects and (e) recovery after new evidence, not mere stability. Counterbalanced event orders are mandatory; summarize effect sizes per independently varied history, not per repeated cell as if those were new agents.

## Failure criteria

The experiment fails to demonstrate added value when candidate status is reached but no novel decision differs; when negative examples are simply reinterpreted as preferences; when false or perfectly confounded context dimensions trigger scope promotion; when data replay changes certainty; when old self propositions remain in actor prompts despite a supposed ablation; or when apparent effects disappear after controlling for option order and wording. Record inconclusive failures and model validation errors rather than removing them from the analysis.

## Local code checks (no paid inference)

```bash
./run.sh test --test contextual_beliefs
./run.sh test --test self_model
./run.sh test --all-targets
./run.sh test --doc
cargo run --release --bin selmem-contextual-fixture > contextual-fixture.jsonl
```

The included `selmem-contextual-fixture` runner produces an offline JSONL A/B/C/D organ-state trace. It is **not** a substitute for an LLM actor-choice experiment. After the deterministic checks pass, implement and freeze the full matched LLM GENESIS arms described above; do not report behavioral findings until their prompts, snapshots, controls and outputs are audited.
