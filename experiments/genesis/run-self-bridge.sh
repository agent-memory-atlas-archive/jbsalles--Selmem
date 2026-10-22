#!/usr/bin/env bash
set -euo pipefail
source_dir=${1:?Usage: bash experiments/genesis/run-self-bridge.sh PILOT_V3_SOURCE OUT_DIR [--prepare-only|--mock]}
out=${2:?Usage: bash experiments/genesis/run-self-bridge.sh PILOT_V3_SOURCE OUT_DIR [--prepare-only|--mock]}
mode=${3:-}
[[ $# -le 3 && -f Cargo.toml ]] || { echo 'Run from the SelMem repository root; at most three arguments.' >&2; exit 2; }
case "$mode" in ''|--prepare-only|--mock) ;; *) echo "Unknown mode: $mode" >&2; exit 2;; esac
[[ -d "$source_dir" && ! -e "$out" && ! -e "$out.tar.gz" ]] || { echo 'Source missing or output already exists.' >&2; exit 2; }
args=(self-bridge --source "$source_dir" --out "$out" --max-http-calls "${GENESIS_SELF_BRIDGE_HTTP_LIMIT:-100}")
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
