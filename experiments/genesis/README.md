# GENESIS

Two maintained experiments, one Rust executable. [Protocol and limitations](protocol.md), [result ledger](../RESULTS.md), [native observation support](../../ARCHITECTURE.md).

## 1. Qualify the decision model

```bash
bash experiments/genesis/run-positive-control.sh experiments/genesis/results/positive-new
```

Uses `.selmem` or `SELMEM_CONFIG`, Grok 4.3 and GPT-4o-mini-2024-07-18 by default. `GROK_MODEL` and `GPT_MODEL` override models. Each receives 48 decisions: two names, reliable/unreliable/empty histories, two costs, two option orders and two repetitions. 96 requests are planned; the ceiling is 120 attempts including retries. No memory, scorer or reconstruction calls occur.

A complete run qualifies only with 48/48 oracle choices. This strict engineering gate was fixed before the retained run; it is not a statistical threshold. Mock results cannot qualify. Failed qualification remains a result and does not prevent the other provider from being tested. Transport/schema failures preserve partial outputs.

## 2. Compare observation support OFF/ON

```bash
bash experiments/genesis/run-evidence-learning.sh \
  experiments/genesis/results/positive-new/grok \
  experiments/genesis/results/evidence-new
```

Only a real, complete, qualified positive-control source is accepted. Model names must match that source. Grok is the default; GPT-mini did not qualify in the retained run. This command does not automatically rerun qualification or replay an old life.

Each cut receives 24 inputs (two twelve-report histories), four readonly final recalls and 72 decisions. Combined: 48 inputs, eight readouts and 144 decisions. Internal calls vary; ceilings are 800 attempts per cut and 1600 total, including retries. The retained pair used 475 attempts. A ceiling is not an expected request count.

Compare full faithful history, admitted observations, native recall, strict recall and empty context. Support updates retain information without a new trace: `kept=true`, `new_trace=false`, `salience_evaluated=false`; score=0 means not evaluated. Ordinary new-trace salience thresholds are unchanged. No sleep occurs within these twelve-event lives.

Outputs include design/configuration, fixtures, encoding receipts, life/clone snapshots, readout audits, decision schedule, raw responses and HTTP journal. Fresh outputs are required. Both scripts create `.tar.gz` evidence packages after successful completion; failed runs retain their directories and logs.

## Direct and offline commands

```bash
./run.sh run --release --bin selmem-genesis -- --help
./run.sh run --release --bin selmem-genesis -- positive --mock --out /tmp/positive-check
./run.sh run --release --bin selmem-genesis -- learning --mock \
  --source /tmp/positive-check --out /tmp/learning-check --observation-evidence on
./run.sh test --bin selmem-genesis
```

`--config FILE` loads the runtime file before either command. The default is normal `.selmem` discovery. Progress is verbose; `operation #` is nested work, `HTTP #` counts actual attempts. WAIT messages are heartbeats, not additional calls. Mock decision choices are prescribed plumbing and provide no scientific evidence.

## Migration

The separate `selmem-genesis-positive` and `selmem-genesis-learning` executables are now `selmem-genesis positive` and `selmem-genesis learning`. Gate/policy/qualification/recall launchers and contrast/presentation/sensitivity/content/threshold troubleshooting commands have been removed. The oversized prepare/run/replay campaign scaffold and unexecuted confirmatory/pilot grids have also been retired. The next singularity/creativity protocols remain a research plan, not executable claims.

Existing evidence is readable and must keep its original manifests. Place qualified `positive-001/grok` outside or back under the ignored results directory if reusing it. Do not run the old suite script. The native organ's algorithms and observation-support implementation are unchanged by this cleanup.

Legacy library exports for benchmark/experiment/fork demonstrations have been removed. The core live/remember/speak/sleep API, daemon and terminal client remain.
