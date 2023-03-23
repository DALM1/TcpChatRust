use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:5000").expect("Failed to connect to server");

    println!("Connected to server!");

    let mut username = String::new();
    let mut buffer = [0; 1024];

    // Prompt user for username
    print!("Enter your username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    // Send username to server
    stream.write(username.as_bytes()).unwrap();

    // Spawn a thread to handle incoming messages
    let handle = thread::spawn(move || {
        loop {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    println!("{}", message);
                }
                Ok(_) | Err(_) => {
                    break;
                }
            }
        }
    });

    // Main loop for sending messages
    loop {
        let mut message = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut message).unwrap();
        let message = message.trim();

        // Quit command
        if message == "/quit" {
            break;
        }

        // Send message to server
        stream.write(message.as_bytes()).unwrap();
    }

    // Wait for the incoming message thread to exit
    handle.join().unwrap();
}
