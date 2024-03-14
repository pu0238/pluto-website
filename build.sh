#!/usr/bin/env bash

set -e
CANISTERS="$1"

function generate_did() {
  local canister=$1
  canister_root="src/$canister"

  npm run build
  rm -r src/pluto_website_backend/views/*
  cp -r src/pluto_website_frontend/dist/. src/pluto_website_backend/views/
  allHtmlFiles=$(find src/pluto_website_backend/views/ -name "*.html")

  for i in src/pluto_website_backend/views/*.html
  do
    fileName=${i##*/}
    fileNameNoExtencion=${fileName%.html}
    #echo $fileNameNoExtencion

    #echo $i
    #echo $fileName
    #echo ${i##*/}
    pathToFile=${i%/*}
    
    sed -i 's/{/@{/g' $i
    sed -i 's/}/@}/g' $i

    sed -i '1i @()' $i
    mv $i ${pathToFile}/${fileNameNoExtencion}.rs.html

  done

  #cargo build --manifest-path="$canister_root/Cargo.toml" \
  #    --target wasm32-unknown-unknown \
  #    --release --package "$canister"
}

# The list of canisters of your project
# Those shoudl use ic_cdk >= v0.11.0
#

for canister in $(echo $CANISTERS | sed "s/,/ /g")
do
    generate_did "$canister"
done