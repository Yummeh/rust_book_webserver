use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use fs::read_to_string;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    println!("{}", String::from_utf8_lossy(&buffer));

    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "response_page/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "response_page/404.html")
    };
    let contents = fs::read_to_string(file_name).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
