//! Finds the Rust++ toolchain: `rustc`, `cargo` and `rust++-analyzer`.
//!
//! Rust++ installs into `~/.rustpp/bin` (`%USERPROFILE%\.rustpp\bin` on
//! Windows). That folder is searched first, then the PATH. Every process
//! DEVR+ starts gets that folder first in its PATH, so `cargo` and `rustc`
//! are always Rust++.

use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Toolchain {
    /// `~/.rustpp/bin`, if Rust++ is installed there.
    pub rustpp_bin: Option<PathBuf>,
    pub rustc: Option<PathBuf>,
    pub cargo: Option<PathBuf>,
    pub analyzer: Option<PathBuf>,
    /// `rustc --version`, e.g. `rustc 1.101.0-dev (Rust++)`.
    pub rustc_version: Option<String>,
    /// Whether `rustc` is Rust++ (and not the official Rust).
    pub is_rustpp: bool,
}

impl Toolchain {
    pub fn find() -> Toolchain {
        let rustc = find_program("rustc");
        let rustc_version = rustc.as_ref().and_then(|rustc| {
            let out = command(rustc).arg("--version").output().ok()?;
            Some(String::from_utf8_lossy(&out.stdout).trim().to_owned())
        });
        Toolchain {
            rustpp_bin: rustpp_bin(),
            is_rustpp: rustc_version.as_deref().is_some_and(|v| v.contains("Rust++")),
            rustc,
            rustc_version,
            cargo: find_program("cargo"),
            analyzer: find_program("rust++-analyzer").or_else(|| find_program("rust-analyzer")),
        }
    }
}

/// `~/.rustpp/bin`, if it exists.
pub fn rustpp_bin() -> Option<PathBuf> {
    let bin = dirs::home_dir()?.join(".rustpp").join("bin");
    bin.is_dir().then_some(bin)
}

/// The language for messages of programs DEVR+ starts (`RUSTPP_LANG`): the
/// compiler, Cargo and rust++-analyzer speak Arabic when the editor does.
pub fn language() -> &'static str {
    match crate::settings::load().get("language").and_then(|v| v.as_str()) {
        Some("ar") => "ar",
        _ => "en",
    }
}

/// The PATH for processes DEVR+ starts: Rust++ first.
pub fn search_path() -> OsString {
    let mut paths: Vec<PathBuf> = rustpp_bin().into_iter().collect();
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

/// Finds `name` (without `.exe`) in the Rust++ folder or the PATH.
pub fn find_program(name: &str) -> Option<PathBuf> {
    let file = format!("{name}{}", env::consts::EXE_SUFFIX);
    env::split_paths(&search_path()).map(|dir| dir.join(&file)).find(|path| path.is_file())
}

/// A command with the Rust++ PATH, and no console window on Windows.
pub fn command(program: impl AsRef<Path>) -> Command {
    let mut cmd = Command::new(program.as_ref());
    cmd.env("PATH", search_path()).env("RUSTPP_LANG", language());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}
