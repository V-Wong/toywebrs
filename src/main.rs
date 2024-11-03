mod request;
mod response;

use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use request::Method;

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request = request::Request::try_from(&mut stream as &mut dyn Read).unwrap();

    if request.method == Method::GET && request.path == "/" {
        let contents = fs::read_to_string("assets/hello.html").unwrap();
        let length = contents.len();

        let response = response::Response {
            status: response::Status::Ok,
            headers: HashMap::from([("Content-Length".into(), length.to_string())]),
            body: Some(contents),
        };

        stream
            .write_all(String::from(&response).as_bytes())
            .unwrap();
    } else {
        let contents = fs::read_to_string("assets/404.html").unwrap();
        let length = contents.len();

        let response = response::Response {
            status: response::Status::NotFound,
            headers: HashMap::from([("Content-Length".into(), length.to_string())]),
            body: Some(contents),
        };

        stream
            .write_all(String::from(&response).as_bytes())
            .unwrap();
    }
}
