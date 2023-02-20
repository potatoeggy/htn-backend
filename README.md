# htn-backend

Written for the 2023 Hack the North backend challenge and to play around with Rust. Hello!

## Usage

Cargo and Rust 1.62+ must first be available on your system. [Here are some instructions to install them](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you haven't already.

### Configuration

Create a `.env` file or rename the sample file.

### Setup

If you'd like, you can use the included `db.sample.sqlite3` and skip all of these steps.

```bash
cargo install diesel_cli --no-default-features --features sqlite
diesel setup
cargo run --bin load_initial  # initial database migration
```

### Run

Run in debug mode:

```bash
cargo run --bin htn-backend
```

Feel free to build in release mode if you need that sweet, sweet performance:

```bash
cargo run --bin htn-backend --release
```

## Details

This project was made to learn Rust, so it's not nearly as clean as I hoped it
would be. There are lots of odd niggles in the language (borrow checker) that are
endlessly fascinating but also occasionally quite frustrating — many practices
from my home languages, Python and TypeScript, don't really transfer over.

I miss duck typing :(

Codebase highlights:

- `src/schema.rs`: generated type definitions for the ORM
- `src/main.rs`: main entrypoint
- `src/bin/load_initial.rs`: initial migrations to copy sample data to the database
- `src/models.rs`: the many, many different structs to deserialise or serialise to depending on the situation — I miss TS most here
- `src/server.rs`: REST endpoint functions
- `src/lib.rs`: a bunch of helper functions as well as configuration handling

### Improvements

- If performance starts to suffer, slap a caching layer on top / use Postgres
- If making this in the real world, use a language that doesn't take years to code in such as Python, TypeScript, or Golang
