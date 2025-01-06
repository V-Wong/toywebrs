# toywebrs
A basic web framework for learning purposes. Built from the ground up with zero dependencies. Heavily inspired by the final project in the Rust book.

## Features
- Simple, extensible API for defining route handlers.
- Basic thread pooling for concurrent request handling.

## Usage
```rs
use std::{fs, net::TcpListener};
use toywebrs::prelude::*;

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();
    Router::new(listener, ThreadPool::new(5))
        .add_route(Method::GET, "/", |_| Response {
            status: Status::Ok,
            headers: Headers::new(),
            body: Some(fs::read_to_string("assets/hello.html").unwrap()),
        })
        .add_route(Method::GET, "/other", |_| Response {
            status: Status::NotFound,
            headers: Headers::new(),
            body: Some(fs::read_to_string("assets/404.html").unwrap()),
        })
        .add_route(Method::POST, "/ping", |request| Response {
            status: Status::Ok,
            headers: Headers::new(),
            body: request
                .body
                .clone()
                .map(|body| format!("Your body: {body}"))
                .or(Some("You didn't provide a body".to_owned())),
        })
        .run();
}
```