# lsk-shorty
Pure Rust-tool to brute-force short Lisk addresses.

Requires Rust 1.25.0: https://rustup.rs/

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

Change `NTHREADS` to the number of your processor cores and make sure you compile it in optimized `--release` mode:

```bash
$ cargo build --release -j $(nproc)
$ ./target/release/lsk-shorty
```

Happy brute-forcing!
