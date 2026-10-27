# GENESIS Contextual Longlife v4 — pre-registered experiment

**Status: protocol and code; not a result.** The earlier v3 pilot showed SelMem ON = deterministic summary = task-aware retrieval (16/16 each). This is designed to distinguish query-independent memory formation from recent-window recall and generic full-stream selection. The code adds a standalone Rust binary; v3 is not modified.

## Hypothesis and controls

Hypothesis: Context-conditional self-models, constructed during life from **observed** actions, retain a usable behavior rule across many competing events. They may help a constrained actor make decisions in unseen settings better than query-blind summaries, frequency-based baselines and lexical retrieval, given the same maximum memory word budget.

There are 12 predefined domains and two mirrored histories per domain. The histories differ in which observed action was repeated with friends versus alone. The first two independently identified observations are consolidated into a broad self-belief by the existing SelMem self-model consolidation routine. **No parent self-belief is injected by the harness** (unlike v3). Two contradictory contextual outcomes then create unexposed candidate branches; fresh confirmations must qualify them. Later, one counterexample per context and additional confirmations arrive amid distractions. The offline qualification fails closed if bootstrapping, differentiated candidates, prospective qualification or the final contextual readout fails.

Each history contains 14 task-related actions (including two contradictory outliers) plus 240 other **observed** action records by default. Distractors share the same input source, provenance structure, attribution, context facet and event store as task evidence. All observation IDs are opaque hashes; baseline selection never receives the IDs of relevant records or the ground-truth outcome strings. ON/OFF histories are verified to contain exactly the same observed text and contextual metadata in the same order.

Six conditions:

- `on`: contextual self-belief selected by actual `view_scoped`.
- `off`: contextual differentiation disabled, using the broad self-belief (or empty fallback if retired).
- `episodes`: most recent observed actions across the entire available lived stream.
- `summary`: **query-independent** deterministic frequency summary from every observed event, generated without topic or target labels.
- `retrieval`: query-time lexical similarity over every observed episode, with the contextual facet value available to all comparators; **no gold relevant-record list**.
- `bare`: memory-free actor.

The summary and retrieval here are generic deterministic baselines, **not** an LLM-powered, semantic RAG or state-of-the-art hierarchical summarizer. Their limitations must be stated in any report. Structured contextual facets and a scripted proposition witness are still supplied by the simulator, so success would NOT show autonomous interpretation of free-form episodes.

## Decision probes

Two paraphrase/option-order cells per context, two contexts per mirrored history, two histories, six arms = **48 actor calls/domain**. The novel choice descriptions are not verbatim copies of observed actions. Every arm shares the same prompt instructions, temperature zero and memory **cap** (default 64 words). The resulting `payloads.jsonl` lists the exact model-visible memory, word count and selected IDs for **all** trials before any network request.

- Offline (`--pairs 2 --distractors 240`): **0 model calls**.
- Grok pilot (`--pairs 2`): **96 model calls**.
- Full (`--pairs 12`): **576 model calls**.
- Independent judge option (`--judge`): **+4 calls/domain** (pilot +8; full +48). The independent judge must be a different provider or model from the actor and can only veto selected hypotheses.

The `--max-http-calls` argument is a preflight limit; network failures stop the run and there is no resume. Use unique output paths. The code does not explicitly set reasoning mode, so reasoning-off cannot be assumed solely from these commands. The prompt format itself can influence choices; counterbalancing mitigates, not eliminates, that risk.

## Preregistered decision criterion

The unit of independent analysis is the **semantic domain**, not the 576 correlated model calls. Success requires all of the following in the full 12-domain run:

1. Mean SelMem ON accuracy at least **10 percentage points above the frequency summary**, and
2. Mean SelMem ON accuracy at least **10 percentage points above the lexical retrieval baseline**, and
3. ON beats **both** baselines in **at least 8 of 12 domains**.

ON minus recency, ON minus OFF, answer-order sensitivity and effective memory words are secondary. Report **invalid model answers** separately (the automated `concordant` field treats them as failures). The run must report failed qualification and controls; do not condition the sample on runs where the mechanism happened to work. Do not tune the fixture or thresholds after observing the 12-domain results without registering a new experiment.

## Commands

```bash
cargo test --bin selmem-contextual-longlife
cargo run --release --bin selmem-contextual-longlife -- \
  --out experiments/genesis/results/contextual-v4-offline-001 \
  --pairs 2 --distractors 240 --memory-words 64

cargo run --release --bin selmem-contextual-longlife -- \
  --out experiments/genesis/results/contextual-v4-grok-pilot-001 \
  --pairs 2 --distractors 240 --memory-words 64 \
  --actor --provider grok --model grok-4.3 \
  --seed 1729 --max-http-calls 96
```

Before paying for Grok, inspect `organ.jsonl`, `manifest.json`, and `payloads.jsonl` from the offline run. The manifest declares the synthetic history, model, seed and planned calls. `choices.jsonl` and `summary.jsonl` exist only in an actor run. The snapshots are not independent archives for recall; only active lived traces are used.

## Interpreting findings

An advantage over these methods would show a value of prospective conditional self-model consolidation under this **specific synthetic constrained-recall setup**, not subjective individuality or creative personality. It would remain necessary to test an equally-budgeted LLM summarizer, embedding RAG with equal metadata, unstructured natural-language life events and longer truly independent trajectories. An equal score or a ceiling is a legitimate negative result and must be published.
