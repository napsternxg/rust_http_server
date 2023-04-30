#![allow(dead_code)]
mod http;
mod server;
use server::{Handler, Server};
mod web_handler;
use web_handler::WebHandler;

use std::env;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("RUST_UDEMY_PUBLIC_PATH").unwrap_or(default_path);
    println!("Public path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());

    let handler = WebHandler::new(public_path);
    server.run(handler);
}
