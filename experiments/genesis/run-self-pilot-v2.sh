#!/usr/bin/env bash
set -euo pipefail
out=${1:?Usage: bash experiments/genesis/run-self-pilot-v2.sh OUT_DIR [--mock|--prepare-only]}
mode=${2:-}
[[ $# -le 2 && -f Cargo.toml ]] || { printf 'Run from the repository root; at most two arguments.\n' >&2; exit 2; }
case "$mode" in ''|--mock|--prepare-only) ;; *) printf 'Unknown mode: %s\n' "$mode" >&2; exit 2;; esac
args=(self-pilot-v2 --out "$out" --max-http-calls "${GENESIS_SELF_HTTP_LIMIT:-256}")
if [[ "$mode" == --mock ]]; then
  args+=(--mock)
else
  config=${SELMEM_CONFIG:-.selmem}
  [[ -f "$config" ]] || { printf 'Missing runtime config: %s\n' "$config" >&2; exit 2; }
  args+=(--config "$config" --llm "${GENESIS_SELF_PROVIDER:-gpt}" --model "${GENESIS_SELF_MODEL:?Set GENESIS_SELF_MODEL to the model you want to test}")
  [[ "$mode" != --prepare-only ]] || args+=(--prepare-only)
fi
export SELMEM_TEMP=0
export SELMEM_REASONING=${GENESIS_SELF_REASONING:-none}
mkdir -p "$(dirname "$out")"
printf 'GENESIS self pilot 2: two scripted histories, four conditions, 128 matched choices; both option orders for each of two seeds.\nThe self view is audited but excluded from actor prompts. Mock choices are not scientific results.\n'
set +e
cargo run --release --bin selmem-genesis -- "${args[@]}" 2>&1 | tee -a "$out.log"
status=${PIPESTATUS[0]}
set -e
if [[ -d "$out" ]]; then tar -czf "$out.tar.gz" -C "$(dirname "$out")" "$(basename "$out")"; fi
exit "$status"
