# Experiments

Published report: [**REPORT.md**](REPORT.md) (method, tables, four excerpts from pair `001`).

This folder is the public record of the SelMem benches. The white paper states the organ and the conclusions. Numbers, scripts, and how to replay a cell live here.

| What | Stimulus | Runner | Offline | Grok |
| --- | --- | --- | --- | --- |
| Bifurcation (one marked hour) | `data/bifurcation.json` | `examples/bifurcation` | yes | 1 pair |
| Split lives (five hours each side) | `data/divergence.json` | `examples/divergence` | yes | not run |
| Erasure (trivia vs aversion) | `data/erasure.json` | `examples/erasure` | yes | not run |
| v0.1 C0 / C1 / C2 | `data/v01.json` | `examples/benchmark` | yes | 10 pairs × k=24 and k=8 |
| Persist, one hour then five same-schema | `data/v01_persist.json` | `examples/persist` | yes | Grok n=1, then P0 n=5 |
| Persist rumination (same meeting ×5, pinned) | `data/v01_ruminate.json` | `examples/persist --ruminate` | yes | Grok n=1, then P0 n=5 |
| P0 ablations (C1, C3, C2, C2−S/R/L/G) | persist + ruminate scripts | `examples/persist` | yes | n=5 each, seed 1, S/N |

C3 is a last-k summary plus **one** profile line (not the five T₀ paragraphs). Persist script: 12 dull days, five same-schema hours, 8 posts. Sparse probes: t0 and post+8 only. Does not rewrite `data/v01.json`.

**P0 cells.** Same scripts, isolated probes. `OrganCut` is runtime-only (not in the vault).

| Cell | What is cut |
| --- | --- |
| C0 | no book |
| C1 | last-k verbatim |
| C3 | rolling summary + one profile line |
| C2 | full organ |
| C2NoSleep (`c2_nosleep`) | every `sleep()` skipped |
| C2NoRecon (`c2_norecon`) | `apply_reconsolidation` skipped |
| C2NoLadder (`c2_noladder`) | night does not mint motif / belief / trait |
| C2NoGround (`c2_noground`) | no pull toward core at recall or night rewrite |

JSON now logs `pulled_*`, `recon_*`, `marker_a` / `marker_b` per instant, and `marker_last_*` on the pair. `D_speak` stays a log. `h2_holds` is still book-only. `marker_holds` is A naming T₀ after the shared posts.

```bash
./run.sh test --test benchmark
./run.sh run --release --example persist -- --pairs 5 --seed 1 --last-k 8 --out selmem-persist-p0-n5.json
./run.sh run --release --example persist -- --ruminate --pairs 5 --seed 1 --last-k 8 --out selmem-ruminate-p0-n5.json
```

`benchmark --p0` on v0.1 was not run. Persist + ruminate already isolate the cuts. Official `marker_last` under-counts C2 once sleep has replaced T₀ words with *wound / presence*; read the late-probe replies. Tables: [REPORT.md](REPORT.md) §7.

Isolated `speak` is the pre-stance path again (retrieved scenes + living axioms). `recall/stance.rs` stays as the ablation that showed A≈B once the scene is withheld.

```bash
./run.sh test --test benchmark
./run.sh run --release --example persist -- --pairs 1 --last-k 8 --out selmem-persist-repeat.json
./run.sh run --release --example persist -- --ruminate --pairs 1 --last-k 8 --out selmem-persist-ruminate.json
```

## Replay the four Grok dumps

Same `.selmem` for all four:

```
llm=https://api.x.ai/v1/chat/completions
model=grok-4.3
api_key=xai-…
reasoning=none
temp=0
http_timeout=60
```

```bash
cp data/config.example .selmem   # then fill api_key
unset SELMEM_QUICK
./run.sh build --release --example benchmark
```

Header on a live run must show `LLM https://api.x.ai/v1/chat/completions model=grok-4.3` and the `last_k=` of that file. Without `llm=` this is `RuleNarrator` and the wording will not match.

| File | What was in it | Command |
| --- | --- | --- |
| `selmem-v01.json` | smoke, 1 pair, C0+C2, k=24 | `./run.sh run --release --example benchmark -- --pairs 1 --seed 1 --last-k 24 --out selmem-v01.json` |
| `selmem-v01-n10.json` | 10 pairs, C0+C2, k=24, post+8 replies empty | `./run.sh run --release --example benchmark -- --pairs 10 --seed 1 --last-k 24 --out selmem-v01-n10.json` |
| `selmem-v01-n10-c1.json` | 10 pairs, C0+C1+C2, k=24, post+8 replies empty | `./run.sh run --release --example benchmark -- --pairs 10 --seed 1 --last-k 24 --out selmem-v01-n10-c1.json` |
| `selmem-v01-k8-n10.json` | 10 pairs, C0+C1+C2, k=8, post+8 replies present | `./run.sh run --release --example benchmark -- --pairs 10 --seed 1 --last-k 8 --out selmem-v01-k8-n10.json` |

The current example always writes C0, C1 and C2. Replaying the first two files therefore adds C1 cells that were not in the original dumps. Fingerprint on C2 will match; wording will not (Grok is not a seed). Post+8 probe text is only filled by the current binary — that is why `selmem-v01-n10.json` and `selmem-v01-n10-c1.json` have empty `post[].replies`.

Offline contract (no API):

```bash
./run.sh test --test benchmark
```