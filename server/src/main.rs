use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = 
        TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // read the request from the stream and store it in the buffer
    stream.read(&mut buffer).unwrap();
    // convert the buffer to a string

    let get = b"GET / HTTP/1.1\r\n";
    // check if the request is a GET request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
        // if the request is a GET request, return the status line and the filename
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
        // if the request is not a GET request, return the status line and the filename
    };

    let contents = fs::read_to_string(filename).unwrap();
    // read the contents of the file and store it in the contents variable

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    // combine the status line and the contents of the file

    stream.write(response.as_bytes()).unwrap();
    // write the response to the stream
    stream.flush().unwrap();
    // flush the stream
}
