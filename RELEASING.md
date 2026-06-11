# Releasing the SchemaLock Zed extension

The extension versions independently of `schemalock/app`. `.app-version` and the
`APP_VERSION` const in `src/lib.rs` pin which app tag the extension downloads;
they must always match.

> **Why the CDN, not GitHub releases?** `schemalock/app` is a **private** repo, so
> Zed's anonymous GitHub API calls 404. The extension instead downloads from the
> public CDN at `https://cdn.schemalock.dev/bin/<tag>/<asset>`. Publishing those
> objects is part of the app release (see "Publishing binaries to the CDN" below).

## Bumping the pinned app version

1. Confirm the target app tag's binaries are published to the CDN — all five
   assets must return HTTP 200:
   ```bash
   for a in darwin-arm64 darwin-x64 linux-arm64 linux-x64 win32-x64.exe; do
     curl -s -o /dev/null -w "%{http_code} $a\n" \
       "https://cdn.schemalock.dev/bin/<tag>/schemalock-$a"
   done
   ```
   If any 404, publish them first (next section).
2. Update `.app-version` and `const APP_VERSION` in `src/lib.rs` to the new tag.
3. Bump `version` in both `extension.toml` and `Cargo.toml`.
4. `cargo build --release --target wasm32-wasip2` to confirm it still compiles
   (Zed builds extensions for `wasm32-wasip2`).
5. Branch + merge (never commit directly to the default branch) + tag the
   extension at the new version.

> Schema/protocol changes in `schemalock/app` are contract-breaking. Ship the app
> bump and the extension bump in lockstep so `.app-version` always points at a
> compatible app.

## Publishing binaries to the CDN

The five per-platform binaries for a tag must live at
`s3://schemalock-cdn/bin/<tag>/schemalock-<os>-<arch>` (served publicly at
`https://cdn.schemalock.dev/bin/<tag>/`). The app release workflow
(`app/.github/workflows/release.yml`) does this automatically when an app tag is
pushed, provided the repo has the `R2_ACCOUNT_ID` / `R2_ACCESS_KEY_ID` /
`R2_SECRET_ACCESS_KEY` Actions secrets.

To publish (or backfill) a tag manually from a checkout with R2 credentials and
the binaries in `dist/`:

```bash
aws s3 cp dist/ "s3://schemalock-cdn/bin/<tag>/" \
  --recursive --exclude "*" --include "schemalock-*" \
  --endpoint-url "https://${R2_ACCOUNT_ID}.r2.cloudflarestorage.com"
```

The CDN path layout (`bin/<tag>/`) is the contract this extension's `CDN_BASE`
const depends on; keep them in lockstep.

## Publishing to the Zed registry

1. Push this repo to its public GitHub home (`schemalock/zed`).
2. Fork [`zed-industries/extensions`](https://github.com/zed-industries/extensions).
3. Add this repo as a git submodule under `extensions/` and add a `schemalock`
   entry to `extensions.toml` pointing at the submodule path + version.
4. Open a PR. The **first** merged PR reserves the extension id `schemalock`.
5. Marketplace versions are immutable — every shipped change needs a new
   `extension.toml` version bump.
