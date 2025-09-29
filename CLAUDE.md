# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Architecture

This is a Cloudflare Workers project written in Rust that implements a URL shortener service. The application:

- Uses Cloudflare Workers runtime with the `worker` crate for serverless edge computing
- Stores shortened URLs in Cloudflare KV (Key-Value) storage via the `urls` binding
- Compiles to WebAssembly (WASM) for deployment to Cloudflare's edge network
- Uses a simple router pattern with two main routes:
  - `/` - redirects to the HOME environment variable (configured in wrangler.toml)
  - `/:url` - looks up the URL parameter in KV storage and redirects if found

## Key Files

- `src/lib.rs` - Main application logic with router and request handlers
- `src/utils.rs` - Utility functions including panic hook setup for debugging
- `wrangler.toml` - Cloudflare Workers configuration including KV namespace bindings
- `Cargo.toml` - Rust dependencies and WebAssembly build configuration

## Commands

### Development
```bash
# Run local development server with file watching
wrangler dev

# Deploy to Cloudflare Workers
wrangler deploy

# Build/test the project (compiles Rust to WASM) - for testing only
wrangler deploy --dry-run --outdir=dist
```

### Rust Toolchain
```bash
# Build release version (used by wrangler)
cargo build --target wasm32-unknown-unknown --release

# Check code without building
cargo check

# Run clippy linter
cargo clippy
```

## Configuration

- Environment variables are defined in `wrangler.toml` under `[vars]`
- KV namespace binding `urls` is configured for URL storage
- WebAssembly build uses `worker-build` tool for proper Cloudflare Workers compilation
- Uses `wasm32-unknown-unknown` target for WebAssembly output

## Features

- `small` (default): Uses `wee_alloc` for smaller WASM binary size
- `debug`: Enables `console_error_panic_hook` for better error debugging in development