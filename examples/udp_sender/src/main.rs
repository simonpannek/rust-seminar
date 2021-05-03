use std::{io::stdin, net::UdpSocket};

const ADDRESS: &str = "127.0.0.1:8080";

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    loop {
        let mut buf = String::new();

        match stdin().read_line(&mut buf) {
            Ok(_) => {
                // Close connection if client writes "exit"
                if buf.trim().to_lowercase() == "exit" {
                    break;
                }

                // Send buffer to server
                socket
                    .send_to(buf.as_bytes(), ADDRESS)
                    .expect("Failed to send data");
            }
            Err(e) => {
                // Print read error
                eprintln!("An error occured when reading from stdin: {}", e);
                break;
            }
        }
    }

    Ok(())
}
