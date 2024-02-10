use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() {
    init_tcp_server().await;
}

pub async fn init_tcp_server() {
    let server = TcpListener::bind("[::1]:8080").await.unwrap();
    loop {
        let (mut socket, _) = server.accept().await.unwrap();
        let mut buf = [0; 200000];
        loop {
            match socket.read(&mut buf).await {
                Ok(0) => {
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            }
        }
    }
}
