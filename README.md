# JSON Validator

A simple JSON validator built with Rust, Leptos, and TailwindCSS 4.

## Features

- Client-side JSON validation using `serde_json`
- Instant feedback with error location (line and column)
- Clean, modern UI styled with TailwindCSS 4

## Development

### Prerequisites

- Rust (with `wasm32-unknown-unknown` target)
- [Trunk](https://trunkrs.dev/) for building
- Node.js (for TailwindCSS)

### Setup

```bash
# Install Rust wasm target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk

# Install dependencies
pnpm install
```

### Run locally

```bash
trunk serve
```

Open http://localhost:8080

### Run tests

```bash
cargo test
```

## Deployment

The app is automatically deployed to GitHub Pages on push to `main`, at
https://a-software-bite.github.io/json-validator/

### GitHub Pages setup (one time)

1. Repository Settings -> Pages -> Build and deployment -> Source: **GitHub Actions**

### Manual deployment

```bash
trunk build --release --public-url /json-validator/
# Upload dist/ to your hosting provider
```
