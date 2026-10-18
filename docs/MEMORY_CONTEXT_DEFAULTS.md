# Memory context defaults and controlled rerun

Structured memory delivery and the experience-use contract are now the default
for new/opened organs and empty runtime configuration:

```ini
memory_context=structured
memory_use=experience
```

No configuration edit is needed when these keys are absent. Explicit settings
and `SELMEM_MEMORY_CONTEXT` / `SELMEM_MEMORY_USE` environment values still take
precedence. Remove old `legacy` overrides if you want the new defaults. Restart
rebuilt chat/daemon binaries to adopt them.

## Organ interface

The HTTP mouth receives already selected lived episodes with attribution,
channel, fidelity, confidence and affect; existing allowed living axioms with
layer, strength and provenance; existing co-recall edges among delivered
scenes; and current organ affect. Missing eligible content stays empty.
External observations are never promoted into the agent's autobiography.
Motifs remain motifs. The adapter never opens the sealed archive.

The English contract invites relevant use of fallible experience. It does not
prescribe creativity, an emotion, a personality or an answer. Its text and the
shared data guide are in `data/memory-context-*.txt`.

Runtime cuts remain independent:

```ini
memory_context_episodes=true
memory_context_dispositions=true
memory_context_associations=true
memory_context_affect=true
memory_context_words=600
```

The structured packet has at most four already supplied episodes, four combined
dispositions/context statements, six selected pairwise edges, and 32 displayed
support IDs per disposition with an explicit full count. Narrative and statement
text shares the word budget (32–4000); per-item limits are 160 and 80 words.
JSON metadata is additional input, so this is not a token or monetary ceiling.
Truncation is flagged. Text within the budget retains its existing wording.
Disabling episodes also removes association endpoints. The existing association
reconstruction switch remains required. Affect cuts remove both episode and
current-state affect. Nonfinite numeric metadata renders as JSON null.

Options are not saved into a vault. Encoding, retrieval and sleep rules are not
changed. Both normal speak and the daemon use the same packet frozen by
`open_mouth`. The daemon still releases its mutex before HTTP.

Compatible HTTP backends use the structured protocol. Existing custom/rule
narrators report no protocol capability and retain their historical reply path
when all components are enabled. When a component is cut they receive filtered
values. Do not count an unsupported backend or a rule fallback as a model
experiment with the new contract.

To audit the prepared packet without an extra reply call:

```rust
# fn inspect(organ: &mut selmem::SelectiveMemory) {
let draft = organ.open_mouth("A current question");
let narrator = organ.narrator_arc();
println!("{}", draft.delivery_audit_json(narrator.as_ref()));
println!("{}", draft.memory_reply().system_prompt());
println!("{}", draft.memory_reply().user_prompt());
organ.cancel_mouth(&draft);
# }
```

Normal recall can still perform its existing model calls/write-backs. The
adapter adds no calls. Capability/audit metadata does not prove model influence.

## Controls and compatibility

`memory_use=neutral` uses the same new base prompt and attribution guide without
the experience-use contract. This is the clean instruction control.
`memory_context=legacy` plus `memory_use=legacy` explicitly restores historical
HTTP replies. The historical prompt already asks the model to follow its
reading profile; it is not a neutral control. Component cuts require structured
format. Invalid option values fail explicitly.

These defaults change future native organ replies. Existing result files are
not edited. Older standalone experiment generators with their own final prompt
are not retroactively changed by a library default. In particular, replay-009
bypasses the organ mouth: use the new memory replay runner to evaluate the new
contract, rather than relaunching replay-009 with different config keys.

Current defaults are described here; historical patch installation notes are retained in Git history.
