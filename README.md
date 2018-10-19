# lsk-shorty
Pure rust-tool to brute-force short Lisk addresses.

Requires Rust 2018 (nightly) 1.30.0: https://rustup.rs/

```bash
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=beta -y
```

Change `N_THREADS` to the number of your processor cores and make sure you compile it in optimized `--release` mode:

```bash
$ cargo build --release -j $(nproc)
$ ./target/release/lsk-shorty
```

Example output:

```bash
#53 *** FOUND TARGET 12; next target: 11 in ~"12hou". 103245730 iterations, 387.919/s/t
  12  82332474551L  "zone daring nest doctor robust roast cradle dance fence cook harvest awesome"
```

Happy brute-forcing!
