# GENESIS — Contextual transfer, version 2

**Status:** experimental runner, not an empirical result. The previous synthetic
Grok pilot established context-sensitive *delivery*, not self-originated preferences.
This protocol tests near transfer to previously unobserved choice descriptions.

## Hypothesis and limitations

H1 (primary): conditional memories yield higher history-concordance on *held-out
paraphrased decisions* than identical episodic evidence without conditional
self-beliefs. The target is a **behavioral action pattern**, not an internal
preference, causal explanation, identity, creativity, or sentience.

The input is synthetic. The broad parent is explicitly seeded after repeated
observation; the memory organ's group/individual evidence labels still use a
structured synthetic witness. An optional **independent LLM auditor** validates
qualified claims before actor requests, but does not replace the organ's witness.
Do not call this fully autonomous belief learning.

Twelve semantically different life domains are preregistered in the Rust source:
workspace, dining, fitness, books, museum, music, commute, lodging, lunch,
shopping, learning, entertainment. Each domain produces two *mirrored* trajectories
which invert the observation-to-company mapping. There are 24 histories, but the
**independent inferential unit is a domain pair (N=12)**, not 480 model calls.

## Five actor conditions

Every condition except BARE sees exactly the same six source observations,
including the explicit `company` label attached when the observation was encoded:

- `on`: contextual self-belief selected via `view_scoped`, alongside all six events.
- `off`: original general self-belief, feature disabled, same six events.
- `episodes`: the six events only; no separate self-belief interpretation.
- `shuffled`: the same observations, but a belief selected using the *opposite*
  company facet; placebo/semantic-inversion check.
- `bare`: no stored history.

The prompt and decision question are identical for the paired arms aside from
these memory passages. `memory_words` in `choices.jsonl` exposes any passage-length
imbalance (word counts are not LLM token counts). All conditions use fresh reopened
vaults and never write to the actor snapshots.

## Memory qualification

For each mirrored life: two initial observed actions in `alone` context seed a
broad fallible belief; two different observed actions in `friends` context
produce *hidden* candidates; a new independent observation per context is then
required to produce two `Conditional` beliefs. OFF produces no children.
A failure aborts **before** API calls. Source IDs cannot be counted twice.
The runner does not replay recovery; the original v1 experiment already tested
that stage. This experiment isolates the qualified-memory behavioral phase.

Optional independent audit: `--judge` uses a separate provider or model from
actor and requests entail/contradict/unknown from independent observed action
records. It vetoes unexpected or unsupported qualified claims. It is a separate
**audit**, not the internal semantic `PropositionScorer`. This distinction must
remain explicit in any report.

## Held-out actor probes

- Two contexts: alone / friends.
- Two new descriptions per domain, with **different wording** from all six
  remembered action sentences.
- Option order reversed, with `1`/`2` strictly parsed.
- Mirrored histories cancel default action or semantic valence preferences.
- Actors only see observed actions, fallible interpretation if applicable,
  and a novel decision — never the expected answer or the experimental labels.
- The actor is prompted to select an action for itself, without assigning
  subjective preference or affect to its past behavior.

Per domain: 2 histories × 2 contexts × 5 arms × 2 counterbalanced (wording, order) cells = 40 calls. This is not a full 2×2 factorial within each history; option order changes with wording inside a history, while mirrored histories reverse option order for each wording and context.
**Full 12-domain run:** 480 actor calls, plus 48 optional independent auditor
calls; strict single `--max-http-calls` with no retries. **Pilot with 2 pairs:**
80 actor calls, plus 8 independent auditor calls.

## Primary analysis, preregistered before model calls

Aggregate binary concordance with the matching life, counting invalid answers
as errors. Per-domain rates include both mirrored trajectories, both contexts,
both phrasings and both option orders (16 trials per domain/arm).
Compute domain-level ON minus EPISODES and ON minus OFF.

The directional preregistered screening rule for the full set is both:
- mean ON - EPISODES >= 0.15;
- ON > EPISODES for at least 10 of the 12 domain pairs.

These fixed thresholds are *screening* criteria, not calibrated power or
proof of significance. Check potential semantic asymmetries, option order,
and failure/invalid rates. Do not reinterpret mirrored lives, paraphrases or
repeated options as independent samples.

## Invocation

With the contextual organ and v1 experiment patch already applied:

```bash
cargo test --bin selmem-contextual-transfer
cargo run --release --bin selmem-contextual-transfer -- \
  --out experiments/genesis/results/contextual-transfer-offline-001 --pairs 12
```

The offline run makes **0 HTTP calls**. Check `organ.jsonl` and the saved vaults.
Then a limited two-domain actor pilot (80 calls):

```bash
cargo run --release --bin selmem-contextual-transfer -- \
  --out experiments/genesis/results/contextual-transfer-grok-pilot-001 \
  --pairs 2 --actor --provider grok --model grok-4.3 \
  --max-http-calls 80
```

Full preregistered set, with an independent judge, after the pilot passes:

```bash
cargo run --release --bin selmem-contextual-transfer -- \
  --out experiments/genesis/results/contextual-transfer-grok-001 \
  --pairs 12 --actor --provider grok --model grok-4.3 \
  --judge --judge-provider openai --judge-model gpt-4.1-mini \
  --max-http-calls 528
```

Set `GROK_API_KEY` and, for the optional auditor, `OPENAI_API_KEY` in the
process environment. Neither key is written to disk. APIs may reject
`temperature: 0` or `max_tokens` for some models; choose compatible chat
completion models. Actor error aborts immediately without retry and output
remains partial; use a new output directory for a new run.

Outputs: `manifest.json`, `organ.jsonl`, optional `judgements.jsonl`, optional
`choices.jsonl`, optional `summary.jsonl`, and `snapshots/*.vault`.

## Interpretation and follow-ups

A favorable ON vs BARE result is trivial if memory is supplied; the primary
comparison with EPISODES is the important test. ON must beat a **factually
matched history**, not simply an empty prompt. Because actions are still
synthetic and the memory text explicitly delivered, positive near-transfer
cannot be described as a lasting autonomous preference. A later naturalistic
study should replay unscripted, externally verified experiences and score
untaught longer-horizon tasks through a fully independent judge.
