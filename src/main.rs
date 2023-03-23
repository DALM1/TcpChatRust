use std::io::{self, prelude::*, BufReader};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Entrez le mot de passe: ");
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    write!(stream, "{}", password)?;
    let mut stream_clone = stream.try_clone()?;
    let mut reader = BufReader::new(&stream);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    if buffer.trim() == "password accepted" {
        println!("Le mot de passe est accepté!");
        loop {
            let mut message = String::new();
            io::stdin().read_line(&mut message)?;
            write!(stream_clone, "{}", message)?;
            let mut buffer = [0; 1024];
            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            print!("{}", str::from_utf8(&buffer[..n]).expect("Invalid UTF-8"));
        }
    } else {
        println!("Le mot de passe est incorrect!");
    }
    Ok(())
}
