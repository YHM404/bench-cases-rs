use std::io::{ErrorKind, IoSlice};

use anyhow::Result;
use bytes::Bytes;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub async fn writev(bufs: Vec<Bytes>, writer: &mut (dyn AsyncWrite + Unpin)) -> Result<()> {
    let mut bufs: Vec<IoSlice> = bufs.iter().map(|b| IoSlice::new(b)).collect();
    let mut bufs: &mut [IoSlice] = bufs.as_mut();

    while !bufs.is_empty() {
        match writer.write_vectored(&bufs).await {
            Ok(n) => IoSlice::advance_slices(&mut bufs, n),
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}
