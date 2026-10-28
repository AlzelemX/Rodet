use crate::util::errors::CargoResult;
use cargo_util::paths;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// The manifest of a Rodet package.
pub const MANIFEST_NAME: &str = "Silk.toml";
/// The manifest of packages from crates.io, still read by Silk.
pub const CARGO_MANIFEST_NAME: &str = "Cargo.toml";

/// The manifest in `dir`: `Silk.toml`, or else `Cargo.toml` (crates.io packages).
pub fn manifest_in(dir: &Path) -> PathBuf {
    let silk = dir.join(MANIFEST_NAME);
    if silk.exists() {
        silk
    } else {
        let cargo = dir.join(CARGO_MANIFEST_NAME);
        if cargo.exists() { cargo } else { silk }
    }
}

/// Whether `name` is a manifest's file name (`Silk.toml` or `Cargo.toml`).
pub fn is_manifest_name(name: Option<&OsStr>) -> bool {
    name == Some(OsStr::new(MANIFEST_NAME)) || name == Some(OsStr::new(CARGO_MANIFEST_NAME))
}

/// Finds the root `Silk.toml` (or `Cargo.toml`).
pub fn find_root_manifest_for_wd(cwd: &Path) -> CargoResult<PathBuf> {
    let valid_cargo_toml_file_name = MANIFEST_NAME;
    let invalid_names = ["silk.toml", "cargo.toml"];
    let mut invalid_cargo_toml_path_exists = false;

    for current in paths::ancestors(cwd, None) {
        for name in [MANIFEST_NAME, CARGO_MANIFEST_NAME] {
            let manifest = current.join(name);
            if manifest.exists() {
                return Ok(manifest);
            }
        }
        if invalid_names.iter().any(|name| current.join(name).exists()) {
            invalid_cargo_toml_path_exists = true;
        }
    }

    if invalid_cargo_toml_path_exists {
        anyhow::bail!(
            "could not find `{}` in `{}` or any parent directory, but found silk.toml please try to rename it to Silk.toml",
            valid_cargo_toml_file_name,
            cwd.display()
        )
    } else {
        anyhow::bail!(
            "could not find `{}` in `{}` or any parent directory",
            valid_cargo_toml_file_name,
            cwd.display()
        )
    }
}

/// Returns the path to the `file` in `pwd`, if it exists.
pub fn find_project_manifest_exact(pwd: &Path, file: &str) -> CargoResult<PathBuf> {
    let manifest = pwd.join(file);

    if manifest.exists() {
        Ok(manifest)
    } else {
        anyhow::bail!("Could not find `{}` in `{}`", file, pwd.display())
    }
}
