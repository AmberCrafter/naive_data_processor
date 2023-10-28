use std::{net::TcpStream, error::Error, io::{Write, Read}, thread::{sleep, self}, time::Duration, sync::{Arc, Mutex}};

// fn main() -> Result<(), Box<dyn Error + 'static>> {
fn main() -> ! {
    loop {
        match connecter() {
            Err(e) => {
                println!("wait server...");
                sleep(Duration::from_secs(5));
            },
            Ok(_) => {}
        }
    }
    // Ok(())
}


fn connecter() -> Result<(), Box<dyn Error + 'static>> {
    let stream = TcpStream::connect("127.0.0.1:12345")?;
    let stream_lock = Arc::new(Mutex::new(stream));

    let stream_lock_clone = stream_lock.clone();
    let stream_reader = thread::spawn(move ||
        {
            loop {
                match stream_lock_clone.lock() {
                    Ok(mut lck) => {
                        let mut rx_bytes =  [0u8; 512];
                        let rx_len = lck.read(&mut rx_bytes);
                        match rx_len {
                            Ok(len) => {
                                println!("{:?}", String::from_utf8_lossy(&rx_bytes[..len]));
                            },
                            Err(_) => {}
                        }
                    },
                    Err(_) => {}
                }
                sleep(Duration::from_secs(1));
            }
        }
    );

    let mut buffer = String::new();
    loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer)?;

        match buffer.as_str().trim() {
            "exit" => {
                stream_lock.lock().unwrap().write_all("exit".as_bytes())?;
                break;
            },
            _ => stream_lock.lock().unwrap().write_all(buffer.as_bytes())?,
        }
    }
    Ok(())
}