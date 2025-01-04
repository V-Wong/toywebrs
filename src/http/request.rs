use std::{
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

use super::{common::CRLF, headers::Headers};

#[derive(Debug, Eq, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Headers,
    pub body: Option<String>,
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
        let mut lines = BufReader::new(value).lines().map(|result| result.unwrap());

        let RequestLine(method, path) = lines
            .next()
            .ok_or(RequestParsingError::NoRequestLine)?
            .parse()?;
        let headers = lines.by_ref().take_while(|line| !line.is_empty()).collect();
        let body = lines.reduce(|acc, line| acc + CRLF + &line);

        Ok(Request {
            method,
            path,
            headers,
            body,
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
            .ok_or(RequestParsingError::InvalidRequestLine)?
            .parse()?;
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
    POST,
}

impl FromStr for Method {
    type Err = RequestParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            _ => Err(RequestParsingError::InvalidMethod),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn can_parse_get_request() {
        let mut message = "\
GET /test.htm HTTP/1.1
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
                .into(),
                body: None
            }
        )
    }

    #[test]
    fn can_parse_post_request() {
        let mut message = "\
POST /test.htm HTTP/1.1
User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
Host: www.vwong.dev
Accept-Language: en-uk
Accept-Encoding: gzip, deflate
Connection: Keep-Alive

FIRST BODY LINE
SECOND BODY LINE"
            .as_bytes();

        assert_eq!(
            Request::try_from(&mut message as &mut dyn Read).unwrap(),
            Request {
                method: Method::POST,
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
                .into(),
                body: Some(format!("FIRST BODY LINE{CRLF}SECOND BODY LINE"))
            }
        )
    }
}
