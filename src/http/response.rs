use std::fmt::Display;

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
            "{request_line}\r\n{header_lines}\r\n{}",
            response.body.unwrap_or("".to_string())
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Ok = 200,
    NotFound = 404,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Status::Ok => "200 OK",
            Status::NotFound => "404 Not found",
        })
    }
}
