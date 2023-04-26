use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, thread, str};

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
        let req = str::from_utf8(&buf).unwrap();

        if bytes_read == 0 {
            print!("Client closed the connection");
            break;
        }

        let mut command = String::new();
        let mut argument = String::new();
        let lines = req.lines();
        if lines.clone().nth(0).unwrap().chars().clone().nth(0).unwrap() == '*' {
            command.push_str(lines.clone().nth(2).unwrap());
            argument.push_str(lines.clone().nth(4).unwrap());
        } 

        match command.as_str() {
            "ECHO" => {
                let response = format!("+{}\r\n", argument);
                stream.write(response.as_bytes()).unwrap();

            }
            "PING" => {    
                stream.write("+PONG\r\n".as_bytes()).unwrap();
            }
            _ => {
                stream.write("+UNKNOWN_COMMAND\r\n".as_bytes()).unwrap();
            }
        }
    }

}
