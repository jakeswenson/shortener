# Shortener: A Cloudflare workers url shortener

This repo contains my url shortener which runs on
Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/).

## Usage

With `wrangler`, you can build, test, and deploy this Worker with the following commands:

```bash
# compiles this project to WebAssembly and will warn of any issues
wrangler build

# run this Worker in an ideal development workflow (with a local server, file watcher & more)
wrangler dev

# deploy this Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
wrangler publish
```

