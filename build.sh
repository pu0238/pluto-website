#!/usr/bin/env bash

set -e
CANISTERS="$1"

function generate_did() {
  local canister=$1
  canister_root="src/$canister"

  npm run build

  cargo build --manifest-path="$canister_root/Cargo.toml" \
      --target wasm32-unknown-unknown \
      --release --package "$canister"
}

# The list of canisters of your project
# Those shoudl use ic_cdk >= v0.11.0
#

for canister in $(echo $CANISTERS | sed "s/,/ /g")
do
    generate_did "$canister"
done