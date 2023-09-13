## Rust web development

Walkthrough of "Rust Web Development" by Bastian Gruber.

https://rustwebdevelopment.com/

## Getting started

1. `cargo install sqlx-cli`[^1]
1. `sqlx database create -D <DATABASE_URL>`
1. `sqlx migrate run -D <DATABASE_URL>`
1. Create `.env` file from `.env_template`
1. Start postgres db `docker compose up`
1. `cargo run`

[^1]: https://crates.io/crates/sqlx-cli
