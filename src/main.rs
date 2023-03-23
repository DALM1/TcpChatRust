use std::io::{self, prelude::*, BufReader};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:5000")?;

    let mut password = String::new();
    print!("Entrez le mot de passe: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut password)?;

    stream.write_all(password.as_bytes())?;
    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    print!("{}", response);

    if response.trim() == "Mot de passe incorrect" {
        return Ok(());
    }

    let mut username = String::new();
    print!("Entrez votre nom d'utilisateur: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut username)?;
    stream.write_all(username.as_bytes())?;

    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    print!("{}", response);

    loop {
        let mut message = String::new();
        io::stdin().read_line(&mut message)?;
        stream.write_all(message.as_bytes())?;

        let mut reader = BufReader::new(&stream);
        let mut response = String::new();
        reader.read_line(&mut response)?;
        print!("{}", response);

        if message.to_lowercase().trim() == "/quit" {
            break;
        }
    }

    Ok(())
}
