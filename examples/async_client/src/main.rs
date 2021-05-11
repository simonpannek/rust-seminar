use tokio::{
    io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

const ADDRESS: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(ADDRESS).await?;

    let mut reader = BufReader::new(stdin());

    loop {
        let mut buf = String::new();

        match reader.read_line(&mut buf).await {
            Ok(_) => {
                // Close connection if client writes "exit"
                if buf.trim().to_lowercase() == "exit" {
                    stream.shutdown().await?;
                    break;
                }

                // Send buffer to server
                stream.write_all(buf.as_bytes()).await?;
            }
            Err(e) => {
                // Print read error and shut down stream
                eprintln!("An error occured when reading from stdin: {}", e);
                stream.shutdown().await?;
                break;
            }
        }
    }

    Ok(())
}
