#!/bin/sh
set -e
ROOT="$(cd "$(dirname "$0")" && pwd)"
if ! ldconfig -p 2>/dev/null | grep -q 'libsqlite3\.so '; then
  mkdir -p /tmp/selmem-libs
  if [ -e /usr/lib/x86_64-linux-gnu/libsqlite3.so.0 ]; then
    ln -sfn /usr/lib/x86_64-linux-gnu/libsqlite3.so.0 /tmp/selmem-libs/libsqlite3.so
    export RUSTFLAGS="-L /tmp/selmem-libs ${RUSTFLAGS:-}"
  fi
fi
cd "$ROOT"
exec cargo "$@"
