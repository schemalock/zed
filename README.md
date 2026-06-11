# SchemaLock for Zed

Schema-pinned validation, completion, and hover for Kubernetes-style YAML in the
[Zed](https://zed.dev) editor, backed by the SchemaLock CDN.

## What it does

Registers the `schemalock` language server for YAML files. In a project rooted at
a `schemalock.yaml`, owned resources (e.g. a VMCluster) get completion, hover, and
diagnostics from the schema pinned by the nearest `schemalock.yaml`. Unowned YAML
is transparently proxied to `yaml-language-server`.

This is a thin client. The actual language server is the `schemalock` binary from
[`schemalock/app`](https://github.com/schemalock/app); the extension downloads the
pinned release (currently `v0.3.2`) for your platform on first use and caches it.

> **Not available:** the schema-version status-bar picker shipped in the VS Code
> and JetBrains clients is not present here — Zed extensions have no custom-UI API.

## Requirements

- Network access on first use (to download the binary from GitHub releases).
- A supported platform: macOS (arm64/x64), Linux (arm64/x64), Windows (x64).

## YAML language-server coexistence

If you see duplicate diagnostics or completions, Zed's built-in YAML language
server is running alongside SchemaLock. SchemaLock already proxies to
`yaml-language-server` internally, so make it the sole YAML server in your Zed
settings:

```jsonc
{
  "languages": {
    "YAML": {
      "language_servers": ["schemalock", "!yaml-language-server"]
    }
  }
}
```

## Development

```bash
rustup target add wasm32-wasip2                 # the target Zed compiles extensions for
cargo build --release --target wasm32-wasip2    # verify it compiles
cargo test -p schemalock-zed-core               # run unit tests
```

Then in Zed: command palette → **zed: install dev extension** → select this
directory. Zed builds the extension itself for `wasm32-wasip2` using a
`rustup`-managed toolchain, so that target must be installed (the command above
adds it).
