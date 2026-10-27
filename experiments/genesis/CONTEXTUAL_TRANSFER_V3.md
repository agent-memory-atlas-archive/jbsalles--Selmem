# GENESIS Contextual Transfer v3 — Long history / bounded memory

## Motivation

v2 reached an accuracy ceiling: the model saw all six original action observations in every memory-enabled arm. v3 hides the full observations from the ON/OFF actor payloads and tests bounded memory after 160 intervening episodes. This is a new experimental protocol, not a finding.

## Preregistered design

- 12 independent semantic domains; two mirrored lives per domain.
- Six relevant action episodes per life plus 160 later distractor episodes stored in the actual SelMem vault for both ON and OFF.
- The same 64-word **maximum** memory budget in every arm; record actual memory_words and selected_self_ids. No padding is used.
- Six arms: ON (typed context-conditional self belief), OFF (general belief), Episodes (unfiltered recency window), Summary (faithful compact aggregate of all six relevant actions), Retrieval (task-aware top matching original episodes), Bare (no autobiographical history).
- Two held-out wordings and answer orders balanced across mirrored lives. Eight decisions per domain per arm, 48 calls/domain. Plan is exactly `2 histories x 2 contexts x 6 arms x 2 counterbalanced cells`.
- Primary: ON minus Episodes accuracy, at least +0.15 averaged over 12 domains and strictly positive in at least 10 domains. Secondary strong-baseline checks: ON minus Summary and ON minus Retrieval. A result meeting only the primary while losing to Summary/Retrieval is NOT evidence that reconstructive memory outperforms compression or retrieval.
- Optional independent LLM judgement: four claims per domain, separate from scripted qualification, can veto before actor calls.
- Offline organ qualification: no network calls; must pass before actor starts.

## Known limitations

The initial parent belief is seeded and the internal qualifier is deterministic. The actor receives explicit historical text; its decisions cannot prove independent personal preferences, subjective identity, or autobiographical consciousness. ON is a compact predictive representation of the action, so superiority over recency-only is expected and alone not particularly informative. Summary and task-aware Retrieval are crucial strong controls. The matched cap is *maximum* words, not identical information content or token count. Number of calls is 48 per domain (96 at pairs=2; 576 at pairs=12), plus 4 judge calls per domain if enabled. API reasoning settings are not directly controlled. No repeated calls or resume on API errors.

## Commands

```bash
cargo test --bin selmem-contextual-transfer
cargo run --release --bin selmem-contextual-transfer -- --out experiments/genesis/results/contextual-v3-offline-001 --pairs 2 --distractors 160 --memory-words 64
cargo run --release --bin selmem-contextual-transfer -- --out experiments/genesis/results/contextual-v3-grok-pilot-001 --pairs 2 --distractors 160 --memory-words 64 --actor --provider grok --model grok-4.3 --seed 1729 --max-http-calls 96
```

For the full run use `--pairs 12 --max-http-calls 576`. With `--judge`, add four calls per domain and use an independent model and credentials. Do not interpret a pilot as evidence for the preregistered 12-domain criterion.
