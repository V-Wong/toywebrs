use std::fmt::Display;

use super::common::CRLF;
use super::headers::Headers;

pub struct Response {
    pub status: Status,
    pub headers: Headers,
    pub body: Option<String>,
}

impl From<Response> for String {
    fn from(response: Response) -> Self {
        let request_line = format!("HTTP/1.1 {}", response.status);
        let header_lines = response.headers.to_string();
        format!(
            "{request_line}{CRLF}{header_lines}{CRLF}{}",
            response.body.unwrap_or("".to_string())
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Ok = 200,
    Created = 201,
    NoContent = 204,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Status::Ok => "200 OK",
            Status::Created => "201 Created",
            Status::NoContent => "204 No Content",
            Status::BadRequest => "400 Bad Request",
            Status::Unauthorized => "401 Unauthorized",
            Status::Forbidden => "403 Forbidden",
            Status::NotFound => "404 Not Found",
            Status::MethodNotAllowed => "405 Method Not Allowed",
            Status::InternalServerError => "500 Internal Server Error",
            Status::NotImplemented => "501 Not Implemented",
            Status::BadGateway => "502 Bad Gateway",
            Status::ServiceUnavailable => "503 Service Unavailable",
        })
    }
}
