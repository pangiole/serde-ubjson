#!/usr/bin/env bash

set -euo pipefail
readonly script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

no_std=''
[[ "${1:-''}" == "--no-std" ]] && no_std="--no-default-features --features embedded-io"


function step {
  echo " "
  echo " "
  echo "$1  =============================="
  echo " "
}

step "CLEANING"
cargo clean


step "LINTING"
cargo clippy $no_std


step "TESTING"
cargo llvm-cov $no_std # --text # --html


step "BUILDING DOCS"
cargo test $no_std --doc -- --show-output
cargo doc $no_std --verbose --no-deps


exit 0