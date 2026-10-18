#!/usr/bin/env bash
set -euo pipefail
fixture=${1:?Usage: bash experiments/genesis/run-creative-preview-guarded.sh FIXTURE OUT_DIR [--resume|--prepare-only]}
out=${2:?Output directory required}
extra=()
case ${3:-} in '') ;; --resume|--prepare-only) extra=("$3");; *) printf 'Unknown third argument\n' >&2; exit 2;; esac
[[ $# -le 3 ]] || { printf 'Too many arguments\n' >&2; exit 2; }
config=${SELMEM_CONFIG:-.selmem}
[[ -f Cargo.toml && -f src/bin/selmem-genesis.rs && -f "$fixture" && -f "$config" ]] || { printf 'Run from repository root with runtime config and preview fixture.\n' >&2; exit 2; }
export SELMEM_TEMP=0
export SELMEM_REASONING=${GENESIS_PREVIEW_REASONING:-none}
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=${CARGO_PROFILE_RELEASE_CODEGEN_UNITS:-1}
args=(creative-preview --guarded --fixture "$fixture" --out "$out" --config "$config" --llm "${GENESIS_PREVIEW_PROVIDER:-grok}" --model "${GENESIS_PREVIEW_MODEL:-grok-4.3}" --max-http-calls "${GENESIS_PREVIEW_HTTP_LIMIT:-1500}")
printf 'GENESIS guarded preview: 788 fresh source turns; 20 nights; 60 matched direct proposals.\nTask-selected lived gists; task-blind retelling and provenance guard; no response-choice selector.\nHTTP ceiling includes all retries and memory calls; it is not a monetary cap or completion guarantee.\n'
mkdir -p "$(dirname "$out")"
set +e
cargo run --release --bin selmem-genesis -- "${args[@]}" "${extra[@]}" 2>&1 | tee -a "$out.log"
status=${PIPESTATUS[0]}
set -e
if [[ -d "$out" ]]; then
  tar -czf "$out.tar.gz" -C "$(dirname "$out")" "$(basename "$out")"
  printf 'SAVED: %s.tar.gz\n' "$out"
fi
exit "$status"
