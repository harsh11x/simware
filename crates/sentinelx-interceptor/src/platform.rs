//! Platform-specific execution hook adapters.
//!
//! - Linux: fanotify / eBPF / inotify fallback
//! - macOS: Endpoint Security framework
//! - Windows: minifilter / AppLocker integration

use sentinelx_core::Platform;

pub fn platform_hooks_available() -> bool {
    match sentinelx_core::detect_platform() {
        Platform::Linux => cfg!(feature = "linux-hooks"),
        Platform::MacOS => cfg!(feature = "macos-hooks"),
        Platform::Windows => cfg!(feature = "windows-hooks"),
        Platform::Unknown => false,
    }
}

pub struct HookConfig {
    pub monitor_paths: Vec<String>,
    pub file_extensions: Vec<String>,
    pub block_on_critical: bool,
}

impl Default for HookConfig {
    fn default() -> Self {
        Self {
            monitor_paths: vec!["/home".into(), "/tmp".into()],
            file_extensions: vec![
                "exe".into(), "msi".into", "sh".into(), "ps1".into(), "bat".into(),
                "py".into(), "jar".into(), "docm".into(), "pdf".into(),
            ],
            block_on_critical: true,
        }
    }
}
