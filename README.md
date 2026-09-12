# SelMem

Selective reconstructive memory for an LLM entity. v0.5

An LLM maps context to the next token. A stack of unmodified facts maximises coverage, not deviation: more evidence, same average path. SelMem sculpts a particular past — forgotten, gilded, anchored — so two instances can diverge. The aim is a non-average, path-dependent continuation, not a taller log. Selection, reconstruction, sleep, identity.

**Manifest:** [WHITEPAPER.md](WHITEPAPER.md)  
**Knobs:** [PARAMETERS.md](PARAMETERS.md) — exploratory, not fitted.

Zero crates. Rust 1.75. SQLite via system `libsqlite3` (macOS SDK or Linux).

```
src/
  core/       model, profile, store
  encode/     intake, scoring, embed, affect, identity
  recall/     retrieve, narrator, http
  dream/      night, drift, singularite
  persist/    file, sqlite
  net/        api, httpx, ui
  engine.rs   the loop
  bin/        selmemd, selmem-chat
```

## Loop

```
experience → interpret → identity paint → gate
        ↓                         ↑
   lived book + sealed archive    |
        ↓                         |
 remember / speak (meaning can move)
        ↓
      sleep
 weather · rewrite · merge · extinguish
 motif → belief → trait → who_am_i
        ↓
   next experience is already colored
```

The model never sees the archive. Only gist, core, schema, affect, fidelity, mood, living axioms.

Claire and Silas are not characters. They are two sensitivities (`tender` / `austere`) on the same corpus. After nights they are not the same past.

## Build

Zero Cargo crates. Persistence is a vault, not the memory: the organ lives in RAM (`MemoryStore`); on `save` it is dumped, on `open` it is reloaded. The model never talks to the vault.

Two backends, same `Snapshot` (`profile`, `mood`, `store`):

| Path | Backend |
|---|---|
| `.db` / `.sqlite` / `.sqlite3` | system `libsqlite3` (prepared statements, `BEGIN IMMEDIATE`) |
| anything else | flat `SELMEM1` file |

IDs are `{prefix}_{pid}_{n}`. The counter is raised on load for both backends. Dropped events leave no archive. Orphans are pruned on sleep and SQLite load.

Link is dynamic against the system `libsqlite3`.

| OS | What you need |
|---|---|
| macOS | Xcode Command Line Tools (`xcode-select --install`). The SDK already ships sqlite3. If link still fails: `brew install sqlite` — `./run.sh` adds Homebrew’s lib path. |
| Linux | `libsqlite3` (the `.so.0` runtime is enough). `./run.sh` invents a `libsqlite3.so` stub when `-dev` is missing. |

Use `./run.sh` instead of bare `cargo` so those paths are set. Flat `.selmem` files do not need SQLite at all.

Apple Silicon and Intel are both fine. Bind `127.0.0.1` or `0.0.0.0` as usual.

```bash
./run.sh test
./run.sh run --release --example compare
./run.sh run --release --example llm_night
./run.sh run --release --example bifurcation
./run.sh run --release --bin selmemd -- --help
```

## Tests and experiments

Rust only. No Python suite. Always use `./run.sh` (not bare `cargo`) so sqlite link flags are set.

Scripts in `data/*.json` are the frozen stimuli. Edit those if you change a protocol; do not rewrite them mid-run. Later prompts in a script never name the marked event.

Published numbers: [WHITEPAPER.md](WHITEPAPER.md) § Experiments.

### Unit tests (no network)

```bash
./run.sh test
./run.sh test --test scenes
./run.sh test --test engine
./run.sh test --test bifurcation
./run.sh test --test divergence
./run.sh test --test erasure
./run.sh test --test json_parse
```

| What | File | Stimulus |
|---|---|---|
| Narrative scenes | `tests/scenes.rs` | `tests/cases/*.json` |
| Decay, anchors, SQLite, Ebbinghaus | `tests/engine.rs` | inline |
| Bifurcation A/B (rules) | `tests/bifurcation.rs` | `data/bifurcation.json` |
| Split lives (rules) | `tests/divergence.rs` | `data/divergence.json` |
| Erasure: trivia vs repeated aversion | `tests/erasure.rs` | `data/erasure.json` |
| Chat JSON walker | `tests/json_parse.rs` | fixtures in the test |

These use `RuleNarrator`. They must stay green offline.

### Replay a protocol (print the probes)

Same scripts as the unit tests, with the full report on stdout:

```bash
./run.sh run --release --example bifurcation
./run.sh run --release --example divergence
./run.sh run --release --example erasure
```

`Finished in 0.00s` means Cargo reused an old binary. After pulling code:

```bash
touch src/experiment.rs examples/bifurcation.rs
./run.sh build --release --example bifurcation
```

You should see `Compiling selmem`.

### Same protocols with a live model

Only `speak` / `reply` hits the HTTP API (`SpeakOnlyHttp`). Encode, sleep and reconstruct stay on the organ.

```bash
export SELMEM_LLM='https://api.x.ai/v1/chat/completions'
export SELMEM_MODEL='grok-4.3'
export SELMEM_API_KEY='xai-…'          # never commit this
export SELMEM_REASONING=none           # skip thinking tokens on grok-4.3
export SELMEM_TEMP=0
export SELMEM_HTTP_TIMEOUT=60
unset SELMEM_QUICK

./run.sh run --release --example bifurcation
./run.sh run --release --example divergence
```

OpenAI-compatible endpoints work the same (`SELMEM_LLM` = `…/v1/chat/completions`). Ollama:

```bash
export SELMEM_LLM='http://127.0.0.1:11434/v1/chat/completions'
export SELMEM_MODEL='llama3'
unset SELMEM_API_KEY
```

