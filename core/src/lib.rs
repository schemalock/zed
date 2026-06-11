//! Pure platform → GitHub-release-asset mapping for the SchemaLock Zed
//! extension. This crate has no `zed_extension_api` dependency so it is
//! unit-testable on the host with plain `cargo test`.

/// A resolved host platform, expressed as the lowercase strings the Zed glue
/// derives from `zed::current_platform()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Platform {
    /// "mac" | "linux" | "windows"
    pub os: &'static str,
    /// "aarch64" | "x86" | "x8664"
    pub arch: &'static str,
}

/// Returns the GitHub release asset name for `platform`, or `None` if SchemaLock
/// publishes no prebuilt binary for that os/arch combination.
///
/// Asset names are the exact `schemalock/app` release asset filenames.
pub fn asset_name(platform: &Platform) -> Option<&'static str> {
    match (platform.os, platform.arch) {
        ("mac", "aarch64") => Some("schemalock-darwin-arm64"),
        ("mac", "x8664") => Some("schemalock-darwin-x64"),
        ("linux", "aarch64") => Some("schemalock-linux-arm64"),
        ("linux", "x8664") => Some("schemalock-linux-x64"),
        ("windows", "x8664") => Some("schemalock-win32-x64.exe"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(os: &'static str, arch: &'static str) -> Platform {
        Platform { os, arch }
    }

    #[test]
    fn maps_supported_platforms() {
        let cases = [
            (("mac", "aarch64"), "schemalock-darwin-arm64"),
            (("mac", "x8664"), "schemalock-darwin-x64"),
            (("linux", "aarch64"), "schemalock-linux-arm64"),
            (("linux", "x8664"), "schemalock-linux-x64"),
            (("windows", "x8664"), "schemalock-win32-x64.exe"),
        ];
        for ((os, arch), want) in cases {
            assert_eq!(asset_name(&p(os, arch)), Some(want), "for {os}/{arch}");
        }
    }

    #[test]
    fn unsupported_platforms_return_none() {
        assert_eq!(asset_name(&p("linux", "x86")), None);
        assert_eq!(asset_name(&p("windows", "aarch64")), None);
        assert_eq!(asset_name(&p("freebsd", "x8664")), None);
    }
}
