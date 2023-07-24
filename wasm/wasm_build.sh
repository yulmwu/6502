#!/bin/bash

wasm-pack build --target web --out-name wasm --out-dir ../static 
rm -rf ../static/.gitignore
rm -rf ../static/package.json
rm -rf ../static/wasm_bg.wasm.d.ts
rm -rf ../static/wasm.d.ts
