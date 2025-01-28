## cargo-cleaner

cargo-cleaner is a Rust program that cleans your Rust projects, returning valuable disk space.

### Usage:

#### Checkout the code:

```bash
cd ~/workspace/rust

git clone https://github.com/gdonald/cargo-cleaner

cd cargo-cleaner
```

#### Build and run it, telling it where to clean:

```bash
cargo run -- ~/workspace/rust/                                                                                                                                                         ─╯
   Compiling cargo-cleaner v1.0.0 (/Users/gd/workspace/rust/cargo-cleaner)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/cargo-cleaner /Users/gd/workspace/rust/`

[...]

Total disk space cleaned: 2.54 GB
```
