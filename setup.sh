#!/usr/bin/env bash

set -euo pipefail

cargo binstall --no-confirm \
  cargo-binstall

cargo binstall --no-confirm \
  cargo-cache \
  cargo-llvm-cov \
  cargo-expand

cargo cache --autoclean