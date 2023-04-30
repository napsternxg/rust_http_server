use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        Response::new(
            StatusCode::Ok,
            Some("<h1>Rust HTTP Server</h1>".to_string()),
        )
    }
    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        println!("Failed to parse request: {}", err);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Server running at: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            let res = listener.accept();
            match res {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Stream buffer: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(err) => handler.handle_bad_request(&err),
                            };
                            if let Err(err) = response.send(&mut stream) {
                                println!("Failed to parse request: {}", err);
                            }
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Error: {}", e),
                    };
                }
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}
