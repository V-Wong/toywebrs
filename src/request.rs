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
        let request_line_components: Vec<&str> =
            request_line.split_whitespace().collect::<Vec<_>>();

        if request_line_components.len() != 3 {
            return Err(RequestParsingError::InvalidRequestLine);
        }

        let method = Method::try_from(*request_line_components.get(0).unwrap())
            .map_err(|_| RequestParsingError::InvalidRequestLine)?;
        let path = request_line_components.get(1).unwrap().to_string();

        let mut headers = HashMap::new();
        for header in lines.into_iter() {
            let header_components = header.split(":").collect::<Vec<_>>();
            let key = *header_components
                .get(0)
                .ok_or(RequestParsingError::InvalidHeader)?;
            let value = *header_components
                .get(1)
                .ok_or(RequestParsingError::InvalidHeader)?;
            headers.insert(key.to_string(), value.to_string());
        }

        Ok(Request {
            method,
            path,
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
