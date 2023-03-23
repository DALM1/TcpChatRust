use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => return,
            Ok(bytes_read) => bytes_read,
            Err(_) => {
                eprintln!("An error occurred while reading from stream");
                return;
            }
        };

        let message = match String::from_utf8(Vec::from(&buffer[..bytes_read])) {
            Ok(message) => message,
            Err(_) => {
                eprintln!("Received invalid UTF-8 from stream");
                return;
            }
        };

        println!("Received message: {}", message.trim());

        match stream.write(message.as_bytes()) {
            Ok(_) => {},
            Err(_) => {
                eprintln!("An error occurred while writing to stream");
                return;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connexion acceptée de {}", stream.peer_addr().unwrap());

                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                eprintln!("Une erreur s'est produite lors de la réception d'une connexion : {}", e);
            }
        }
    }
}
