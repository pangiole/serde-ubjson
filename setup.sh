#!/usr/bin/env bash

set -euo pipefail
#script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"; readonly script_dir


cargo binstall --no-confirm \
  cargo-binstall

cargo binstall --no-confirm \
  cargo-cache \
  cargo-llvm-cov \
  cargo-expand \
  mdbook

cargo cache --autoclean