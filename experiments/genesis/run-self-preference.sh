#!/usr/bin/env bash
set -euo pipefail
out=${1:?Usage: bash experiments/genesis/run-self-preference.sh OUT_DIR [--mock|--prepare-only]}
mode=${2:-}
[[ $# -le 2 && -f Cargo.toml ]] || { echo 'Run from the SelMem root; at most two arguments.' >&2; exit 2; }
case "$mode" in ''|--mock|--prepare-only) ;; *) echo "Unknown mode: $mode" >&2; exit 2;; esac
[[ ! -e "$out" && ! -e "$out.tar.gz" ]] || { echo 'Output or output archive already exists; use a new directory.' >&2; exit 2; }
args=(self-preference --out "$out" --max-http-calls "${GENESIS_SELF_PREFERENCE_HTTP_LIMIT:-36}")
if [[ "$mode" == --mock ]]; then
  args+=(--mock)
else
  config=${SELMEM_CONFIG:-.selmem}
  [[ -f "$config" ]] || { echo "Missing runtime config: $config" >&2; exit 2; }
  args+=(--config "$config" --llm "${GENESIS_SELF_PROVIDER:-grok}" --model "${GENESIS_SELF_MODEL:-grok-4.3}")
  [[ "$mode" != --prepare-only ]] || args+=(--prepare-only)
fi
export SELMEM_TEMP=0
export SELMEM_REASONING=${GENESIS_SELF_REASONING:-none}
mkdir -p "$(dirname "$out")"
set +e
cargo run --release --bin selmem-genesis -- "${args[@]}" 2>&1 | tee "$out.log"
status=${PIPESTATUS[0]}
set -e
if [[ -d "$out" ]]; then tar -czf "$out.tar.gz" -C "$(dirname "$out")" "$(basename "$out")"; fi
exit "$status"
