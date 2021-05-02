use std::{
    io::{BufRead, BufReader},
    net::{Shutdown, TcpListener, TcpStream},
};

const ADDRESS: &str = "127.0.0.1:8080";

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    println!("New connection.");
    let mut reader = BufReader::new(&stream);

    loop {
        let mut buf = String::new();

        match reader.read_line(&mut buf) {
            Ok(size) => {
                // Empty buffer means EOF (closed connection)
                if size == 0 {
                    break;
                }

                // Print output
                print!("{}", buf);
            }
            Err(e) => {
                // Print read error and shut down stream
                eprintln!("An error occured when reading from socket: {}", e);
                stream.shutdown(Shutdown::Both)?;
                break;
            }
        }
    }

    println!("Connection closed.");
    Ok(())
}

fn main() {
    let listener = TcpListener::bind(ADDRESS).expect("Could not bind to address");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream).expect("Could not handle client");
        }
    }
}
