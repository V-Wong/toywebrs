use std::{collections::HashMap, fs, net::TcpListener};

use toywebrs::prelude::*;

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();
    Router::new(listener, ThreadPool::new(5))
        .add_route(Method::GET, "/", |_| Response {
            status: Status::Ok,
            headers: HashMap::new(),
            body: Some(fs::read_to_string("assets/hello.html").unwrap()),
        })
        .add_route(Method::GET, "/other", |_| Response {
            status: Status::NotFound,
            headers: HashMap::new(),
            body: Some(fs::read_to_string("assets/404.html").unwrap()),
        })
        .run();
}
