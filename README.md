# webchat-client-rs

A doodle to create a mostly-rust client-server web app with a shared data model and binary communication over websocket between a server and WebAssembly client built using wasm-bindgen.

## Instructions

To build and run the server, run `cargo run localhost:8081` inside the `webchat-server-rs` directory.

The client requires nightly toolchain, wasm32-unknown-unknown target and wasm-bindgen tool: `rustup install nightly && rustup target add wasm32-unknown-unknown --toolchain nightly && cargo install wasm-bindgen-cli`

To build and run the client first run `npm install` inside `webchat-client-rs/web`, then run `make run` inside the `webchat-client-rs` directory. It will run at `localhost:8080`.

## Known issues

Does not currently work in chromium. Explained in [wasm-bindgen's chrome caveat](https://github.com/rustwasm/wasm-bindgen/blob/master/examples/hello_world/README.md#caveat-for-chrome-users).
