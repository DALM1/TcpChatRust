use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // Attends que le client envoie le mot de passe
    stream.read(&mut buffer).unwrap();
    let password = String::from_utf8_lossy(&buffer[..]).trim().to_string();

    // Vérifie le mot de passe
    if password != "my_secret_password" {
        stream.write(b"Mot de passe incorrect\n").unwrap();
        return;
    }

    // Demande le nom d'utilisateur
    stream.write(b"Entrez votre nom d'utilisateur: ").unwrap();

    // Attends que le client envoie le nom d'utilisateur
    buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let username = String::from_utf8_lossy(&buffer[..]).trim().to_string();

    // Affiche un message pour indiquer que le client a rejoint le chat
    println!("{} a rejoint le chat", username);

    // Boucle pour recevoir les messages du client
    loop {
        buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(0) => break, // Le client a fermé la connexion
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                println!("{}: {}", username, message);
            }
            Err(e) => {
                eprintln!("Erreur de lecture: {}", e);
                break;
            }
        }
    }

    // Ferme la connexion
    println!("{} a quitté le chat", username);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5000")?;

    // Accepte les connexions en boucle
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connexion acceptée de {}", stream.peer_addr().unwrap());
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Erreur de connexion: {}", e),
        }
    }

    Ok(())
}
