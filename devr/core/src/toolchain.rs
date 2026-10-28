//! Finds the Rodet toolchain: `rustc`, `cargo` and `rodet-analyzer`.
//!
//! Rodet installs into `~/.rodet/bin` (`%USERPROFILE%\.rodet\bin` on
//! Windows). That folder is searched first, then the PATH. Every process
//! DEVR+ starts gets that folder first in its PATH, so `cargo` and `rustc`
//! are always Rodet.

use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Toolchain {
    /// `~/.rodet/bin`, if Rodet is installed there.
    pub rodet_bin: Option<PathBuf>,
    pub rustc: Option<PathBuf>,
    pub cargo: Option<PathBuf>,
    pub analyzer: Option<PathBuf>,
    /// `rustc --version`, e.g. `rustc 1.101.0-dev (Rodet)`.
    pub rustc_version: Option<String>,
    /// Whether `rustc` is Rodet (and not the official Rust).
    pub is_rodet: bool,
}

impl Toolchain {
    pub fn find() -> Toolchain {
        let rustc = find_program("rustc");
        let rustc_version = rustc.as_ref().and_then(|rustc| {
            let out = command(rustc).arg("--version").output().ok()?;
            Some(String::from_utf8_lossy(&out.stdout).trim().to_owned())
        });
        Toolchain {
            rodet_bin: rodet_bin(),
            is_rodet: rustc_version.as_deref().is_some_and(|v| v.contains("Rodet")),
            rustc,
            rustc_version,
            cargo: find_program("cargo"),
            analyzer: find_program("rodet-analyzer").or_else(|| find_program("rust-analyzer")),
        }
    }
}

/// `~/.rodet/bin`, if it exists.
pub fn rodet_bin() -> Option<PathBuf> {
    let bin = dirs::home_dir()?.join(".rodet").join("bin");
    bin.is_dir().then_some(bin)
}

/// The language for messages of programs DEVR+ starts (`RODET_LANG`): the
/// compiler, Cargo and rodet-analyzer speak Arabic when the editor does.
pub fn language() -> &'static str {
    match crate::settings::load().get("language").and_then(|v| v.as_str()) {
        Some("ar") => "ar",
        _ => "en",
    }
}

/// The PATH for processes DEVR+ starts: Rodet first.
pub fn search_path() -> OsString {
    let mut paths: Vec<PathBuf> = rodet_bin().into_iter().collect();
    if let Some(path) = env::var_os("PATH") {
        paths.extend(env::split_paths(&path));
    }
    if let Some(home) = dirs::home_dir() {
        // Desktop apps on macOS and Linux often start without the shell's PATH.
        for extra in [".cargo/bin", ".local/bin"] {
            paths.push(home.join(extra));
        }
    }
    for extra in ["/usr/local/bin", "/opt/homebrew/bin", "/usr/bin", "/bin"] {
        paths.push(PathBuf::from(extra));
    }
    env::join_paths(paths).unwrap_or_default()
}

/// Finds `name` (without `.exe`) in the Rodet folder or the PATH.
pub fn find_program(name: &str) -> Option<PathBuf> {
    let file = format!("{name}{}", env::consts::EXE_SUFFIX);
    env::split_paths(&search_path()).map(|dir| dir.join(&file)).find(|path| path.is_file())
}

/// A command with the Rodet PATH, and no console window on Windows.
pub fn command(program: impl AsRef<Path>) -> Command {
    let mut cmd = Command::new(program.as_ref());
    cmd.env("PATH", search_path()).env("RODET_LANG", language());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}
