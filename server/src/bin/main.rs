use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use server::ThreadPool;


fn main() {
    let listener = 
        TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);


    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // read the request from the stream and store it in the buffer
    stream.read(&mut buffer).unwrap();
    // convert the buffer to a string

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // check if the request is a GET request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
        // if the request is a GET request, return the status line and the filename
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
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
