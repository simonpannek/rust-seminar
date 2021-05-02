use std::{
    io::{stdin, Write},
    net::{Shutdown, TcpStream},
};

const ADDRESS: &str = "127.0.0.1:8080";

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(ADDRESS)?;

    loop {
        let mut buf = String::new();

        match stdin().read_line(&mut buf) {
            Ok(_) => {
                // Close connection if client writes "exit"
                if buf.trim().to_lowercase() == "exit" {
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }

                // Send buffer to server
                stream
                    .write_all(buf.as_bytes())
                    .expect("Failed to write to server");
            }
            Err(e) => {
                // Print read error and shut down stream
                eprintln!("An error occured when reading from stdin: {}", e);
                stream.shutdown(Shutdown::Both)?;
                break;
            }
        }
    }

    Ok(())
}
