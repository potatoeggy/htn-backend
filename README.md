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

### Test

Run everything in `src/tests` via Cargo (ensure that the database is in a clean state, freshly migrated, for everything to pass):

```bash
cargo test
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

## Endpoints

Basically, all minimum expectations are complete.

`GET /users` returns all users.

**Query parameters**:

- `name`: filter by name (optional)
- `company`: filter by company (optional)
- `email`: filter by email (optional)
- `has_skill[]`: contains any of the skills provided, can be repeated (optional)

Returns 400 if any of the query parameters are invalid. If `has_skill[]` is provided, users that have at least one of the skills provided will be included with a `skills` response that only includes those that match the query.

```json
{
    "name": <string>,
    "company": <string>,
    "email": <string>,
    "phone": <string>,
    "skills": [
        {
            "name": <string>,
            "rating": <int>
        }
    ],
    ...
}
```

`GET /users/:id` returns one user, where `id` is the `id`th user inserted into the database. Returns 404 if the user does not exist.

```json
{
    "name": <string>,
    "company": <string>,
    "email": <string>,
    "phone": <string>,
    "skills": [
        {
            "name": <string>,
            "rating": <int>
        },
        ...
    ]
}
```

`PUT /users/:id` allows partial/full updates of one user. Returns 404 if the user does not exist, or 400 if the request is malformed. Otherwise returns new user data. The schema below is for both the request and the response. All fields are optional, and omitted fields will not be updated.

```json
{
    "name": <string>,
    "company": <string>,
    "email": <string>,
    "phone": <string>,
    "skills": [
        {
            "name": <string>,
            "rating": <int>
        },
        ...
    ]
}
```

`GET /skills` returns a skill frequency distribution.

**Query parameters:**

- `max_freq`: maximum frequency to return (inclusive, optional)
- `min_freq`: minimum frequency to return (inclusive, optional)

Returns 400 if `max_freq` or `min_freq` are not integers.

```json
[
    {
        "name": <string>,
        "frequency": <int>
    },
    ...
]
```

## Details

This project was made to learn Rust, so it's not nearly as clean as I hoped it
would be. There are lots of odd niggles in the language (borrow checker) that are
endlessly fascinating but also occasionally quite frustrating — many practices
from my home languages, Python and TypeScript, don't really transfer over.

I miss duck typing :(

Codebase highlights:

- `migrations/**/up.sql`: database schema
- `src/main.rs`: main entrypoint
- `src/bin/load_initial.rs`: initial migrations to copy sample data to the database
- `src/models.rs`: the many, many different structs to deserialise or serialise to depending on the situation — I miss TS most here (or just a better ORM)
- `src/server.rs`: REST endpoint functions
- `src/lib.rs`: a bunch of helper functions as well as configuration handling

### Improvements

- If performance starts to suffer, slap a caching layer on top / use Postgres
- If making this in the real world, use a language that doesn't take years to code in such as Python, TypeScript, or Golang
