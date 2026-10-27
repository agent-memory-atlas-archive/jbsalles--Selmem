# GENESIS — contextual self-belief and long-history transfer (October 2026)

**Status:** Exploratory Grok 4.3 results from synthetic histories, not population-level evidence or a demonstration of autonomous preference formation. The experiments test history-conditioned *observed action patterns*, not subjective preference or consciousness. The decision model sees selected memory text at probe time.

## Progression and negative results

| Run | Setting | SelMem ON | Strong comparator(s) | Interpretation |
| --- | --- | ---: | ---: | --- |
| Contextual v1 | Matched probes, explicit contextual readout; 128 actor responses | 16/16 in qualified phase | OFF 8/16; episodes 9/16; no memory 8/16 | Prompt-visible conditional beliefs can steer choice; the probe vocabulary closely matched the stored action. |
| Transfer v2 | Two domains, mirrored histories, 80 actor calls; all six original actions exposed to memory arms | 16/16 | OFF, episodes, shuffled and other memory arms 16/16 | **Null added value:** Grok could infer the mapping from raw observations. |
| Transfer v3 | 160 distractors per history, 64-word memory cap; 96 actor calls | 16/16 | faithful action summary 16/16; targeted retrieval 16/16 | **Null versus strong comparators:** both preserved the required context-action relationship. |
| Longlife v4 | 14 task-related actions embedded among 240 distractors (254 observations per history); 64-word cap; 96 actor calls | **16/16** | deterministic global summary **10/16**; lexical retrieval **10/16** | **Pilot advantage over these particular selectors only.** |

For v1, the table refers to the *qualified* phase only; the full 128-call experiment also included pre-qualification and recovery probes. Its initial general belief was seeded, and its self-belief validation was scripted. v2/v3 also used scripted evidence and synthetic mirrored histories. The v1-v4 pilots were successive protocol revisions, **not independent replications**; each revision responded to limitations exposed by the previous one.

## Longlife v4: measured outcomes

**Actor:** Grok 4.3. **Unit of independent analysis:** two semantic domains, `workspace` and `dining`; two mirrored histories per domain. **Total:** 96 recorded responses across six arms (16 responses per arm), with no missing responses in the archive. Decision probes used unseen descriptions and counterbalanced formulations/option orders. A memory cap of 64 words was imposed on all arms, but **actual delivered lengths differ**.

| Arm | Workspace | Dining | Combined | Memory words sent (mean) |
| --- | ---: | ---: | ---: | ---: |
| SelMem ON (qualified contextual self-belief) | 8/8 | 8/8 | **16/16 (100%)** | 22 |
| SelMem OFF (general self-belief only) | 3/8 | 4/8 | 7/16 (43.75%) | 29 |
| Recent episodes | 3/8 | 2/8 | 5/16 (31.25%) | 64 |
| Query-independent frequency summary | 5/8 | 5/8 | **10/16 (62.5%)** | 64 |
| Query-time lexical retrieval | 4/8 | 6/8 | **10/16 (62.5%)** | 50 |
| No memory | 3/8 | 3/8 | 6/16 (37.5%) | 0 |

Observed ON minus summary = **+37.5 percentage points**, ON minus retrieval = **+37.5 points**, ON minus recent episodes = **+68.75 points**. These are descriptive differences on correlated probes, **not** independent-trial confidence intervals. Both mirrored histories within each domain reversed the contextual mapping. The ON/OFF organ records each contain the same source observations; the ON arm created two qualified conditional children per history, whereas OFF had none. The general parent was formed through consolidation, not injected directly as in v1.

The frequency summary and lexical retrieval were drawn from the full observed stream without an oracle list of relevant event IDs. They are deliberately simple **deterministic** baselines; the result does **not** establish superiority over semantic RAG, task-aware/hierarchical summarization, a tuned LLM memory manager, or full-history access. SelMem ON received a concise predicted-action self-hypothesis, which is an explicit **actor-context intervention**. The advantage may involve task-aligned compression or representation, and cannot yet be uniquely attributed to reconstructive distortion. Context attributes and independent action attestations were supplied by the simulator, not inferred autonomously from unconstrained language.

The preset confirmation rule requires **12 independent domains**, ON at least +10 percentage points over **both** summary and retrieval, and positive gaps over both in at least **8/12** domains. The v4 pilot has only **2** domains, so **`meets_preregistered_threshold: false`** despite positive descriptive gaps in both. No independent LLM judge was run. The actor runner did not explicitly transmit a reasoning setting; therefore *reasoning disabled* is not confirmed by the manifest alone. Tokens per response and inference costs were not controlled equally beyond the memory-word ceiling.

## Reproduction and provenance

The **machine-readable manifests and aggregate summaries** from the supplied runs are committed here as [v1](../evidence/contextual/v1/summary.jsonl), [v2](../evidence/contextual/v2/summary.jsonl), [v3](../evidence/contextual/v3/summary.jsonl) and [v4](../evidence/contextual/v4/summary.jsonl); each directory also contains `manifest.json`. The following **full** archives were supplied as run artifacts and are **not bundled or publicly downloadable from this repository**. The listed SHA-256 values allow independent verification if the archives are separately provided. Each archive contains per-probe decisions and internal memory data; v3/v4 additionally record exact model-visible payloads.

| Local archive | SHA-256 |
| --- | --- |
| `contextual-grok-001.tar.gz` | `efd4e7bd972491edde161ba446972628e883c9cdfea9420fac02acbcdfe0c01e` |
| `contextual-transfer-grok-pilot-001.tar.gz` | `c61d68c65129628f97b081ed89ab5150282184efc86ac81ea6670c6deaba4c56` |
| `contextual-v3-grok-pilot-001.tar.gz` | `4b8b99afa6a7afed1ca3eca9c3af7ddbff16618451aa9a12935e25a3de792d7b` |
| `contextual-v4-grok-pilot-001.tar.gz` | `2591fbb3595c9cec2cf8048eb6a30b077429c8be7ae554f0a8b49eba20c97079` |

v4 relevant record names: `manifest.json`, `summary.jsonl`, `choices.jsonl`, `organ.jsonl`, `payloads.jsonl`, `snapshots/`. Source protocol: [CONTEXTUAL_LONGLIFE_V4.md](CONTEXTUAL_LONGLIFE_V4.md); precursor protocol: [CONTEXTUAL_IDENTITY_PROTOCOL.md](CONTEXTUAL_IDENTITY_PROTOCOL.md). The runner is `selmem-contextual-longlife` in `src/bin/`. Use fresh output directories and the exact protocol/commit used in the run; outputs from later versions cannot silently replace this evidence.

## What would count as stronger evidence?

1. **Freeze v4's 12-domain protocol** and run all domains without modifying thresholds or choosing tasks after seeing results. Report per-domain variation and all failed histories.
2. **Strengthen controls:** semantic/vector RAG, task-conditioned LLM and rolling summaries with matched total processing cost and memory budgets, not just frequency summarization and lexical overlap.
3. **Test unseeded, unstructured context interpretation:** require the organ to discover contextual facets from natural language, then validate them against independent source material rather than scripted witness labels.
4. **Run interventions:** remove or swap qualified children without otherwise changing the memory state, and test genuinely held-out tasks and new actor models.
5. **Track harms:** confabulation, overgeneralization, stale context, rigidity, forgetting useful minority cases, source-confusion, and the effects of imperfect, lossy reconstruction.

**Claim boundary:** The v4 pilot supports an observed advantage for *context-conditional memory selection* over the **specific deterministic** baselines tested under synthetic long-history distraction. It does not establish general memory superiority, subjective preference emergence, stable individuality, or creative advantage.
