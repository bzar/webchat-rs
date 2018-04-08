# webchat-client-rs

A doodle to create a mostly-rust client-server web app with a shared data model and binary communication over websocket between a server and WebAssembly client built using wasm-bindgen.

## Instructions

To build and run the server, run `cargo run localhost:8081` inside the `webchat-server-rs` directory.

To build the client do `cargo install wasm-bindgen-cli` and `npm install` inside `webchat-client-rs/web`. To run the client, run `make run` inside the `webchat-client-rs` directory. It will run at `localhost:8080`.
