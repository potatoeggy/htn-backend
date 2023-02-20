# htn-backend

Written for the 2023 Hack the North backend challenge.

## Install dependencies

Cargo and Rust must first be available on your system. [Here are some instructions to install them](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you haven't already.

## Configuration

Create a `.env` file or rename the sample file.

## Setup

If you'd like, you can use the included `db.sample.sqlite3` and skip all of these steps.

```bash
cargo install diesel_cli --no-default-features --features sqlite
diesel setup
cargo run --bin load_initial  # initial database migration
```

## Usage

Run in debug mode:

```bash
cargo run --bin htn-backend
```

Feel free to compile it if you want better performance:

```bash
cargo build --release
./target/release/htn-backend
```
