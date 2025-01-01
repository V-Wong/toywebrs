use super::headers::Headers;

pub struct Response {
    pub status: Status,
    pub headers: Headers,
    pub body: Option<String>,
}

impl From<Response> for String {
    fn from(response: Response) -> Self {
        let request_line = format!("HTTP/1.1 {}", String::from(&response.status));
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

impl From<&Status> for String {
    fn from(status: &Status) -> Self {
        match status {
            Status::Ok => "200 OK",
            Status::NotFound => "404 Not found",
        }
        .to_string()
    }
}
