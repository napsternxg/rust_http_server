use std::fs;

use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;

pub struct WebHandler {
    public_path: String,
}

impl WebHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        // fs::read_to_string(path).ok()
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack: {}", { file_path });
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/test" => Response::new(
                    StatusCode::Ok,
                    Some("<h1>Rust HTTP Server</h1><pre>Test Page</pre>".to_string()),
                ),
                path => match self.read_file(path) {
                    Some(body) => Response::new(StatusCode::Ok, Some(body)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
