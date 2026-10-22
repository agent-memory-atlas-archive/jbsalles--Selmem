# GENESIS: contextual self and behavioral transmission — October 2026

This is an **exploratory, locally recorded result**, not a benchmark score or a
proof of durable individuality, autonomous preferences, consciousness or
creativity. The experiments use scripted simulated histories and Grok 4.3 with
temperature 0; repeated tasks, prompt orders and phases are matched observations
from the same two constructed histories, **not independent lives**. The results
below were checked against the original formation and actor archives. The raw
archives are not included in this documentation patch.

## The causal question

An organ-level self trace and an actor-level behavioral preference are different
objects. We separate (1) forming and revising a self belief from lived episodes,
(2) retrieving it on a new task, (3) explicitly delivering it to the actor, and
(4) the actor actually changing its choice. A missing step must not be reported
as a success of the entire chain.

## Recorded lots

| Diagnostic | Grok 4.3 observations | Main result | Boundary |
| --- | --- | --- | --- |
| Self-Bridge v4 | 84/84 choices; 84 HTTP attempts | 0 matched decision changes between the main memory/self arms; first option chosen on all 72 non-positive-control decisions. Explicit preference control: 12/12. | No independent memory benefit; context activation and option-order bias were bottlenecks. |
| Self-Preference | 24/24 choices; 24 HTTP attempts | Past action: 6/8 history-aligned. Autobiographical preference: 8/8. Direct preference: 8/8. Action → autobiographical: 2/8 favorable switches; autobiographical → direct: 0/8. | Synthetic prompt-only context; **no** SelMem memory in this assay; the autobiographical wording already expresses a preference. |
| Preference Emergence — formation | 70 HTTP attempts; five scripted histories | Active self beliefs formed in both affective histories **and** both action-only controls (1 per history), but not in the neutral control (0). | A belief about repeated action is **not** proof of subjective preference. |
| Preference Emergence — choices | 96/96 choices; 96 HTTP attempts | Episodes → episodes + own self: 1/16 changed. Own self → swapped self: 6/16 changed, all 6 in the **initial** phase (6/8); after contradictions: 0/8. Explicit preference control: 16/16. | Swaps are **explicit actor-context substitutions**; not autonomous unconscious influence. |

Each completed run reports `error: null`. The organ was not modified in the
Preference Emergence intervention. Actor prompts, snapshots and responses are
preserved in the source archives described in the provenance table below.

## Formation, evidence and revision

- The two affect-bearing histories, `consult` and `check`, encode three distinct
  performed choices with consequences and self-reported satisfaction, but do **not**
  state a preference rule. The action-only controls omit affect. The neutral
  control contains no performed decision.
- The stored self claims describe **repeated actions and outcomes**. They do not
  assert a standalone preference such as “I prefer consulting”. Formation in the
  action-only controls demonstrates that the current organ need not use positive
  affect to produce a contextual self belief.
- Both active histories start at a recorded certainty of approximately **0.35**.
  After one independent contrary experience, the initial belief remains active
  at approximately **0.20**. After three, it is retired at approximately **0.05**.
  No new opposite self belief formed in this recorded sequence.
- The retirement thresholds and confidence updates are algorithm rules; the
  source-to-proposition entailment and expectation labels remain fallible model
  judgments. These are **scripted imposed experiences**, not free actor choices.

## Behavioral comparisons

The Preference Emergence decision collection has two active lives, two held-out
but similar tasks, two orders, two actor phases (`initial` and
`after_3_counters`) and six arms: 2 × 2 × 2 × 2 × 6 = **96 choices**.

| Arm | Choices aligned with the **initial** history | First option chosen |
| --- | ---: | ---: |
| Episodes only | 11/16 | 13/16 |
| Episodes + own self | 12/16 | 12/16 |
| Self only | 12/16 | 12/16 |
| `memory_ablation` (invalid control) | 11/16 | 13/16 |
| Episodes + swapped self | 6/16 | 14/16 |
| Explicit direct preference (positive control) | 16/16 | 8/16 |

The swap contrast reports **6/16** changed paired choices in total, but **6/8**
only while the original self beliefs are active and **0/8** after retirement.
These are descriptive, not independent-sample counts. In particular, the swap
can show that Grok responds to **provided self statements**; it does not show
that the organ affects decisions when those statements are not delivered.

**Important control defect:** the 16 `memory_ablation` actor prompts are
identical to the 16 `episodes` prompts in the recorded lot. Its 0/16 differences
therefore convey **no valid memory-ablation inference**. A corrected control must
remove all delivered memory, or carry out a rigorously checked causal lesion
whose effect on the actor context is verified before generation.

The phase comparison should not be called a preference reversal. Following
contrary experiences the self signal disappeared and choices largely reverted
toward option order; no stable opposite belief and no autonomous new decision
policy were demonstrated. The intermediate one-counter snapshot was analyzed
structurally but **not** sent to the actor in this 96-choice plan.

## Provenance and reproduction

Original local archives, as supplied for analysis (SHA-256 of the `.tar.gz`
files; not claims that these files are published in Git):

| Local archive | SHA-256 |
| --- | --- |
| `self-bridge-v4-grok-real-002.tar.gz` | `b3969beb01c4b8354ec1afc85acef31c0b22b54d0ebe5f943d0be96e84665406` |
| `self-preference-grok-real-001.tar.gz` | `a05cbe9727cf0a53e94f62c2c72de6b26844a707b82a8ab0dc1417fdedfc3d17` |
| `self-emergence-formation-grok-001.tar.gz` | `e7808ee259db0accfb195cc0e5125fc6ee0df7fd9f8732741d2b26e79af2f91d` |
| `self-emergence-choices-grok-001.tar.gz` | `2104fb76e93095af202dcb59e85f687282a8ea13a81df93c96a85c1c4586d2cd` |

See [Self-Bridge](SELF_BRIDGE.md), [Self-Preference](SELF_PREFERENCE.md),
[Preference Emergence](SELF_EMERGENCE.md), and the original
[self-model v3 results](SELF_MODEL_RESULTS.md). For a fresh replication use
new output directories and keep the pinned design, source vaults, provider
metadata, code/fixture hashes, raw actor prompts and HTTP responses together.
No archive from these lots was added to Git by this documentation update.

## Next falsification tests

1. Replace the invalid ablation with an actually **memory-free actor prompt**;
   retain the same task and option order for paired comparisons.
2. Follow the trajectory at `initial`, `after_1_counter`, `after_3_counters`,
   and after enough **new opposite actions** to support a new belief. Do not
   label mere retirement as reversal.
3. Compare affect-rich vs action-only sources while coding propositions
   independently for *action pattern*, *positive appraisal* and *preference*.
4. Probe natural contextual activation separately from explicitly supplying
   a self model; use new task families and genuinely independent histories.
5. Replicate on more histories and models before making inferential claims.

The working hypothesis remains that an imperfect, selective and reconstructive
personal history could create path-dependent behavior; this sequence measures
some necessary parts of that chain but has **not demonstrated durable behavioral
individuality or a creative advantage**.
