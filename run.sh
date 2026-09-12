#!/bin/sh
# Portable cargo wrapper. On Linux, invent a libsqlite3.so stub if only
# libsqlite3.so.0 is installed. On macOS the SDK already exports sqlite3.
set -e
ROOT="$(cd "$(dirname "$0")" && pwd)"

os="$(uname -s 2>/dev/null || echo unknown)"

sqlite_ok() {
  echo 'int main(void){return 0;}' | cc -x c - -lsqlite3 -o /tmp/selmem-sqlitetest >/dev/null 2>&1
}

case "$os" in
  Darwin)
    if ! sqlite_ok; then
      for d in \
        "$(brew --prefix sqlite 2>/dev/null)/lib" \
        /opt/homebrew/opt/sqlite/lib \
        /usr/local/opt/sqlite/lib \
        /usr/lib
      do
        [ -n "$d" ] || continue
        if [ -e "$d/libsqlite3.dylib" ] || [ -e "$d/libsqlite3.tbd" ]; then
          export RUSTFLAGS="-L $d ${RUSTFLAGS:-}"
          break
        fi
      done
    fi
    ;;
  Linux)
    if ! ldconfig -p 2>/dev/null | grep -q 'libsqlite3\.so '; then
      mkdir -p /tmp/selmem-libs
      for so in \
        /usr/lib/x86_64-linux-gnu/libsqlite3.so.0 \
        /usr/lib/aarch64-linux-gnu/libsqlite3.so.0 \
        /usr/lib64/libsqlite3.so.0 \
        /lib/x86_64-linux-gnu/libsqlite3.so.0
      do
        if [ -e "$so" ]; then
          ln -sfn "$so" /tmp/selmem-libs/libsqlite3.so
          export RUSTFLAGS="-L /tmp/selmem-libs ${RUSTFLAGS:-}"
          break
        fi
      done
    fi
    ;;
esac

cd "$ROOT"
exec cargo "$@"
