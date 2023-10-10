use std::{net::{TcpListener, TcpStream}, error::Error, io::{BufReader, BufRead}};

fn connection_handler(stream: TcpStream) {
    let mut buf_reader = BufReader::new(stream);

    let mut buffer = String::new();
    loop {
        buffer.clear();
        buf_reader.read_line(&mut buffer).expect("System Error");

        match buffer.trim() {
            "exit" => {
                // stream.shutdown(std::net::Shutdown::Both).expect("Stream shutdown failed");
                break;
            },
            msg => {
                println!("{}", msg);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    // let mut args = Cli::parse();
    // println!("cli args: {:?}", args);
    // let qc_level = args.level.unwrap_or(usize::MAX);
    // let data_format = args.format.split(',').collect::<Vec<_>>();


    let listener = TcpListener::bind("127.0.0.1:12345")?;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        connection_handler(stream);
    }
    Ok(())
}