use bytes::BytesMut;
use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::error;
use crate::error::ErrorCode;

#[allow(dead_code)]
pub(crate) fn local_main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _ = runtime.block_on(async_main());
}


async fn async_main() -> Result<(), ErrorCode> {
    let listener = TcpListener::bind("0.0.0.0:9999").await?;
    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(
            async move {
                let mut buf = BytesMut::with_capacity(2048);
                loop {
                    let n = match stream.read_buf(&mut buf).await {
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            error!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };
                    if let Err(e) = stream.write_all(&buf[0..n]).await {
                        error!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                    buf.clear();
                }
            }
        );
    }
}