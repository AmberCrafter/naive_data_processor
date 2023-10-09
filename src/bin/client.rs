use std::{net::TcpStream, error::Error, io::Write};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let mut stream = TcpStream::connect("127.0.0.1:12345")?;
    let mut buffer = String::new();
    loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer)?;

        match buffer.as_str().trim() {
            "exit" => {
                stream.write_all("exit".as_bytes())?;
                break;
            },
            _ => stream.write_all(buffer.as_bytes())?,
        }
    }
    Ok(())
}