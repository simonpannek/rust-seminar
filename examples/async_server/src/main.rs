use std::io::Error;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

const ADDRESS: &str = "127.0.0.1:8080";

fn handle_client(mut stream: TcpStream) {
    tokio::spawn(async move {
        println!("New connection.");

        let mut reader = BufReader::new(&mut stream);

        loop {
            let mut buf = String::new();

            match reader.read_line(&mut buf).await {
                Ok(0) => {
                    // Empty buffer means EOF (closed connection)
                    break;
                }
                Ok(_) => {
                    // Print received data
                    print!("{}", buf);
                }
                Err(e) => {
                    // Print read error and shut down stream
                    eprintln!("An error occured when reading from socket: {}", e);
                    stream.shutdown().await?;
                    break;
                }
            }
        }

        println!("Connection closed.");
        Ok::<(), Error>(())
    });
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(ADDRESS).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        handle_client(stream);
    }
}
