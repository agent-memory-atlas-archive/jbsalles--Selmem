# Creativity rerun with default memory delivery

This is a new, separately recorded 72-proposal lot, with the same two histories,
three tasks, six conditions, two requested repeat seeds, model and reasoning
as creative-replay-003. It uses the organ's current structured/experience
contract in final generation. No raw history is re-encoded and no sleep is run.
The previous experiments and results remain available.

## Reuse the completed work

Inputs are the original `creative-preview-002` folder (states, contexts and vault
checksums) and the completed real `creative-replay-003` folder (contexts,
validated response cache and response audit). Keep both folders intact.

The new runner reuses the exact guarded retellings and gist fallbacks from
replay-003, along with its RAG context and accepted no-memory preliminary ideas.
It validates each cache entry against a recorded successful response, and maps
retellings back to selected IDs only if they reproduce the old background
exactly. Previous final proposals are not used as model input. There is no paid
retell/guard/draft step in this rerun.

**72 nominal new HTTP calls** produce 72 new final proposals. The default
200-attempt ceiling includes bounded retries. This is not a monetary cap;
structured metadata adds prompt tokens. The recorded replay-004 completed 72 proposals in 72 attempts. Its compact output is published separately from the original raw archive.

## Launch from the repository root

```bash
# Optional inspection: write all 36 inputs and the exact system prompt, zero calls.
bash experiments/genesis/run-creative-memory-replay.sh collect \
  --source experiments/genesis/results/creative-preview-002 \
  --previous experiments/genesis/results/creative-replay-003 \
  --out experiments/genesis/results/creative-replay-004 \
  --config .selmem --llm grok --model grok-4.3 --prepare-only

# Start the prepared lot.
bash experiments/genesis/run-creative-memory-replay.sh collect \
  --source experiments/genesis/results/creative-preview-002 \
  --previous experiments/genesis/results/creative-replay-003 \
  --out experiments/genesis/results/creative-replay-004 \
  --config .selmem --llm grok --model grok-4.3 --max-http-calls 200 --resume
```

For an immediate run in a fresh output directory, omit `--prepare-only` from the
first command and add `--max-http-calls 200`; do not use `--resume` on a directory
that has not been prepared or started. After interruption, use the second
command. Completed responses are reused. A completed resume adds no calls.
Model, endpoint and reasoning must match the previous lot (grok-4.3, reasoning
none for the supplied inputs). Reasoning is still explicitly sent to the API.

The effective options come from runtime Config, with structured/experience
as the new defaults. The rerun requires structured format. A deliberate
`memory_use=neutral` run is possible in another fresh folder and is explicitly
recorded; do not mistake it for the default experience treatment.

## Treatment and controls

Every arm receives exactly the same final system instruction, obtained from
`MemoryReply::system_prompt`, plus the same JSON/180-word output specification.
Native arms receive selected episode narratives and the available saved affect
and metadata. The no-memory arm receives an empty memory packet. The RAG arm
receives its old background explicitly labelled external retrieved context; the
two-step arm receives its old draft labelled preliminary ideas, not personal
memory. Both keep empty native memory packets. Working conversation is empty
in every arm. Final temperature remains 0.7, ceiling 450 tokens, requested seeds
100/101. Requested seeds are not a guarantee of deterministic provider behavior.

The snapshot inspection adapter is explicitly partial. The saved JSON does
not contain episode confidence, channel or disgust; these fields are null,
never guessed from fidelity or other fields. Top-level mood is available and
parsed from the saved state. Dispositions are restricted to the exact axiom
statements already allowed by the original context audit, and matched to living
state axioms and support provenance. It does not inject all saved motifs into a
query where the original engine allowed none. The supplied run has no such
allowed dispositions and zero graph links. These fields correctly remain empty.
A nonzero graph count without exported edge weights is rejected rather than
replaced with invented links. The adapter never parses vault contents; it only
verifies their bytes against saved checksums and reads inspection JSON.

Thus this evaluates the new delivery treatment over fixed previous readouts,
not a new full life, new retrieval, new sleep or new reconstruction mechanism.
A difference from replay-003 combines the new prompt, structure and available
metadata; it does not isolate any single component. Compare arms within the new
lot, and use a separate neutral lot to isolate the additional experience
instruction. No improvement in creativity is presumed.

## Audit, resume and rating

The protocol includes both input manifests, exact final system prompt, options,
code fingerprint, model and reasoning. All 36 `contexts/*.json` contain the exact
new user input and its source/guard audit. Resume refuses changed inputs,
implementation, prompts or settings. Output must be outside both source trees.
An exclusive lock prevents concurrent writers. Remove a stale lock only after
confirming no process is running. Keep truncated audit logs for diagnosis.

Outputs use fresh `M0001`–`M0072` blind IDs to avoid collision with replay-003.
Use two independent judges with no access to conditions or the private key:

```bash
target/release/selmem-creative-memory-replay template \
  experiments/genesis/results/creative-replay-004 judge-a.json
target/release/selmem-creative-memory-replay template \
  experiments/genesis/results/creative-replay-004 judge-b.json
# Fill the files independently: valid, utility/originality 0–4 and rationale.
# Constraint violations require valid=false and zero scores.
target/release/selmem-creative-memory-replay combine ratings.json judge-a.json judge-b.json
target/release/selmem-creative-memory-replay analyze \
  experiments/genesis/results/creative-replay-004 ratings.json
```

Rating/report destinations must be fresh. Full judge coverage is validated;
actual judge independence cannot be established by software. Treat comparisons
as exploratory, with two histories and shared contexts across repeat pairs.
Mock outputs are explicitly marked and refused as experimental evidence.
