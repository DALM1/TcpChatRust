use std::io::{self, BufRead, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    println!("Le serveur a démarré et écoute sur le port 5000...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Connexion acceptée de {}", stream.peer_addr().unwrap());

                let mut buffer = [0; 1024];

                // Envoi du message de demande de mot de passe
                let message = "Entrez le mot de passe : ";
                stream.write(message.as_bytes()).unwrap();

                // Lecture du mot de passe envoyé par le client
                stream.read(&mut buffer).unwrap();
                let password = String::from_utf8_lossy(&buffer[..]).trim().to_string();

                // Vérification du mot de passe
                if password != "my_secret_password" {
                    let message = "Mot de passe incorrect";
                    stream.write(message.as_bytes()).unwrap();
                    continue;
                }

                // Envoi du message de demande de nom d'utilisateur
                let message = "Entrez votre nom d'utilisateur : ";
                stream.write(message.as_bytes()).unwrap();

                // Lecture du nom d'utilisateur envoyé par le client
                stream.read(&mut buffer).unwrap();
                let username = String::from_utf8_lossy(&buffer[..]).trim().to_string();
                println!("L'utilisateur {} est connecté", username);

                loop {
                    // Envoi du message de demande de commande
                    let message = "> ";
                    stream.write(message.as_bytes()).unwrap();

                    // Lecture de la commande envoyée par le client
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer).unwrap();
                    let command = String::from_utf8_lossy(&buffer[..]).trim().to_string();

                    if command.to_lowercase() == "/quit" {
                        break;
                    }

                    // Traitement de la commande
                    let response = match command.as_str() {
                        "/help" => "Liste des commandes : /help, /quit",
                        _ => "Commande inconnue",
                    };

                    // Envoi de la réponse
                    stream.write(response.as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("Erreur de connexion : {}", e);
            }
        }
    }
}
