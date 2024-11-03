use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug)]
pub enum RequestParsingError {
    NoRequestLine,
    InvalidRequestLine,
    InvalidHeader,
}

impl TryFrom<&mut dyn Read> for Request {
    type Error = RequestParsingError;

    fn try_from(value: &mut dyn Read) -> Result<Self, RequestParsingError> {
        let buf_reader = BufReader::new(value);
        let mut lines = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty());

        let request_line = lines.next().ok_or(RequestParsingError::NoRequestLine)?;
        let mut request_line_components = request_line.split_whitespace();
        let method = request_line_components
            .next()
            .map(Method::try_from)
            .ok_or(RequestParsingError::InvalidRequestLine)?
            .map_err(|_| RequestParsingError::InvalidRequestLine)?;
        let path = request_line_components
            .next()
            .ok_or(RequestParsingError::InvalidRequestLine)?;

        let mut headers = HashMap::new();
        for header in lines.into_iter() {
            let mut header_components = header.split(":");
            let key = header_components
                .next()
                .ok_or(RequestParsingError::InvalidHeader)?;
            let value = header_components
                .next()
                .ok_or(RequestParsingError::InvalidHeader)?;
            headers.insert(key.to_string(), value.to_string());
        }

        Ok(Request {
            method,
            path: path.to_string(),
            headers,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Method {
    GET,
}

impl TryFrom<&str> for Method {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Method::GET),
            _ => Err(()),
        }
    }
}
