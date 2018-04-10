WASM_BINDGEN=~/.cargo/bin/wasm-bindgen
MODE=release

default: run

run: web/webchat_client_rs_bg.wasm
	cd web; npm run serve;

web/webchat_client_rs_bg.wasm: target/wasm32-unknown-unknown/${MODE}/webchat_client_rs.wasm
	${WASM_BINDGEN} target/wasm32-unknown-unknown/${MODE}/webchat_client_rs.wasm --out-dir web

target/wasm32-unknown-unknown/${MODE}/webchat_client_rs.wasm: src/lib.rs
	cargo +nightly build --target wasm32-unknown-unknown --${MODE}
