use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(&stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: &TcpStream) {
    println!("accepted new connection");
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            print!("Client closed the connection");
            break;
        }
        stream.write("+PONG\r\n".as_bytes()).unwrap();
    }

}
