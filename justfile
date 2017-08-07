name = "rust-wasm-start"
exported_functions = "'_update'"

build:
	mkdir -p wasm
	rm -rf target/wasm32-unknown-emscripten/release/deps/*.wasm
	rm -rf target/wasm32-unknown-emscripten/release/{{name}}.js
	cargo rustc --release \
	--target=wasm32-unknown-emscripten -- \
	-C link-args="-s EXPORTED_FUNCTIONS=[{{exported_functions}}]" \
	--verbose 
	cp target/wasm32-unknown-emscripten/release/{{name}}.js wasm/{{name}}.js
	cp target/wasm32-unknown-emscripten/release/deps/*.wasm wasm/{{name}}.wasm