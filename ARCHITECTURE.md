# Architecture

SelMem is a Rust memory library with a daemon, terminal client and two experiment adapters. The current organ includes optional cumulative observation support; earlier benchmark source hashes identify historical versions, not a frozen current organ.

| Directory | Responsibility |
| --- | --- |
| `core/` | Traces, immutable cores, provenance, profiles, working conversation, co-recall graph and bounded observation support |
| `encode/` | Interpret, validate, paint, split and select incoming experiences |
| `recall/` | Select traces, reconstruct, judge against core, ground and optionally reconsolidate |
| `dream/` | Accessibility decay, consolidation, schemas, beliefs, merging and release |
| `persist/` | File and SQLite snapshots |
| `net/`, `config.rs` | Runtime interface/API and provider configuration |
| `bin/selmem-genesis/` | Positive-control and OFF/ON learning orchestration |
| `bin/selmem-locomo/` | External strategy benchmark adapter |

`engine.rs` coordinates the organ. It does not contain an experimental world or prescribed behavior. `WorkingTalk` is temporary; sleep commits recorded turns through encoding and clears the working frame.

## Records and grounding

A lived trace contains gist, core, schema, affect, confidence, accessibility and provenance. Gist can change; the core supplies a stable grounding reference. The sealed archive is an audit record and is not supplied to the narrator or consulted for runtime grounding. Grounding detects deviation from core, not objective truth about the world.

State transformations carry provenance operations. Readonly recall may generate new output but must not modify the saved memory. GENESIS probes use independent snapshot clones and verify hashes.

## Independent observation support

`SelectiveMemory::set_observation_evidence(true)` enables bounded support accumulation for `EncodeInput` with a stable `source` and `observation_id`. Exact repeated propositions with new identities update an existing active trace instead of creating another trace. Repeated ingestion of the same identity and conflicting reuse of it are rejected. Recall never increments the observation count.

Near but different propositions are not positive support. An attached proposition scorer may identify a counter-observation; each alternative claim retains its own count. Unknown labels remain on the ordinary salience path. The scorer can be wrong. Distinct caller-provided identities do not themselves prove statistical independence.

Counts accompany a selected trace as labeled metadata after reconstruction, grounding and association learning. They never enter gist/core write-back and do not require archive access. Suppression/deletion of the carrier prevents rendering. GENESIS can lesion support groups independently of traces. File and SQLite snapshots preserve identities and counts.

A saturated reconstructive weight (`support_weight`) is derived from distinct observation counts. When `OrganCut::support_signal` is true (default; `no_support_signal()` lesions it), the weight modestly boosts recall score and injects a short strength line (plus optional schema prototype for Internal traces) into the narrative. The cut does not discard groups or rewrite core/gist.

This mechanism defaults off in ordinary stores. The GENESIS learning script explicitly compares off/on with the same inputs. It does not force a decision, lower the salience threshold or declare a personality. Counts are not currently transferred between merged traces; consolidation and long-term behavior require separate validation.

## Co-recall

Association learning and use during reconstruction are independent cuts. Co-recalled accepted traces strengthen links; associative cues are fallible context, not historical facts. See [CO_RECALL_ASSOCIATIONS.md](docs/CO_RECALL_ASSOCIATIONS.md). Enabling the feature does not prove it affected a particular recall.

## Experiments

GENESIS has one executable and shared configuration, provider, transport and snapshot code. Positive controls fix a decision oracle; learning compares external faithful history, admitted observations, native/strict recall and empty context. LoCoMo uses its own dataset adapter. Runtime memory algorithms are shared, not copied into experiment-specific organs.

## Axiom context persistence

Axiom family matching uses `stake_kind`, `bearer`, `loss_kind` and `stake_mark`. These fields are now retained in SELMEM1 (an optional `axiom_context` record with a length-prefixed mark) and SQLite. SQLite also persists the corresponding trace fields and absence; losing either side of the family comparison can cause repeated consolidation to create duplicate motifs after reopening.

The new reader accepts old files. Missing axiom context is recovered only from unanimous existing lived support, excluding traits and incomplete/mixed support. Explicitly saved empty context remains empty. No sealed archive or heuristic parsing of axiom prose is used. Legacy SQLite trace stakes use the same lived-gist fallback as legacy flat traces; original omitted annotations cannot be guaranteed recoverable. Existing historical duplicates are preserved rather than silently deleted. Updated SQLite schemas are migrated on load/save; old binaries should not be used to write the new format.

Regression coverage includes both containers, explicit empty fields, legacy loading and twenty forced-deep nights with reopening between each. The original sleep experiment remains an observation of the older codec, not evidence from the corrected engine.
