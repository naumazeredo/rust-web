#!/usr/bin/env bash

if [[ $1 == "debug" ]]; then
  wasm-pack build ./frontend --target web --out-name wasm --out-dir static
else
  wasm-pack build ./frontend --release --target web --out-name wasm --out-dir static
fi

cp ./frontend/static/wasm.js ./static/wasm.js
cp ./frontend/static/wasm_bg.wasm ./static/wasm_bg.wasm
