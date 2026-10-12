#!/usr/bin/env bash
set -euo pipefail
source_dir=${1:-experiments/genesis/results/positive-001/grok}
out_dir=${2:-experiments/genesis/results/evidence-001}
runtime_config=${SELMEM_CONFIG:-.selmem}
export SELMEM_REASONING=off
[[ -f run.sh && -f "$runtime_config" ]] || { printf 'Run from repository root with .selmem configured.\n' >&2; exit 2; }
[[ -f "$source_dir/positive.json" && -f "$source_dir/positive-design.json" ]] || { printf 'Missing qualified positive control.\n' >&2; exit 2; }
[[ ! -e "$out_dir" && ! -e "$out_dir.tar.gz" ]] || { printf 'Choose a fresh output directory.\n' >&2; exit 2; }
./run.sh build --release --bin selmem-genesis
mkdir -p "$out_dir"
printf 'OBSERVATION EVIDENCE: same Grok model and fixtures, cut OFF then ON. Per condition: 24 live inputs, 4 readonly readouts, 72 decisions. Maximum 1600 HTTP attempts TOTAL including internal calls/retries.\n'
for setting in off on; do
  printf '\nEVIDENCE CUT=%s\n' "$setting"
  ./run.sh run --release --bin selmem-genesis -- learning \
    --source "$source_dir" --out "$out_dir/$setting" --config "$runtime_config" \
    --llm grok --model "${GROK_MODEL:-grok-4.3}" --observation-evidence "$setting" \
    --max-http-calls 800 2>&1 | tee "$out_dir/$setting.log"
done
tar -czf "$out_dir.tar.gz" -C "$(dirname "$out_dir")" "$(basename "$out_dir")"
printf 'COMPLETE: %s.tar.gz\n' "$out_dir"
