use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;

    loop {
        let (mut stream, _) = listener.accept().await?;
        println!("Nouveau client");

        let mut buf = [0;1024];
        let _ = stream.read(&mut buf).await?;

        println!("{}", String::from_utf8_lossy(&buf));

        let _response204 = b"HTTP/1.1 204 No Content\r\nContent-Length: 0\r\n\r\n";
        let response200 = b"HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello World!";
        stream.write_all(response200).await?;

        println!("{}\n\n", "-".repeat(50));
    }
}