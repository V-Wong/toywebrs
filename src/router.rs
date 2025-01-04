use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
};

use crate::{
    http::request::{self, Method},
    http::response,
    thread_pool::ThreadPool,
};

type Handler = fn(&request::Request) -> response::Response;

pub struct Router {
    listener: TcpListener,
    thread_pool: ThreadPool,
    routes: HashMap<(Method, String), Handler>,
}

impl Router {
    pub fn new(listener: TcpListener, thread_pool: ThreadPool) -> Self {
        Self {
            listener,
            thread_pool,
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, method: Method, path: &str, handler: Handler) -> &mut Self {
        self.routes.insert((method, path.into()), handler);
        self
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let request = request::Request::try_from(&mut stream as &mut dyn Read).unwrap();
            let handler = *self
                .routes
                .get(&(request.method, request.path.clone()))
                .unwrap();

            self.thread_pool.exec(move || {
                let mut response = handler(&request);
                compute_restricted_headers(&mut response);
                stream.write_all(String::from(response).as_bytes()).unwrap();
            });
        }
    }
}

fn compute_restricted_headers(response: &mut response::Response) -> &response::Response {
    response.headers.add(
        "Content-Length",
        &response
            .body
            .as_ref()
            .map(|body| body.len())
            .unwrap_or(0)
            .to_string(),
    );
    response
}
