use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::string::FromUtf8Error;

use crate::constants;

pub enum Status {
    Ok,
    NotFound,
    Created,
    BadRequest,
}

impl Status {
    fn value(&self) -> &str {
        match self {
            Self::Ok => constants::OK,
            Self::NotFound => constants::NOT_FOUND,
            Self::Created => constants::CREATED,
            Self::BadRequest => constants::BAD_REQUEST,
        }
    }
}

pub enum ContentType {
    Plain,
    OctetStream,
}

impl ContentType {
    fn value(&self) -> &str {
        match self {
            Self::Plain => constants::PLAIN_TEXT,
            Self::OctetStream => constants::OCTET_STREAM,
        }
    }
}

pub enum HttpMethod {
    GET,
    POST,
}

impl HttpMethod {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            _ => HttpMethod::GET,
        }
    }
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn parse(request: String) -> Self {
        let parts: Vec<&str> = request.split("\r\n\r\n").collect();

        let head_parts = parts[0];
        let body_parts = parts[1];

        let head_lines: Vec<&str> = head_parts.split("\r\n").collect();
        let request_line: Vec<&str> = head_lines.first().unwrap_or(&"").split(" ").collect();

        let mut headers_map: HashMap<String, String> = HashMap::new();

        for kv_line in head_lines.iter().skip(1) {
            if let Some((k, v)) = kv_line.split_once(": ") {
                headers_map.insert(k.trim().to_string(), v.trim().to_string());
            }
        }

        Self {
            method: HttpMethod::from(request_line[0]),
            path: request_line[1].to_string(),
            headers: headers_map,
            body: body_parts.to_string(),
        }
    }
    pub fn read_stream_request(stream: &mut TcpStream) -> Result<String, FromUtf8Error> {
        let mut buf: Vec<u8> = vec![0; 1024];
        let read_len = stream.read(&mut buf).unwrap();
        buf.truncate(read_len);
        String::from_utf8(buf)
    }
}

pub struct HttpResponse {
    pub status: Status,
    pub content: String,
    pub content_length: usize,
    pub content_type: ContentType,
}

impl HttpResponse {
    pub fn make(&self) -> String {
        let mut response = String::new();
        let status = Status::value(&self.status);

        response.push_str(status);
        response.push_str(constants::CRLF);

        let content_type = ContentType::value(&self.content_type);

        response.push_str(format!("Content-Type: {}", content_type).as_str());
        response.push_str(constants::CRLF);

        if self.content_length != 0 {
            response.push_str(format!("Content-Length: {}", self.content_length).as_str());
            response.push_str(constants::CRLF);
            response.push_str(constants::CRLF);
            response.push_str(self.content.as_str());
        }

        response.push_str(constants::CRLF);

        response
    }
    pub fn write_stream_response(stream: &mut TcpStream, response: String) {
        stream
            .write(format!("{}", response).as_bytes())
            .expect("an error occured");
    }
}
