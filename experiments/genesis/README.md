# GENESIS

A set of controlled exploratory assays around the SelMem organ. Start with the [result ledger](../RESULTS.md), [creativity findings](../CREATIVITY_RESULTS.md) and [replay rules](../REPRODUCIBILITY.md). The protocols below deliberately test different interventions; do not pool their samples.

## Qualification and cumulative learning

```bash
bash experiments/genesis/run-positive-control.sh experiments/genesis/results/positive-new
bash experiments/genesis/run-evidence-learning.sh \
  experiments/genesis/results/positive-new/grok \
  experiments/genesis/results/evidence-new
```

The qualification panel requires 48/48 oracle choices for this specific decision task. The historical reported Grok run passed; GPT-mini did not. Learning then compares observation support OFF/ON on two twelve-report histories. Retained historical reports show 8/16 versus 16/16 oracle agreement for native and strict recall. The benefit is explicit evidence retention, not a proven effect of reconstruction or creativity. Qualification sources must be real, complete and model-matched.

## Published creative preview

```bash
bash experiments/genesis/run-creative-preview-guarded.sh \
  experiments/genesis/creative-preview.json \
  experiments/genesis/results/creative-preview-new --prepare-only
# After inspecting the prepared design, start paid collection:
bash experiments/genesis/run-creative-preview-guarded.sh \
  experiments/genesis/creative-preview.json \
  experiments/genesis/results/creative-preview-new --resume
```

The [guarded protocol](CREATIVE_PREVIEW_GUARDED.md) encodes 788 turns from two complete histories once, applies ten nights per history, and collects 60 proposals across five conditions. Task-conditioned retrieval is followed by task-blind retelling and provenance checking. All conditions use matched direct generation; response-choice selection is not invoked.

The published preview completed 60 proposals in 887 HTTP attempts. This is a collection result, not proof of creativity superiority. The older native preview remains available through `run-creative-preview.sh`; the guarded and native readouts must not be treated as the same intervention.

## Replays and response selection

- [Fixed-selection replay](CREATIVE_REPLAY.md): reuse saved states and selected IDs; add a task-blind reteller/guard and a no-memory two-step comparator.
- [Structured delivery rerun](CREATIVE_MEMORY_REPLAY.md): reuse the earlier accepted readouts, change final memory delivery, and collect another separate lot.
- [Memory-influence diagnostic](MEMORY_INFLUENCE.md): two controlled histories, six tasks, three conditions, two repeats. Analyze candidate selection independently from generation and quality.

The recorded replays completed 72 answers each. Influence 005 reused 24 direct answers, 48 candidate pools and 48 quality results, then made 33 grounding attempts. These lots share inputs or outputs and are not independent replications.

## Other retained protocols

[Sleep](SLEEP.md) compares matched no-sleep/normal/deep states; its historical results and persistence defect are documented in the ledger. [Creative transfer](CREATIVITY.md) retains the original pilot and lesion follow-up. [Creative life](CREATIVE_LIFE.md) retains both the 600-proposal economy plan and 6,480-proposal full plan for later runs. They are plans, not completed creativity results.

## Offline checks and rating

```bash
./run.sh test --all-targets -- --test-threads=1
./run.sh test --doc
./run.sh run --release --bin selmem-genesis -- --help
bash experiments/genesis/tools.sh
```

The [Rust tools](RUST_TOOLS.md) prepare pinned dataset fixtures, generate blank rating forms, combine independent ratings and analyze results. At least two independently completed judge forms are required for the intended creativity assessment. Distinct judge names alone do not establish independence.

Give judges only the original blind proposals, their blank form and the rubric. Keep conditions, source memories, pipeline scores and analysis hidden. Model self-evaluation and the earlier single unblinded assistant review are not independent blind evidence.

## Contextual self-model diagnostic

The [self-model pilot 3](SELF_MODEL_PILOT_V3.md) separates self-dependent admission,
learning, freezing and model swapping before another creativity collection.
Start with `bash experiments/genesis/run-self-pilot-v3.sh OUT --mock`. Its 128 mock
choices validate the circuit and are not behavioral evidence. Real collection
requires explicit provider configuration and a new output directory.

The recorded Grok v3 lot completed 128 choices. Self swaps and correction
changed contextual admission, and evolving beliefs retired and later reformed
on separate new-action copies. Full loop vs memory-only differed on 2/32
matched choices; full loop vs evolution without admission feedback on 0/32.
These are two scripted histories with limited decision effects, not a
demonstration of durable individuality. [Results and archive provenance](SELF_MODEL_RESULTS.md).
