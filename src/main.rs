mod request;
mod response;
mod router;

use std::{collections::HashMap, fs, net::TcpListener};

use request::Method;
use response::Response;
use router::Router;

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();
    Router::new(listener)
        .add_route((Method::GET, "/".to_string()), |_| Response {
            status: response::Status::Ok,
            headers: HashMap::new(),
            body: Some(fs::read_to_string("assets/hello.html").unwrap()),
        })
        .add_route((Method::GET, "/other".to_string()), |_| Response {
            status: response::Status::NotFound,
            headers: HashMap::new(),
            body: Some(fs::read_to_string("assets/404.html").unwrap()),
        })
        .run();
}
