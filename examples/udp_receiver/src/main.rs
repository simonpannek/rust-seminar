use std::{net::UdpSocket, str::from_utf8};

const ADDRESS: &str = "127.0.0.1:8080";

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(ADDRESS)?;

    loop {
        let mut buf = [0; 1500];

        match socket.recv_from(&mut buf) {
            Ok(_) => {
                // Print received data
                print!(
                    "{}",
                    from_utf8(&buf).expect("Failed to convert data to str")
                );
            }
            Err(e) => {
                // Print read error
                eprintln!("An error occured when reading from socket: {}", e);
                break;
            }
        }
    }

    Ok(())
}
