# Organ integrity corrections

Baseline: `b69cb5a0467f7c2ea493df82ba09f193b1b96e4d`.

This change addresses time, persistence, explicit reinterpretation, scorer requests,
embedding spaces, SQLite migrations and behavioral regressions. Archive access and
authorization are outside its scope.

## Time

Every `SelectiveMemory` has its own clock from construction. `detach_clock()` is
idempotent. Advancing one organ never advances another, including in the HTTP API.
Changing scale rebases the clock without retroactively rescaling elapsed time.
Origin, jump and scale survive file and SQLite saves. Wall time, including downtime,
continues to advance at the recorded scale; experimental runners retain their
explicit simulated origins. The legacy free clock helpers remain available for
low-level callers; use `memory.advance_hours()` for an organ.

Old attached snapshots did not necessarily contain the global jump. Loading them
cannot recover information that was never saved. Their independent clock is moved
forward, if necessary, to cover retained trace, operation, axiom and night dates.
This is a migration safeguard, not a reconstruction of the lost history.

## Persistence

Flat saves use exclusively created temporary files unique to each writer, sync the
file, atomically replace the destination, and sync its parent directory on Unix.
Failures propagate and temporary files owned by the writer are cleaned up. Concurrent
saves produce a complete snapshot; the last successful replacement wins. This does
not merge concurrent mutations. A failure after replacement, such as a directory
sync error, can leave the new snapshot present while still reporting failure.

SQLite uses full synchronous mode and an explicit transaction with rollback on early
return. Migrations inspect existing columns, propagate errors and run transactionally.
`user_version=1` identifies this schema; future versions are rejected. Opening an old
SQLite file can migrate its schema; opening an up-to-date file does not stamp the
version again. Retain backups before upgrading historical data.

HTTP mutation routes return an error if persistence fails; the terminal chat exits
with an error. In-memory mutations may already have occurred, so an error means the
operation was not confirmed durable. They are not silently reported as saved.

Both formats preserve the interpretation statement, valence and confidence. The
flat format additionally preserves archive cores, operation centers and stake marks
with spaces, and exact profile names. Declared truncated blobs fail to load. Legacy
interpretations are recovered from the latest encode or reinterpret operation when
an explicit stamp is absent. Runtime dialogue, narrator bindings and API keys remain
runtime state, not snapshot content.

## Reinterpretation and scoring

`reinterpret()` revises only the subjective interpretation and records its provenance
with the organ's clock. The gist, retained core, semantic claim, reality claim and
archive remain intact. Recalled output appends a clearly marked tentative
interpretation **after** grounding, reconsolidation and association learning. It is
available to the decision policy but cannot be reconsolidated into an observed fact
by that same recall. Read-only probes do not mutate the stored trace.

The HTTP scorer uses the narrator's token-cap policy, forwards configured reasoning
effort (omitted for `off`), and omits temperature for recognized OpenAI reasoning model
families. Its output allowance is 128 tokens with reasoning disabled and 2048 when a
reasoning effort is requested. Only the first choice's explicit `message.content`
answer is accepted. Ambiguous prose, missing answers and transport failures abstain
as `unknown`; reasoning text and sibling metadata are never substituted for a label.
Compatibility is tested with local fixtures, not paid provider requests. Provider and
model-specific reasoning support must still match the runtime configuration.

## Embeddings

The store records an embedding-space identifier. Attaching an embedder explicitly
reindexes retained gists even when dimensions match. `open()` defaults to hash96 and
reindexes saved external or unidentified legacy vectors locally; attaching a remote
embedder afterward reindexes them in that remote space and can incur API calls.
Custom embedders should override `space_id()` to identify their model/version and
configuration. Read-only recall with a different space uses lexical signals.

An HTTP embedding error or invalid vector produces an empty vector, not a hash vector
masquerading as remote output. Lexical signals remain available, including novelty at
encoding. A live recall refreshes a vector only if it changed the stored gist;
read-only recall never updates vector caches. A backend silently changing its model
behind the same endpoint/model identifier requires an explicit reindex.

## Offline validation

```sh
./run.sh test --lib --test organ_integrity_fixes --test axiom_context_persist
./run.sh test --all-targets
./run.sh test --doc
```

Regressions cover independent and nested clocks, clock continuity, save/load in both
formats, atomic concurrent flat writes, HTTP save errors, SQLite rollback and legacy
migrations, strict scorer answers and request parameters, equal-dimensional backend
switches, embedding outages, read-only probes, incompatible merge stakes and the
30-day merge boundary. Sleep tests cover shallow maintenance and twenty deep nights
with reopenings, factual anchors, archive content, interpretation persistence and
valid deduplicated axiom support. Existing grounding, association and consolidation
tests remain part of the suite.

These tests establish implementation invariants. They do not establish a positive
GENESIS behavioral or creativity effect, and no paid scientific run is part of this
change.
