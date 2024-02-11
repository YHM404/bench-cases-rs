use anyhow::Result;
use bytes::Bytes;
use clap::{command, Parser};
use my_bench::{write_all, write_buffer, writev};
use tokio::{
    io::{AsyncWrite, BufWriter},
    net::TcpStream,
};

#[derive(Parser, Debug)]
enum WriteType {
    All,
    Vec,
    Buffer,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    write_type: WriteType,

    #[arg(long, default_value_t = 1000)]
    data_batch: usize,

    #[arg(long, default_value_t = 1000)]
    data_size: usize,

    #[arg(long, default_value_t = 1000)]
    batches: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let data_batch = args.data_batch;
    let data_size = args.data_size;
    let batches = args.batches;
    let write_type = args.write_type;

    let data = vec![Bytes::from(vec![0; data_size]); data_batch];

    let client = TcpStream::connect("[::1]:8080").await?;
    let mut writer: Box<dyn AsyncWrite + Unpin> = match write_type {
        WriteType::Buffer => Box::new(BufWriter::with_capacity(data_batch * data_size, client)),
        _ => Box::new(client),
    };

    for _ in 0..batches {
        match write_type {
            WriteType::All => write_all(data.clone(), &mut writer).await?,
            WriteType::Vec => writev(data.clone(), &mut writer).await?,
            WriteType::Buffer => write_buffer(data.clone(), &mut writer).await?,
        }
    }
    Ok(())
}
