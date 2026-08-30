#!/bin/sh
set -e

bin="$1"
shift

if [ "$(basename "$bin")" = "studio" ]; then
  app="$(dirname "$bin")/Ciart Studio.app/Contents/MacOS/Ciart Studio"
  if [ -x "$app" ]; then
    exec "$app" "$@"
  fi
fi

exec "$bin" "$@"
