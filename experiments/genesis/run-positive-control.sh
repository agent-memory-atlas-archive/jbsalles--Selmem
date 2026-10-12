#!/usr/bin/env bash
set -euo pipefail
out_dir=${1:-experiments/genesis/results/positive-001}
runtime_config=${SELMEM_CONFIG:-.selmem}
export SELMEM_REASONING=off
[[ -f run.sh && -f "$runtime_config" ]] || { printf 'Run from repository root with .selmem configured.\n' >&2; exit 2; }
[[ ! -e "$out_dir" && ! -e "$out_dir.tar.gz" ]] || { printf 'Choose a fresh output directory.\n' >&2; exit 2; }
./run.sh build --release --bin selmem-genesis
mkdir -p "$out_dir"
printf 'POSITIVE CONTROL: 48 decisions per model; 96 planned HTTP calls, maximum 120 attempts with retries. No SelMem memory trials.\n'
for provider in grok gpt; do
  if [[ "$provider" == grok ]]; then model=${GROK_MODEL:-grok-4.3}; else model=${GPT_MODEL:-gpt-4o-mini-2024-07-18}; fi
  ./run.sh run --release --bin selmem-genesis -- positive \
    --config "$runtime_config" --llm "$provider" --model "$model" \
    --out "$out_dir/$provider" --max-http-calls 60 2>&1 | tee "$out_dir/$provider.log"
done
tar -czf "$out_dir.tar.gz" -C "$(dirname "$out_dir")" "$(basename "$out_dir")"
printf 'COMPLETE: %s.tar.gz\nInspect qualified for EACH provider before proceeding to memory trials.\n' "$out_dir"
