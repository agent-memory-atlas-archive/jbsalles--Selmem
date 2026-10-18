# Guarded creative preview (026)

This variant starts a fresh affordable Creative Preview using the current memory implementation. The legacy preview launcher and both Creative Life protocols remain available. Existing results are not reused or edited. Use a fresh output directory such as `experiments/genesis/results/creative-preview-003`.

## Design

The unchanged, checksum-pinned `creative-preview.json` contains two complete official LoCoMo histories: conv-26 (419 turns) and conv-30 (369). Each of the 788 turns is encoded once, chronologically, with one virtual hour between turns. Each history then receives ten ordinary sleep cycles. Its no-sleep control advances the same 240 hours. Three fixed tasks, five conditions and two matched seeds yield 60 proposals. No extra persona, salience boost, synthetic trauma, or new source experience is added.

| Condition key | Delivered context in this variant |
| --- | --- |
| `empty` | No memory |
| `rag` | BM25 over the original exposed source turns, capped at 600 words |
| `before_sleep` | Current lived gists from the equal-time no-sleep state, with task-blind guarded retelling |
| `after_no_recon` | Current lived gists from the post-sleep state, without retelling |
| `after_full` | The same selected post-sleep gists, with task-blind guarded retelling |

The condition keys remain compatible with existing blind-rating tools. **Here `after_full` and `before_sleep` are controlled readout interventions, not unchanged full native delivery.** The original native protocol remains available without `--guarded`.

Native retrieval sees the task to select relevant traces, with reconstruction disabled. Only selected current lived gists and anonymous trace IDs enter the retelling request. The reteller and its separate provenance guard never receive the task, raw source archive, cores, reading profile, or axioms. A guard acceptance permits the retelling; a rejection or malformed reply falls back to that same trace's current gist. The guard is fallible and is not a creativity judge. Interpretation can still leak an invented inference; inspect the saved audits. Excluding reading profiles and axioms also limits which effects this variant can detect.

The generator receives at most 600 words of whole memory entries. Every condition uses the same single-proposal generator, prompt, temperature and paired repeat seeds. This experiment does **not** invoke the response-choice bonus or selector from the influence experiment. It isolates the influence of memory delivery and sleep on direct generation; it does not measure every organ feature together.

## Prepare and collect

Run from the repository root with the existing `.selmem` runtime configuration and provider credentials. The frozen preview fixture is already in the repository; there is no need to encode the source dataset again outside this experiment.

```bash
bash experiments/genesis/run-creative-preview-guarded.sh \
  experiments/genesis/creative-preview.json \
  experiments/genesis/results/creative-preview-003 --prepare-only

# Inspect preview-design.json, then start the paid collection:
bash experiments/genesis/run-creative-preview-guarded.sh \
  experiments/genesis/creative-preview.json \
  experiments/genesis/results/creative-preview-003 --resume
```

The first command builds and records the design without any model calls. The second starts fresh encoding. After any interruption, repeat the second command. Do not rebuild with changed code or settings during a collection. Resume validates the fixture, source and binary hash, provider/model/reasoning and memory configuration. Prepared output must also be resumed with the identical binary/settings.

Defaults: provider `grok`, model `grok-4.3`, explicit reasoning `none`, memory temperature zero, cumulative ceiling **1,500 HTTP attempts**. Environment overrides: `SELMEM_CONFIG`, `GENESIS_PREVIEW_PROVIDER`, `GENESIS_PREVIEW_MODEL`, `GENESIS_PREVIEW_REASONING`, `GENESIS_PREVIEW_HTTP_LIMIT`. The launcher defaults release codegen units to one and honors an existing `CARGO_PROFILE_RELEASE_CODEGEN_UNITS` setting.

The attempt ceiling counts interpretation, sleep, retrieval, guard, generation and retry calls together. It is neither a dollar cap nor a completion guarantee. Only an increased ceiling is allowed on resume, with an amendment record. Do not change other settings. Latency, token usage, capacity failures and state-dependent memory calls make final duration/cost unknown before collection. Historical preview measurements are reference points, not current prices or a promise for this variant.

Journaled operations, exact semantic-annotation caches, retelling/guard stages and creative generations are retained. A committed retelling is reused if its guard was interrupted. Model/network failures pause collection; malformed retelling/guard content is preserved and causes an explicit gist fallback, without an extra repair model call. Source checkpoints remain immutable. The launcher archives both completed and interrupted output and preserves the command's failure exit status.

## Inspect and rate

`preview-design.json` and `preview.json` identify the guarded protocol. `contexts/`, `guarded-readouts/`, `guarded-stages/`, `responses.jsonl`, `http.jsonl`, and checkpoint inspections retain selection, fallback, provenance and attempt evidence. Read the actual trace/sleep changes before attributing a proposal difference to sleep.

Give only `blind-proposals.json` to at least two independent judges. Keep the private key, conditions, contexts and logs hidden. Follow `CREATIVE_PREVIEW.md` for the unchanged utility/originality/constraint rating format. Existing `tools rate-preview` analysis preserves the new protocol metadata and explains the intervention labels. Rating can also be run directly with `cargo run --release --bin selmem-genesis -- tools ...` if `run.sh` is unavailable.

The primary contrasts remain full minus empty/RAG, sleep versus the equal-time no-sleep control, and guarded retelling versus the same selected post-sleep gists. Two histories are two source units, not sixty independent lives. Scores are exploratory ordinal summaries, not proof of worldwide novelty, unconsciousness, causal source recombination or success of the larger experiment.

## Offline rehearsal

```bash
SELMEM_TEMP=0 SELMEM_REASONING=none cargo run --release --bin selmem-genesis -- \
  creative-preview --guarded --mock \
  --fixture experiments/genesis/creative-preview.json \
  --out /fresh/path/guarded-preview-mock --max-http-calls 1500
```

Mock results verify workflow only. They provide no creativity evidence, API-shape guarantee, or paid cost/latency estimate.
