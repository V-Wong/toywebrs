use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
};

use crate::{
    request::{self, Method},
    response,
};

pub struct Router {
    listener: TcpListener,
    routes: HashMap<(Method, String), fn(&request::Request) -> response::Response>,
}

impl Router {
    pub fn new(listener: TcpListener) -> Self {
        Self {
            listener,
            routes: HashMap::new(),
        }
    }

    pub fn add_route(
        &mut self,
        route: (Method, String),
        handler: fn(&request::Request) -> response::Response,
    ) -> &mut Self {
        self.routes.insert(route, handler);
        self
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let request = request::Request::try_from(&mut stream as &mut dyn Read).unwrap();
            let handler = self
                .routes
                .get(&(request.method, request.path.clone()))
                .unwrap();
            let response = handler(&request);
            stream
                .write_all(String::from(&response).as_bytes())
                .unwrap();
        }
    }
}
