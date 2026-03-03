# mdbook-mermaid

An [mdbook](https://rust-lang.github.io/mdBook/) preprocessor that renders [Mermaid](https://mermaid.js.org/) diagrams to inline SVG at build time.

## Features

- **Inline SVG** — diagrams are rendered during build, no JavaScript required at runtime
- **5 mdbook themes** — generates a variant for each theme (Light, Navy, Coal, Ayu, Rust) with automatic switching via CSS
- **Pure Rust** — uses [mermaid-rs-renderer](https://crates.io/crates/mermaid-rs-renderer) for rendering

## Installation

```sh
cargo install mdbook-mermaid
```

## Configuration

Add the preprocessor to your `book.toml`:

```toml
[preprocessor.mermaid]
command = "mdbook-mermaid"
```

## Usage

Write Mermaid diagrams in fenced code blocks:

````markdown
```mermaid
graph LR
  A[Start] --> B[End]
```
````

The preprocessor replaces each block with inline SVG wrapped in theme-specific `<div>` elements. The correct variant is shown automatically based on the active mdbook theme.

## License

[MIT](LICENSE)
