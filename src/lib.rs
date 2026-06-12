//! SchemaLock Zed extension: registers `schemalock serve --stdio` as the YAML
//! language server, downloading the pinned per-platform binary from the public
//! SchemaLock CDN on first use.

use zed_extension_api::{self as zed, LanguageServerId, Result};

/// The pinned `schemalock/app` release tag whose binary this extension version
/// ships against. Keep in lockstep with the `.app-version` file (the file is the
/// human-facing source of truth; this const is what actually drives the
/// download, because the WASM sandbox cannot read repo files at runtime).
const APP_VERSION: &str = "v0.3.2";

/// Public CDN prefix under which the per-tag binaries are published. The full
/// asset URL is `<CDN_BASE>/<APP_VERSION>/<asset>`. We download straight from the
/// CDN (not the GitHub API) because `schemalock/app` is a private repository —
/// only the binaries are public.
const CDN_BASE: &str = "https://cdn.schemalock.dev/bin";

struct SchemaLockExtension;

impl SchemaLockExtension {
    /// Resolves the cached binary path, downloading the pinned release asset on
    /// first use.
    fn binary_path(&self, id: &LanguageServerId) -> Result<String> {
        let (os, arch) = zed::current_platform();
        let platform = schemalock_zed_core::Platform {
            os: os_str(os),
            arch: arch_str(arch),
        };
        let asset = schemalock_zed_core::asset_name(&platform).ok_or_else(|| {
            format!(
                "SchemaLock has no prebuilt binary for {}/{}",
                platform.os, platform.arch
            )
        })?;

        let dir = format!("schemalock-{APP_VERSION}");
        let bin_path = format!("{dir}/{asset}");

        if std::fs::metadata(&bin_path).is_ok_and(|m| m.is_file()) {
            return Ok(bin_path);
        }

        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        // `download_file` does not create parent directories, so make the
        // version dir before writing the binary into it.
        std::fs::create_dir_all(&dir).map_err(|e| format!("creating {dir}: {e}"))?;

        let url = format!("{CDN_BASE}/{APP_VERSION}/{asset}");
        zed::download_file(&url, &bin_path, zed::DownloadedFileType::Uncompressed)?;
        zed::make_file_executable(&bin_path)?;
        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(bin_path)
    }
}

impl zed::Extension for SchemaLockExtension {
    fn new() -> Self {
        SchemaLockExtension
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = self.binary_path(language_server_id)?;
        Ok(zed::Command {
            command: path,
            args: vec!["serve".into(), "--stdio".into()],
            env: vec![],
        })
    }
}

/// Maps Zed's `Os` enum to the lowercase strings `schemalock-zed-core` expects.
fn os_str(os: zed::Os) -> &'static str {
    match os {
        zed::Os::Mac => "mac",
        zed::Os::Linux => "linux",
        zed::Os::Windows => "windows",
    }
}

/// Maps Zed's `Architecture` enum to the lowercase strings `schemalock-zed-core`
/// expects.
fn arch_str(arch: zed::Architecture) -> &'static str {
    match arch {
        zed::Architecture::Aarch64 => "aarch64",
        zed::Architecture::X8664 => "x8664",
        zed::Architecture::X86 => "x86",
    }
}

zed::register_extension!(SchemaLockExtension);
