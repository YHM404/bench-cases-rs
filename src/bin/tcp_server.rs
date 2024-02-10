use anyhow::Result;
use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<()> {
    init_tcp_server().await
}

pub async fn init_tcp_server() -> Result<()> {
    let server = TcpListener::bind("[::1]:8080").await?;
    loop {
        let (mut socket, _) = server.accept().await?;
        let mut buf = [0; 200000];
        loop {
            if socket.read(&mut buf).await? == 0 {
                eprintln!("socket closed");
                break;
            }
        }
    }
}
