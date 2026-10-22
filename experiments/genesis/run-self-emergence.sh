#!/usr/bin/env bash
set -euo pipefail
# Use only from the SelMem repository root.
out=${1:?Usage: bash experiments/genesis/run-self-emergence.sh OUT [--mock|--prepare-only|--formation-only|--choices-from FORMATION_DIR]}
mode=${2:-}
[[ -f Cargo.toml ]] || { echo 'Run from SelMem repository root.' >&2; exit 2; }
[[ ! -e "$out" && ! -e "$out.tar.gz" ]] || { echo 'Output or output archive exists; use a new directory.' >&2; exit 2; }
args=(self-emergence --out "$out" --max-http-calls "${GENESIS_SELF_EMERGENCE_HTTP_LIMIT:-512}")
case "$mode" in
  '') [[ $# -eq 1 ]] || exit 2 ;;
  --mock|--prepare-only|--formation-only) [[ $# -eq 2 ]] || exit 2 ;;
  --choices-from) [[ $# -eq 3 && -d "${3:-}" ]] || { echo 'Require existing formation directory.' >&2; exit 2; }; args+=(--source "$3") ;;
  *) echo "Unrecognized mode: $mode" >&2; exit 2 ;;
esac
if [[ "$mode" == --mock ]]; then
  args+=(--mock)
else
  config=${SELMEM_CONFIG:-.selmem}
  [[ -f "$config" ]] || { echo "Missing runtime config: $config" >&2; exit 2; }
  args+=(--config "$config" --llm "${GENESIS_SELF_PROVIDER:-grok}" --model "${GENESIS_SELF_MODEL:-grok-4.3}")
  if [[ "$mode" == --prepare-only || "$mode" == --formation-only ]]; then args+=("$mode"); fi
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
