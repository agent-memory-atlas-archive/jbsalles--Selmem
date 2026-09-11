# SelMem

Selective reconstructive memory for an LLM entity.

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
./run.sh run --release --bin selmemd -- --help
```

## Tests

Rust only. No Python suite.

| What | Where |
|---|---|
| Narrative scenes (edit these) | `tests/cases/*.json` |
| JSON runner | `tests/scenes.rs` |
| Physiology — decay, anchors, SQLite, Ebbinghaus | `tests/engine.rs` |

```bash
./run.sh test
./run.sh test --test scenes
```

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
