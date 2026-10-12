# Retained experiments

The maintained experiment paths are:

1. [GENESIS](genesis/README.md): qualify a decision model, then test cumulative learning with observation support OFF/ON.
2. [LoCoMo strategy](../docs/LOCOMO_STRATEGY.md): external conversation-derived strategy, design and factual probes.

[RESULTS.md](RESULTS.md) records the known observations and limits. Old narrative demos, P0–P67 grids, witnesses, AMA adapters and GENESIS troubleshooting commands have been retired. Their raw evidence remains in the separately supplied history archive and the original Git history; they are not active commands.

Generated runs do not belong in the source tree. Use fresh paths under `experiments/genesis/results/` or `experiments/strategy/`, preserve their manifests and raw responses, and archive them for analysis. Both locations are ignored by Git. No committed result should be silently overwritten or upgraded to the current code version.
