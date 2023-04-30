# Rust HTTP Server - Udemy Course - Learn Rust by Building Real Applications


This is a toy HTTP server written in Rust during the course [Learn Rust by Building Real Applications by Lyubomir Gavadinov](https://www.udemy.com/course/rust-fundamentals/). DO NOT USE THIS FOR ANYTHING RELATED TO PRODUCTION.

You can run the server by running `cargo run` and opening the URL http://127.0.0.1:8080 in your browser.

* This server implements the HTTP/1.1 protocol and only supports the following status codes: 200, 400, 404
* It only supports the GET method (see [`./src/web_handler.rs`](./src/web_handler.rs))


## Setup

Install rust using `rustup` using instructions at: https://www.rust-lang.org/tools/install

Setup VSCode using the plugins:

* [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
* [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) - For GDB on Mac


## Development

Code formatting via `cargo fmt`

Expand macros via `cargo expand` (first install via `cargo add cargo-expand`)