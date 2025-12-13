#!/usr/bin/env bash

set -euo pipefail
readonly script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"



function step {
  echo " "
  echo " "
  echo "$1  =============================="
  echo " "
}

step "CLEANING"
cargo clean


step "LINTING"
cargo clippy --features std


step "TESTING"
cargo llvm-cov --features std # --text # --html


step "BUILDING DOCS"
cargo test --doc --features std -- --show-output
cargo doc --verbose --features std --no-deps


step "BUILDING RELEASE"
cargo build --release --features std

#step "BUILDING BOOK"
#cd "$script_dir/book" && mdbook  build

exit 0