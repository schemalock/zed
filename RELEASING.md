# Releasing the SchemaLock Zed extension

The extension versions independently of `schemalock/app`. `.app-version` and the
`APP_VERSION` const in `src/lib.rs` pin which app tag the extension downloads;
they must always match.

## Bumping the pinned app version

1. Confirm the target app tag exists as a GitHub release with all five assets
   (`schemalock-{darwin-arm64,darwin-x64,linux-arm64,linux-x64}` and
   `schemalock-win32-x64.exe`):
   `gh release view -R schemalock/app <tag> --json assets -q '.assets[].name'`
2. Update `.app-version` and `const APP_VERSION` in `src/lib.rs` to the new tag.
3. Bump `version` in both `extension.toml` and `Cargo.toml`.
4. `cargo build --release --target wasm32-wasip1` to confirm it still compiles.
5. Branch + merge (never commit directly to the default branch) + tag the
   extension at the new version.

> Schema/protocol changes in `schemalock/app` are contract-breaking. Ship the app
> bump and the extension bump in lockstep so `.app-version` always points at a
> compatible app.

## Publishing to the Zed registry

1. Push this repo to its public GitHub home (`schemalock/zed`).
2. Fork [`zed-industries/extensions`](https://github.com/zed-industries/extensions).
3. Add this repo as a git submodule under `extensions/` and add a `schemalock`
   entry to `extensions.toml` pointing at the submodule path + version.
4. Open a PR. The **first** merged PR reserves the extension id `schemalock`.
5. Marketplace versions are immutable — every shipped change needs a new
   `extension.toml` version bump.
