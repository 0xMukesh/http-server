use crate::utils::{ContentType, HttpResponse, Status};
use std::{env, fs, net::TcpStream};

pub fn index(stream: &mut TcpStream) {
    let response = HttpResponse {
        status: Status::Ok,
        content: String::new(),
        content_length: 0,
        content_type: ContentType::Plain,
    }
    .make();
    HttpResponse::write_stream_response(stream, response)
}

pub fn not_found(stream: &mut TcpStream) {
    let response = HttpResponse {
        status: Status::NotFound,
        content: String::new(),
        content_length: 0,
        content_type: ContentType::Plain,
    }
    .make();
    HttpResponse::write_stream_response(stream, response)
}

pub fn respond(stream: &mut TcpStream, query: &str) {
    let response = HttpResponse {
        status: Status::Ok,
        content: String::new(),
        content_length: query.len(),
        content_type: ContentType::Plain,
    }
    .make();
    HttpResponse::write_stream_response(stream, response);
}

pub fn return_file(stream: &mut TcpStream, file_name: &str) {
    // ex: ./your_server.sh --directory /tmp/
    // path = "/tmp/"
    let mut env_args: Vec<String> = env::args().collect();
    let path = &mut env_args[2];
    path.push_str(file_name);
    let file = fs::read(path);

    match file {
        Ok(content) => {
            let value = match String::from_utf8(content) {
                Ok(value) => value,
                Err(_) => "".to_string(),
            };
            let length = value.len();

            let response = HttpResponse {
                status: Status::Ok,
                content: value,
                content_length: length,
                content_type: ContentType::OctetStream,
            }
            .make();
            HttpResponse::write_stream_response(stream, response)
        }
        Err(_) => {
            let response = HttpResponse {
                status: Status::NotFound,
                content: "".to_string(),
                content_length: 0,
                content_type: ContentType::OctetStream,
            }
            .make();
            HttpResponse::write_stream_response(stream, response)
        }
    }
}

pub fn create_file(stream: &mut TcpStream, file_name: &str, data: &str) {
    let mut env_args: Vec<String> = env::args().collect();
    let path = &mut env_args[2];
    path.push_str(file_name);

    match fs::write(path, data) {
        Ok(_) => {
            let response = HttpResponse {
                status: Status::Created,
                content: "".to_string(),
                content_length: 0,
                content_type: ContentType::OctetStream,
            }
            .make();
            HttpResponse::write_stream_response(stream, response)
        }
        Err(_) => {
            let response = HttpResponse {
                status: Status::BadRequest,
                content: "".to_string(),
                content_length: 0,
                content_type: ContentType::OctetStream,
            }
            .make();
            HttpResponse::write_stream_response(stream, response)
        }
    };
}
