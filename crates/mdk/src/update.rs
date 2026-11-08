//! "Check for updates": asks GitHub for the latest release, only when the user clicks.
//!
//! mdk compiles in no HTTP client (NFR-02, checked by `cargo deny`). The one request runs
//! through the system `curl`, which ships with macOS, Windows 10 and later, and most Linux.

use std::process::Command;

/// Where the user downloads a new version.
pub const RELEASES_PAGE: &str = "https://github.com/Abdulkader-Safi/MDK/releases/latest";
const LATEST_API: &str = "https://api.github.com/repos/Abdulkader-Safi/MDK/releases/latest";

/// The newest release's tag, such as `v0.1.4`. Blocks for up to 10 seconds, so call it
/// off the UI thread.
pub fn latest_tag() -> Result<String, String> {
    let mut curl = Command::new("curl");
    curl.args(["-fsSL", "--max-time", "10"])
        .args(["-H", "Accept: application/vnd.github+json"])
        .args(["-H", concat!("User-Agent: mdk/", env!("CARGO_PKG_VERSION"))])
        .arg(LATEST_API);
    // mdk is a GUI app on Windows. Without this, curl opens a console window for a moment.
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        curl.creation_flags(CREATE_NO_WINDOW);
    }
    let out = curl
        .output()
        .map_err(|e| format!("curl did not start: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    let release: serde_json::Value =
        serde_json::from_slice(&out.stdout).map_err(|e| format!("not a release: {e}"))?;
    release["tag_name"]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| "the release has no tag".into())
}

/// True when `latest` (a tag like `v0.1.5`) is a higher version than `current` (`0.1.4`).
/// Anything that is not dotted numbers counts as not newer.
pub fn is_newer(latest: &str, current: &str) -> bool {
    fn parts(v: &str) -> Option<Vec<u64>> {
        v.trim_start_matches('v')
            .split('.')
            .map(|p| p.parse().ok())
            .collect()
    }
    match (parts(latest), parts(current)) {
        (Some(latest), Some(current)) => latest > current,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_newer;

    #[test]
    fn compares_versions_as_numbers() {
        assert!(is_newer("v0.1.5", "0.1.4"));
        assert!(is_newer("v0.2.0", "0.1.9"));
        assert!(is_newer("v0.1.10", "0.1.9"));
        assert!(is_newer("v1.0", "0.9.9"));
        assert!(!is_newer("v0.1.4", "0.1.4"));
        assert!(!is_newer("v0.1.3", "0.1.4"));
        assert!(!is_newer("nightly", "0.1.4"));
        assert!(!is_newer("v0.2.0-beta", "0.1.4"));
    }
}
