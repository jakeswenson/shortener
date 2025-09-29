# Shortener: A Cloudflare Workers URL Shortener

A fast, serverless URL shortener service built with Rust and deployed on Cloudflare's global edge network. This service provides instant URL redirection with global low-latency access through Cloudflare's distributed infrastructure.

## Features

- **Edge-first architecture**: Deployed across Cloudflare's global network for minimal latency
- **Rust-powered**: Written in Rust and compiled to WebAssembly for maximum performance
- **KV storage**: Uses Cloudflare KV for persistent, globally distributed URL mappings
- **Instant redirects**: Sub-millisecond redirect responses from edge locations
- **Observability**: Built-in logging and monitoring through Cloudflare Workers

## How it Works

The service operates with a simple routing structure:

1. **Root path (`/`)**: Redirects to a configured home URL (set via `HOME` environment variable)
2. **Short URLs (`/:url`)**: Looks up the URL parameter in Cloudflare KV storage and redirects to the target URL if found

URL mappings are stored in Cloudflare KV, allowing for:
- Global distribution and replication
- Fast lookup times from any edge location
- Persistent storage without managing databases

## Development

### Prerequisites

- [Rust](https://rustup.rs/) with `wasm32-unknown-unknown` target
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- Cloudflare account with Workers and KV enabled

### Commands

```bash
# Run local development server with file watching and hot reload
wrangler dev

# Deploy to Cloudflare Workers globally
wrangler deploy

# Build the project (compiles Rust to WebAssembly) - for testing only
wrangler deploy --dry-run --outdir=dist
```

### Configuration

Edit `wrangler.toml` to configure:
- Environment variables (like `HOME` redirect URL)
- KV namespace bindings
- Workers deployment settings

## Architecture

- **Runtime**: Cloudflare Workers (V8 isolates)
- **Language**: Rust compiled to WebAssembly
- **Storage**: Cloudflare KV (globally distributed key-value store)
- **Build**: Uses `worker-build` for optimized WASM compilation

