mod request;
mod response;
mod router;

use std::net::TcpListener;

use router::Router;

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();
    Router::new(listener).run();
}
