# SelMem on AMA-Bench

Private adapter. AMA-Bench is a
poor fit for SelMem and is not part of the persist claim. This bench rewards the transcript; the organ compresses it...

AMA-Bench scores QA on agent trajectories (`memory_construction` then
`memory_retrieve` → context string → their LLM answers). The questions
want the tool journal: exact SQL at step 18, grep at step 8, DOM id that
opened a submenu. Last-k is the store those items assume. The salience
gate and one night are built for lived hours; they drop or rewrite step
ids on purpose. A low C2 score here does not falsify T₀ path dependence
(see persist P1 / Drop in [../REPORT.md](../REPORT.md) §8–§9). A high
last-k score does not mean last-k is the better organ. It means the bench
is a transcript exam.

Ran 23 Sep 2026, open-end, Grok-4.3 as model and judge, **same three
episodes** (129 TEXT2SQL, 169 SWE, 186 Web), 36 questions:

| | last-k | static | C2 |
| --- | --- | --- | --- |
| All | **0.50** | 0.28 | 0.19 |
| SQL 129 | **0.75** | 0.33 | 0.33 |
| SWE 169 | 0.08 | 0.33 | 0.17 |
| Web 186 | **0.67** | 0.17 | 0.08 |

Last-k sees the end of the SQL and Web logs. SWE asks about steps 2–9
while last-k starts near step 78 (0.08). Static/C2 answer “no details on
step 18.” Do not scale to 208 episodes to chase a %.

That is the only external QA number in this tree. It is a side table.

| Cell | What it is |
| --- | --- |
| `selmem_lastk` | last 8 packed turns, no organ |
| `selmem_static` | gate, freeze, stored gist, read-only retrieve |
| `selmem_c2` | gate + one sleep + reconstructive readout, read-only retrieve |

Retrieve returns scenes + axioms. It does not speak. Their judge still
answers the question. Log packs / kept / traces from construct stdout.

## Build the organ CLI

From the SelMem crate root:

```bash
./run.sh build --release --example ama
cp target/release/examples/ama /tmp/selmem-ama
chmod +x /tmp/selmem-ama
export SELMEM_AMA_BIN=/tmp/selmem-ama
```

Smoke, no AMA-Bench tree:

```bash
printf 'Step 1\nOpened the ticket about the missing invoice.\n\nStep 2\nClosed it as duplicate.\n' \
  | /tmp/selmem-ama construct --mode c2 --book /tmp/ama-smoke.selmem
/tmp/selmem-ama retrieve --book /tmp/ama-smoke.selmem --question "Why was the ticket closed?"
```

## Drop into AMA-Bench

```bash
git clone https://github.com/AMA-Bench/AMA-Bench.git
cd AMA-Bench
cp /path/to/selmem/experiments/ama_bench/selmem_method.py src/method/selmem_method.py
```

In `src/method_register.py` add to `_LAZY_REGISTRY`:

```python
"selmem_c2": ("src.method.selmem_method", "SelMemC2Method"),
"selmem_static": ("src.method.selmem_method", "SelMemStaticMethod"),
"selmem_lastk": ("src.method.selmem_method", "SelMemLastKMethod"),
```

Dataset:

```bash
huggingface-cli download AMA-bench/AMA-bench --repo-type dataset --local-dir ./dataset
```

One domain first (open-end, whatever `src/run.py` accepts as `--subset`).
Same LLM and judge for all three methods. Record tokens and wall time.

```bash
export SELMEM_AMA_BIN=/tmp/selmem-ama

python src/run.py \
  --llm-config configs/your_api.yaml \
  --subset openend \
  --method selmem_lastk \
  --test-dir dataset/test \
  --judge-config configs/llm_judge_api.yaml
```

Then `selmem_static`, then `selmem_c2`. Do not mix judges.

## How to read a number

- last-k ≥ C2 and static on the same judge: the organ compressed away
  facts the questions need. **This is the observed order (0.50 / 0.28 /
  0.19).** Expected on tool traces. Not a falsifier of T0.
- C2 ≈ static or worse: sleep did not help this QA set and can blur ids
  further. Same qualitative finding as LongMemEval — consolidation is
  store size, not step-id accuracy.
- C2 ≫ last-k: did not happen. If it ever does, still a side table.

Never a single % as “SelMem on AMA-Bench.” The persist claim lives in
[../REPORT.md](../REPORT.md), not here.
