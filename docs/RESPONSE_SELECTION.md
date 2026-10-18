# Memory-guided response selection

After recall freezes a `MouthDraft`, the HTTP narrator can generate three candidates, evaluate them and let Rust choose an answer. Candidate text and temporary orientations do not enter the memory store. Only the selected reply follows the normal `close_mouth` path. A failed selection cancels that mouth turn; already completed recall writes remain.

## Defaults and routing

```ini
memory_context=structured
memory_use=experience
response_selection=auto
memory_influence=moderate
candidate_words=180
```

Auto routes the whole-word triggers design, propose, brainstorm, invent and imagine without a routing model call. It is an English heuristic, not a semantic intent classifier. `always` explicitly routes all requests; `direct` keeps one final generation. Legacy-format delivery stays direct. Settings follow the existing file/environment/CLI precedence and require restarting rebuilt processes.

The selector receives only the frozen allowed memory packet and working conversation. It cannot open the store or sealed archive and does not change intake, retrieval thresholds, axiom learning or archive policy.

## Separate generation, quality and grounding

1. Review each delivered episode/disposition for use, irrelevance, uncertainty or bounded capacity. Generate at most three tentative orientations with known source IDs, exact supporting excerpts, a concrete consequence and an application limit. Generate three complete answers. Invented analogies are not labelled as learned co-recall associations.
2. Evaluate utility, feasibility, originality and task violations in a separate task-only call. Anonymous answer order is permuted and checked. Memory, candidate kinds, original IDs, orientations and source declarations are withheld.
3. When actual episodes/dispositions are available, perform a separate grounding review of anonymous standalone answers against those sources. The reviewer does not see original candidate labels or declared source references. Nonzero alignment requires an exact contiguous quote from that same anonymous answer.
4. Rust validates source IDs, shapes, bounds, evidence and exclusions, then combines the fixed quality/grounding results. A memory coefficient changes selection preference; it cannot turn low-quality content into evidence of creativity.

External observations remain external. Empty or irrelevant memory can legitimately yield no orientation. The named agent context is supplied context, not an acquired disposition. Historical source checks and model grounding are fallible.

## Validation, retries and audit

The current decoder validates empty axes, scalar axis values and each answer's own exact quote. Invalid semantic/evidence content is not repaired into a positive result. Supported transport-shape normalizations retain raw and effective values. Feedback identifies the offending field. A failed bounded stage pauses collection and preserves accounting.

The collector counts every HTTP start, caches accepted stages by exact inputs and never silently resets the budget. Resumption requires matching inputs, settings and implementation. Legacy upgrade switches have been removed; recover older interrupted lots with their original source revision. Existing result files remain unchanged. See [reproduction policy](../experiments/REPRODUCIBILITY.md).

Audits retain candidates, orientations/source reviews, anonymous mappings, quality and grounding results, selected ID and weight sweeps. They are diagnostic evidence, not independent ratings. Compare zero and moderate weights on the same candidate pool to isolate selection from generation.

## Recorded scope

The [memory-influence protocol](../experiments/genesis/MEMORY_INFLUENCE.md) provides controlled lives, six tasks, direct and no-memory controls, independent rating templates and fixed-pool reevaluation. Published 005 reused earlier generation and quality work. Its small number of weight-driven switches and mixed quality outcomes do not establish a creativity gain.

Creative Preview 003 deliberately uses matched direct generation in all five conditions. It does not exercise this candidate selector. Its guarded readout also excludes the profile/axiom delivery used by the general organ interface.
