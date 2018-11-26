#!/bin/bash

set -eu

# Derived from: https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html
cargo +nightly build --target=wasm32-unknown-unknown --release
mkdir -p gen
wasm-bindgen ../target/wasm32-unknown-unknown/release/toybox_wasm.wasm \
  --out-dir gen