Check the endpoint before a 100-call run:

```bash
curl -sS --max-time 30 "$SELMEM_LLM" \
  -H "Authorization: Bearer $SELMEM_API_KEY" \
  -H "Content-Type: application/json" \
  -d "{\"model\":\"$SELMEM_MODEL\",\"reasoning_effort\":\"none\",\"messages\":[{\"role\":\"user\",\"content\":\"dis: ok\"}]}"
```

| Variable | Default | Role |
|---|---|---|
| `SELMEM_LLM` | unset | chat URL; unset = rules only |
| `SELMEM_MODEL` | — | model id |
| `SELMEM_API_KEY` | unset | `Authorization: Bearer` |
| `SELMEM_REASONING` | `none` | xAI `reasoning_effort` (`none`/`low`/… or `off` to omit) |
| `SELMEM_TEMP` | `0` | sampling temperature |
| `SELMEM_HTTP_TIMEOUT` | `60` | curl `--max-time` seconds |
| `SELMEM_QUICK` | unset | `1` = salient condition only, 2 probes, one persist window |
| `SELMEM_PROBES` | all / 2 if quick | how many probe questions to speak |

A line `narrator: 0/10 answers are the RuleNarrator template` means the model answered. `Cela me revient` means the HTTP call failed and the rules ran. `selmem LLM reply failed:` prints the error.

Full bifurcation ≈ 100 `reply` calls (5 probes × 2 agents × 5 snapshots × 2 LLM conditions). Quick mode ≈ 12. Ablation `salient-no-consolidation` never calls the model.

Fingerprint distance is on the book (0 = clones). Speak distance is wording overlap; two clones with the same book already differ under Grok (~0.7). Do not read H₁ off Δspeak alone.

## Run

```bash
./run.sh run --release --bin selmemd -- \
  --bind 0.0.0.0:7420 \
  --path claire.db \
  --name Claire \
  --profile tender \
  --ground-overlap 0.18 \
  --ground-strikes 3 \
  --narrator-firmness 0.42 \
  --token secret \
  --llm https://api.openai.com/v1/chat/completions \
  --model gpt-4o-mini \
  --api-key "$SELMEM_API_KEY"
```

UI: `http://IP:7420/` — paste the token at the top, talk.  
Local Ollama: `--llm http://127.0.0.1:11434/v1/chat/completions --model llama3`

Behind nginx:

```
location / { proxy_pass http://127.0.0.1:7420; proxy_read_timeout 90s; }
```

```bash
export SELMEM_LLM=https://api.openai.com/v1/chat/completions
export SELMEM_MODEL=gpt-4o-mini
export SELMEM_API_KEY=sk-...
export SELMEM_EMBED=https://api.openai.com/v1/embeddings
export SELMEM_TOKEN=secret
```

No endpoint: `RuleNarrator` + hashed vectors. The organ still runs.

One thread per connection. `/health` and `/` do not take the memory lock. `/turn` is serialized on the organ (one writer). Auth is `Authorization: Bearer` only — not `?token=`.

## HTTP

| Method | Route | Role |
|---|---|---|
| GET | `/health` | liveness (no token, no lock) |
| GET | `/who` | living axioms (trait > belief > motif) |
| GET | `/lineage?schema=` | history of a belief |
| GET | `/mood` | mood |
| GET | `/profile` | knobs (`τ`, embellish, ground, …) |
| POST | `/profile` | set knobs |
| GET | `/audit?id=` | sealed verbatim (human debug, never the model) |
| POST | `/live` | encode |
| POST | `/remember` | reconstruct |
| POST | `/speak` | embodied reply |
| POST | `/turn` | live + reply |
| POST | `/sleep` | consolidate |
| POST | `/save` | flush |

## Rules that hold

- Two books: lived narrative / archive. The model never reads the second. Grounding rewrites the gist toward the *core*; it never injects the journal.
- Fading traces (cold / myth / low hold) may warp without a ceiling. Living traces are pulled back in proportion to `narrator_firmness`.
- Latent forgetting: the scene drops out of recall; schema and affect still color the next event.
- Two channels: `self` is sculpted, `world` is not.
- Forgetting by default. Encoding threshold.
- Unlabelled events get lexical affect (FR+EN), then identity, then optional LLM `interpret` on the live sentence.
- Living axioms color the next encoding before the gate.
- Cherished memories embellish. Recalled disgust amplifies. Neglected disgust extinguishes.
- Recall can shift *meaning* (valence under current mood), not only wording.
- Nearby episodes fuse into myth. Heavy anchors do not merge.
- Axioms climb: 2 traces → motif, 3+ → belief, aligned beliefs → trait. Lineage stays.
- Two profiles on the same corpus diverge. Fingerprint uses founders, traits, contradictions.
- Proxy for originality: reconstruction sticks less than the unmodified log; clones are not interchangeable; `world` facts survive (`originality_is_path_dependent_not_a_taller_log`).
- Persistence: `.db` (SQLite) or `.selmem` (flat file).

## Rust API

```rust
let mut mem = SelectiveMemory::open("claire.db", EntityProfile::tender("Claire"))?;
mem.live_with(input);
let recalled = mem.remember("that evening");
let _ = mem.speak("do you remember?");
mem.sleep();
mem.who_am_i();
mem.lineage("loyalty");
singularity_distance(&fingerprint(&a), &fingerprint(&b));
```

## What this is not

Not RAG. Not a vector database. Not a personality in a system prompt.  
Not a neocortex and not a brain. An executive–autobiographical loop around a next-token transducer.  
No local neural encoder ships in-tree: pass `--embed` if you have one.

## How it's built

Humans own architecture, concepts and governance. Models draft code, tests, and prose. Generated patches are reviewed.
