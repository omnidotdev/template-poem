# 📜 Poem GraphQL API Template

This is a template repository for a GraphQL API built with [Poem](https://github.com/poem-web/poem) and [async-graphql](https://github.com/async-graphql/async-graphql) in Rust.

## Features

- 🚀 **Modern Stack**: Built with Rust 2024 edition for performance and safety
- 📊 **GraphQL API**: [async-graphql](https://docs.rs/async-graphql) with Poem integration
- 🔭 **Observability**: OpenTelemetry tracing built-in
- ⚠️ **Error Handling**: [thiserror](https://docs.rs/thiserror) for custom error types
- 🔒 **Safety**:
  - Unsafe code forbidden via lint
  - Strict Clippy lints (pedantic, nursery, cargo)
- ⚡ **Optimized Builds**:
  - LTO (Link-Time Optimization) enabled
  - Single codegen unit for maximum optimization
  - Binary stripping for smaller size
  - Panic abort for reduced binary size
- 🛠️ **Developer Experience**:
  - Comprehensive Clippy configuration
  - Cargo metadata for crates.io publishing
  - Easy spin up with [Tilt](https://tilt.dev)

## Prerequisites

- [Rust](https://rustup.rs) 1.85+

## Local Development

### Installation

```sh
cargo build
```

### Running

```sh
cargo run
```

The server starts at `http://0.0.0.0:3000` with GraphQL endpoint at `/graphql`.

### Testing

```sh
cargo test
```

### Linting

```sh
cargo clippy
```

## GraphQL

Query the GraphQL endpoint:

```sh
curl -X POST http://localhost:3000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ hello }"}'
```

## License

The code in this repository is licensed under Apache 2.0, &copy; [Omni LLC](https://omni.dev). See [LICENSE.md](LICENSE.md) for more information.
