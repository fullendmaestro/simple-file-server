use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use infer::Infer;
use mime_guess::MimeGuess;

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
                let file_content = fs::read(&new_path)?;

                // Determine the MIME type
                let mime_type = Infer::new().get_from_path(&new_path)
                    .ok()
                    .flatten()
                    .map(|kind| kind.mime_type())
                    .unwrap_or_else(|| match new_path.extension().and_then(|ext| ext.to_str()) {
                        Some("html") => "text/html",
                        Some("css") => "text/css",
                        Some("js") => "application/javascript",
                        Some("png") => "image/png",
                        Some("jpg") | Some("jpeg") => "image/jpeg",
                        Some("mp4") => "video/mp4",
                        Some("gif") => "image/gif",
                        Some("pdf") => "application/pdf",
                        Some("txt") | Some("gitignore") => "text/plain",
                        Some("rs") => "text/rs",
                        _ => "application/octet-stream", // Default MIME type
                    });

                let response_header = format!(
                    "{} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\nContent-Disposition: inline\r\n\r\n",
                    version,
                    ResponseStatus::Ok,
                    file_content.len(),
                    mime_type
                );

                response_body.push_str(&response_header);
                // Convert file_content to UTF-8 and handle potential errors
                response_body.push_str(&String::from_utf8(file_content)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?);
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

impl std::fmt::Display for ResponseStatus {
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

impl std::fmt::Display for AcceptRanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            AcceptRanges::Bytes => "Accept-Ranges: bytes",
            AcceptRanges::None => "Accept-Ranges: none",
        };
        write!(f, "{}", msg)
    }
}
