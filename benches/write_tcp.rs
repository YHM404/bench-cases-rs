use bytes::Bytes;
use criterion::{criterion_group, criterion_main, Criterion};
use my_bench::{write_all, write_buffer, writev};
use tokio::{io::AsyncWriteExt, net::TcpStream};

mod utils;

fn criterion_benchmark(c: &mut Criterion) {
    let data_batch = 1000;
    let data_size = 1000;
    let batches = 500;
    let prepare_data = || vec![Bytes::from(vec![0; data_size]); data_batch];
    let runtime = utils::new_tokio_runtime();

    let mut group = c.benchmark_group("tcp-write");
    group.bench_function("write", |b| {
        b.to_async(&runtime).iter(|| {
            let data = prepare_data();
            async move {
                let mut writer = TcpStream::connect("[::1]:8080").await.unwrap();
                for _ in 0..batches {
                    write_all(data.clone(), &mut writer).await.unwrap();
                }
            }
        });
    });

    group.bench_function("writev", |b| {
        b.to_async(&runtime).iter(|| {
            let data = prepare_data();
            async move {
                let mut writer = TcpStream::connect("[::1]:8080").await.unwrap();
                for _ in 0..batches {
                    writev(data.clone(), &mut writer).await.unwrap();
                }
            }
        });
    });

    group.bench_function("write_buffer", |b| {
        b.to_async(&runtime).iter(|| {
            let data = prepare_data();
            async move {
                let writer = TcpStream::connect("[::1]:8080").await.unwrap();
                let mut buffer_writer = tokio::io::BufWriter::with_capacity(100000, writer);
                for _ in 0..batches {
                    write_buffer(data.clone(), &mut buffer_writer)
                        .await
                        .unwrap();
                    buffer_writer.flush().await.unwrap();
                }
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
