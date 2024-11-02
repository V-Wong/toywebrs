mod request;

use std::{
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
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("assets/hello.html").unwrap();
        let length = contents.len();

        stream
            .write_all(
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}").as_bytes(),
            )
            .unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("assets/404.html").unwrap();
        let length = contents.len();

        stream
            .write_all(
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}").as_bytes(),
            )
            .unwrap();
    }
}
