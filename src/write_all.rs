use anyhow::Result;
use bytes::Bytes;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub async fn write_all(bufs: Vec<Bytes>, writer: &mut (dyn AsyncWrite + Unpin)) -> Result<()> {
    let buf_len = bufs.iter().map(|b| b.len()).sum();
    let mut buf = Vec::with_capacity(buf_len);
    for b in bufs {
        buf.extend_from_slice(&b);
    }
    writer.write_all(&buf).await?;
    Ok(())
}
