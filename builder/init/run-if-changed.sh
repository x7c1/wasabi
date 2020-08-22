#!/usr/bin/env bash

set -xue

script_file=$1
current_hash=$(sha1sum "$script_file")
hash_marker="$script_file".sha1

main() {
  if already_executed; then
    echo "already executed: ${script_file}"
    exit
  fi
  $script_file
  echo "done!"
  echo "$current_hash" > "$script_file".sha1
}

already_executed() {
  if [ ! -e "$hash_marker" ]; then
    return 1
  fi
  cached=$(cat "$hash_marker")
  if [ "$cached" = "$current_hash" ]; then
    return 0
  else
    return 1
  fi
}

main
