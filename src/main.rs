mod constants;
mod controllers;
mod utils;

use std::net::TcpListener;
use std::str;

use crate::utils::{HttpMethod, HttpRequest};

fn main() {
    let listener = TcpListener::bind(constants::IP_ADDRESS).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let data = HttpRequest::read_stream_request(&mut stream).unwrap();
                let data = HttpRequest::parse(data);

                let method = data.method;
                let path = data.path;
                let headers = data.headers;
                let body = data.body;

                match method {
                    HttpMethod::GET => {
                        if path == "/" || path == "/index.html" {
                            controllers::index(&mut stream);
                        } else if path.starts_with("/echo") {
                            // ex: /echo/abc
                            // query = ["", "echo", "abc"]
                            //    ^^ = [empty, path, query]
                            let query: Vec<&str> = path.split("/").collect();
                            let query = query[2];

                            controllers::respond(&mut stream, query);
                        } else if path == "/user-agent" {
                            let user_agent = match headers.get("User-Agent") {
                                Some(value) => value,
                                None => "",
                            };

                            controllers::respond(&mut stream, user_agent);
                        } else if path.starts_with("/files") {
                            // ex: /files/foo
                            // file_name = ["", "files", "foo"]
                            let file_name: Vec<&str> = path.split("/").collect();
                            let file_name = file_name[2];

                            controllers::return_file(&mut stream, file_name)
                        } else {
                            controllers::not_found(&mut stream)
                        }
                    }
                    HttpMethod::POST => {
                        if path.starts_with("/files") {
                            let file_name: Vec<&str> = path.split("/").collect();
                            let file_name = file_name[2];

                            controllers::create_file(&mut stream, file_name, body.as_str())
                        } else {
                            controllers::not_found(&mut stream)
                        }
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
