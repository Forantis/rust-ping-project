use std::env;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET /ping HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let headers = str::from_utf8(&buffer).unwrap();
        let headers = headers.split("\r\n\r\n").next().unwrap();
        let headers_json = format!(
            "{{\"headers\": \"{}\"}}",
            headers.lines().skip(1).collect::<Vec<&str>>().join("\\n")
        );

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            headers_json.len(),
            headers_json
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write(status_line.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() {
    let port = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "7878".to_string());
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).unwrap();

    println!("Listening on http://{}", address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}