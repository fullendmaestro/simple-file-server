use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use mime_guess::mime; // Import for content type detection

use super::request::Version;
use super::request::HttpRequest;

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
        let new_path = server_root_path.join(&resource);

        if new_path.exists() {
            if new_path.is_file() {
                let mut file = File::open(&new_path)?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;

                content_length = buffer.len();
                status = ResponseStatus::Ok;
                accept_ranges = AcceptRanges::Bytes;

                let content_type = mime_guess::from_path(&new_path).first_or_octet_stream();
                let headers = format!(
                    "Content-Type: {}\r\nContent-Length: {}\r\n{}\r\n",
                    content_type,
                    content_length,
                    accept_ranges
                );

                response_body.push_str(&headers);
                response_body.push_str(&String::from_utf8_lossy(&buffer));
            } else if new_path.is_dir() {
                status = ResponseStatus::Ok;
                accept_ranges = AcceptRanges::None;
                let mut links = String::new();

                for entry in std::fs::read_dir(new_path)? {
                    let entry = entry?;
                    let path = entry.path();
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    let link = if path.is_dir() {
                        format!("<li><a href=\"{}/\">{}/</a></li>", name, name)
                    } else {
                        format!("<li><a href=\"{}\">{}</a></li>", name, name)
                    };
                    links.push_str(&link);
                }

                let content = format!(
                    "<!DOCTYPE html><html><body><h1>{}</h1><a href=\"../\">Go Back</a><ul>{}</ul></body></html>",
                    current_path,
                    links
                );

                content_length = content.len();
                let headers = format!(
                    "{} {}\r\n{}\r\nContent-Length: {}\r\n\r\n",
                    version,
                    status,
                    accept_ranges,
                    content_length
                );
                response_body.push_str(&headers);
                response_body.push_str(&content);
            } else {
                status = ResponseStatus::NotFound;
                let four_o_four = "<html><body><h1>404 Not Found</h1></body></html>";
                content_length = four_o_four.len();
                let headers = format!(
                    "{} {}\r\n{}\r\nContent-Length: {}\r\n\r\n",
                    version,
                    status,
                    accept_ranges,
                    content_length
                );
                response_body.push_str(&headers);
                response_body.push_str(four_o_four);
            }
        } else {
            status = ResponseStatus::NotFound;
            let four_o_four = "<html><body><h1>404 Not Found</h1></body></html>";
            content_length = four_o_four.len();
            let headers = format!(
                "{} {}\r\n{}\r\nContent-Length: {}\r\n\r\n",
                version,
                status,
                accept_ranges,
                content_length
            );
            response_body.push_str(&headers);
            response_body.push_str(four_o_four);
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