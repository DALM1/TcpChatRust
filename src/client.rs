use std::io::{self, BufRead, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut buffer = [0; 1024];

    // Lire la réponse du serveur après la connexion
    stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..]).trim().to_string();
    println!("{}", response);

    print!("Entrez le mot de passe : ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    stream.write(password.trim().as_bytes()).unwrap();
    stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..]).trim().to_string();

    if response == "Mot de passe incorrect" {
        println!("{}", response);
        return;
    }

    print!("Entrez votre nom d'utilisateur : ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    stream.write(username.trim().as_bytes()).unwrap();
    println!("\n");

    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut message = String::new();
        stdin.lock().read_line(&mut message).unwrap();

        stream.write(message.trim().as_bytes()).unwrap();

        if message.trim().to_lowercase() == "/quit" {
            break;
        }

        stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..]).trim().to_string();
        println!("{}", response);
    }
}
