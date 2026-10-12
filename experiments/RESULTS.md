# Result ledger

These observations come from the supplied run archives. They do not substitute for raw evidence or independent replications. Historical manifests identify the code and model used at the time; a source cleanup does not change those identities.

| Run | Observation | Interpretation |
| --- | --- | --- |
| `gate-001` | Targeted scorer panel 14/14 for both models. Grok retained 1/2 prefix events and both branches; GPT retained all. | Improved targeted checks did not establish scorer generality or behavioral divergence. |
| `policy-001` | New panel: Grok 16/16; GPT-mini 8/16. Grok strict restored missing facts, but histories/policies still yielded identical decisions. | Faithful recall alone did not make the old probe sensitive. GPT scorer accepted omissions and an unsupported cause. |
| `positive-001` | Grok 48/48 oracle choices; GPT-mini 28/48, including 0/16 on reliable history. | Grok qualified for the explicit learning task; GPT-mini did not. |
| `learning-001` | One of twelve reports retained per history. Admitted-only history reproduced the reliable-history failure. | Loss of cumulative information at admission was the dominant diagnosed bottleneck in this assay. |
| `evidence-001` OFF | One retained observation per history; one trace each. Native 8/16 and strict 8/16 matched the full-history oracle. | No positive-history effect. Decisions were reasonable given the reduced evidence. |
| `evidence-001` ON | Twelve observations represented in one trace per history. Native 16/16 and strict 16/16 matched the oracle. | Positive cumulative-learning result carried by explicit support counts. Not proof of reconstructive superiority, individuality or creativity. |

`evidence-001`: 234 OFF + 241 ON = 475 HTTP attempts; 144 decisions; no network errors. Runtime: 18m48s OFF + 20m53s ON. The configurations differ only in observation-support activation. The factual sentence representation was revised for both cuts, so OFF/ON is the appropriate comparison, not ON versus the older `learning-001` alone. Sources and life snapshots were unchanged by probes.

A faithful external baseline passes in both cuts. In ON, Grok follows the reliable actor and chooses verification/alternative for the unreliable actor according to cost. Each final recall context is frozen across eight decision cells. Those cells and repetitions are not independent lives. Fixed OFF-then-ON order, one history pair, and model-generation variation limit causal generalization.

The next informative tests are matched support lesions, mixed/reversing evidence, repeated independent lives and transfer to new tasks/actors. Creativity should be evaluated independently with executable solutions, utility and novelty measures, and equally budgeted conventional-memory/no-memory controls.

## LoCoMo strategy record

The supplied strategy evidence has 179 successful Grok answers out of 180 requested and 180/180 GPT answers; the technical rehearsal contains no real model answers. This is a conversation-derived strategy task, not the full LoCoMo benchmark. Historical source hashes and reported conditions stay with the audit archive. No general benefit in individuality or creativity is claimed.

## Sleep pilot and persistence defect

`sleep-001-measured` completed 240 decisions, 168 readonly readouts and 160 nights, in 2h38m51s with 1555 new HTTP attempts (4712 including failed preparation and recovery). Controlled histories remained 8/8 correct in every arm at cycles 0/1/5/20; support counts stayed 12 per history without duplication. LoCoMo-derived correctness was no-sleep 2/2/1/2, normal 2/1/1/1 and forced-deep 2/1/0/1, each out of twelve. All other factual choices were abstentions. This is a small, admission-limited multiple-choice panel, not the official LoCoMo score or a demonstrated sleep benefit.

Forced deep sleep accumulated 119 and 272 stored, unsuperseded motifs for only 6 and 15 distinct statements. The flat codec omitted axiom stake context, so reopening broke family matching; SQLite additionally omitted trace stake context. The corrected codecs preserve these fields. A local no-HTTP replay on copies of the prepared real prefixes keeps 6 and 15 motifs across twenty reopen/deep-sleep cycles. This verifies the persistence fix, not corrected model performance.

Keep original outputs unchanged. To assess corrected sleep effects, reuse the unchanged `sleep-001-prepared` prefixes and run new isolated interventions into a fresh output directory (for example `sleep-002-measured`); record this patch/code revision with the new result. No paid encoding replay is needed. The earlier LoCoMo admission losses and the latency/cost of proposition scoring are separate issues; this patch does not change the gate, retrieval policy or sleep thresholds to optimize the observed score.
