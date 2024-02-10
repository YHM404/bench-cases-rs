# BENCHES-IN-RUST

This repository is used to implement benchmarks for some scenes that interest me.

## preinstall

``` BASH
    # install flamegraph
    cargo install flamegraph
```

## TCP write

This benchmark is used to test the performance of writing data to a TCP connection.

``` BASH
    # Run the server
    cargo run --release --bin tcp_server
    # Run the benchmark
    cargo bench tcp-write
```

Show the flamegraph

``` BASH
    # flamegraph of `writev`.
    cargo flamegraph --bin tcp_client -- vec
    # flamegraph of `write`.
    cargo flamegraph --bin tcp_client -- all
    # flamegraph of `write` with a buffer.
    cargo flamegraph --bin tcp_client -- buffer
```
