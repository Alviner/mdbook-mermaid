# Contributing

## Setup

```sh
git clone https://github.com/alvinera/mdbook-mermaid.git
cd mdbook-mermaid
cargo build
```

## Development

Run tests:

```sh
cargo test
```

Check formatting and lints:

```sh
cargo fmt --check
cargo clippy
```

## Pull requests

1. Fork the repository and create a feature branch
2. Make your changes
3. Ensure `cargo test`, `cargo clippy`, and `cargo fmt --check` pass
4. Open a pull request with a clear description of the change
