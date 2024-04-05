use std::net::TcpListener;

// fn handle_client(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();

//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
// }

fn main() {
    let listener = 
        TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established!");
        // handle_client(stream);
    }

}
