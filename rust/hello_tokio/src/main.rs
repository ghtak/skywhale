use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let _ = rt.block_on(aync_main());
}

async fn aync_main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9999").await?;
    loop{
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(
            async move {
                let mut buf = [0; 1024];
                loop {
                    let n = match socket.read(&mut buf).await {
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                }
            }
        );
    }
}