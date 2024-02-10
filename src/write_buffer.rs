use anyhow::Result;
use bytes::Bytes;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub async fn write_buffer(bufs: Vec<Bytes>, writer: &mut (dyn AsyncWrite + Unpin)) -> Result<()> {
    for b in bufs {
        writer.write_all(&b).await?;
    }
    writer.flush().await?;
    Ok(())
}
