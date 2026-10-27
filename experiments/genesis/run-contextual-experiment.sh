#!/usr/bin/env bash
set -euo pipefail

# Run from any working directory; keep all experiment execution in Rust.
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

if [[ $# -eq 0 ]]; then
  echo 'Usage: bash experiments/genesis/run-contextual-experiment.sh --out NEW_DIR [--actor --provider grok --model grok-4.3 --max-http-calls 128]' >&2
  exit 2
fi
cargo run --release --bin selmem-contextual-experiment -- "$@"
