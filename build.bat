wasm-pack build .\frontend --target web --out-name wasm --out-dir static

echo F | xcopy /Y .\frontend\static\wasm.js .\static\wasm.js
echo F | xcopy /Y .\frontend\static\wasm_bg.wasm .\static\wasm_bg.wasm
