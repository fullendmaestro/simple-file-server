use std::fmt::Display;
use std::fs;
use std::io;
use std::path::Path;

use super::request::{Version, HttpRequest};

#[derive(Debug)]
pub struct HttpResponse {
    version: Version,
    status: ResponseStatus,
    content_length: usize,
    accept_ranges: AcceptRanges,
    pub response_body: String,
    pub current_path: String,
}

impl HttpResponse {
    pub fn new(request: HttpRequest) -> io::Result<HttpResponse> {
        let version = Version::V2_0;
        let mut status = ResponseStatus::NotFound;
        let mut content_length = 0;
        let mut accept_ranges = AcceptRanges::None;
        let current_path = request.resource.path.clone();
        let mut response_body = String::new();

        let server_root_path = std::env::current_dir()?;
        let resource = request.resource.path.clone();
        let new_path = server_root_path.join(resource);
        if new_path.exists() {
            if new_path.is_file() {
                let content = std::fs::read_to_string(&new_path)?;
                content_length = content.len();
                status = ResponseStatus::Ok;
                accept_ranges = AcceptRanges::Bytes;
                let content = format!(
                    "{} {}\r\n{}\r\nContent-Length: {}\r\n\r\n{}",
                    version,
                    status,
                    accept_ranges,
                    content_length,
                    content
                );
                response_body.push_str(&content);
            } else {
                let four_o_four = "<html>
                <body>
                <h1>404 Not Found</h1>
                </body>
                </html>";
                let content_length = four_o_four.len();
                let content = format!(
                    "{} {}\r\n{}\r\nContent-Length: {}\r\n\r\n{}",
                    version,
                    status,
                    accept_ranges,
                    content_length,
                    four_o_four
                );
                response_body.push_str(&content);
            }
        }
        Ok(HttpResponse {
            version,
            status,
            content_length,
            accept_ranges,
            response_body,
            current_path,
        })
    }
}

#[derive(Debug)]
enum ResponseStatus {
    Ok = 200,
    NotFound = 404,
}

impl Display for ResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ResponseStatus::Ok => "200 OK",
            ResponseStatus::NotFound => "404 Not Found",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
enum AcceptRanges {
    Bytes,
    None,
}

impl Display for AcceptRanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            AcceptRanges::Bytes => "Accept-Ranges: bytes",
            AcceptRanges::None => "Accept-Ranges: none",
        };
        write!(f, "{}", msg)
    }
}
