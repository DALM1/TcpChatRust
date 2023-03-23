use std::io::{self, prelude::*, BufReader};
use std::net::{TcpStream};
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:5000")?;
    println!("Entrez le mot de passe: ");
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    write!(stream, "{}", password)?;
    let mut reader = BufReader::new(&stream);
    loop {
        let mut message = String::new();
        io::stdin().read_line(&mut message)?;
        write!(stream, "{}", message)?;
        let mut buffer = [0; 1024];
        reader.read(&mut buffer)?;
        print!("{}", str::from_utf8(&buffer).expect("Invalid UTF-8"));
    }
    Ok(())
}
