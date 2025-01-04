use std::{
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

use super::headers::Headers;

#[derive(Debug, Eq, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Headers,
}

#[derive(Debug)]
pub enum RequestParsingError {
    NoRequestLine,
    InvalidRequestLine,
    InvalidMethod,
}

impl TryFrom<&mut dyn Read> for Request {
    type Error = RequestParsingError;

    fn try_from(value: &mut dyn Read) -> Result<Self, RequestParsingError> {
        let buf_reader = BufReader::new(value);
        let mut lines = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty());

        let RequestLine(method, path) = lines
            .next()
            .ok_or(RequestParsingError::NoRequestLine)?
            .parse()?;

        Ok(Request {
            method,
            path,
            headers: lines.collect(),
        })
    }
}

struct RequestLine(Method, String);

impl FromStr for RequestLine {
    type Err = RequestParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut request_line_components = s.split_whitespace();
        let method = request_line_components
            .next()
            .map(Method::from_str)
            .ok_or(RequestParsingError::InvalidRequestLine)?
            .map_err(|_| RequestParsingError::InvalidMethod)?;
        let path = request_line_components
            .next()
            .ok_or(RequestParsingError::InvalidRequestLine)?
            .to_string();

        Ok(Self(method, path))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Method {
    GET,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn can_parse_get_request() {
        let mut message = "GET /test.htm HTTP/1.1
User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
Host: www.vwong.dev
Accept-Language: en-uk
Accept-Encoding: gzip, deflate
Connection: Keep-Alive"
            .as_bytes();

        assert_eq!(
            Request::try_from(&mut message as &mut dyn Read).unwrap(),
            Request {
                method: Method::GET,
                path: "/test.htm".into(),
                headers: HashMap::from([
                    (
                        "User-Agent".into(),
                        "Mozilla/4.0 (compatible; MSIE5.01; Windows NT)".into()
                    ),
                    ("Host".into(), "www.vwong.dev".into()),
                    ("Accept-Language".into(), "en-uk".into()),
                    ("Accept-Encoding".into(), "gzip, deflate".into()),
                    ("Connection".into(), "Keep-Alive".into())
                ])
                .into()
            }
        )
    }
}
