//! The Doc Book that comes with the installed Rodet.
//!
//! Every Rodet release carries its Doc Book in
//! `<sysroot>/share/doc/rodet/book/<language>/` (`en`, `ar`): the Markdown
//! chapters and their images. The editor shows the book of the Rodet that is
//! installed, so it always matches the language the code is compiled with.

use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::toolchain::{command, find_program};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookInfo {
    /// The folder with one sub-folder per language, if this Rodet has a book.
    pub root: Option<PathBuf>,
    /// The languages available, e.g. `["ar", "en"]`.
    pub languages: Vec<String>,
}

/// `DEVR_BOOK_DIR` (for development), else the book of the installed Rodet.
fn book_root() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("DEVR_BOOK_DIR") {
        return Some(PathBuf::from(dir));
    }
    let rodetc = find_program("rodetc")?;
    let out = command(rodetc).args(["--print", "sysroot"]).output().ok()?;
    let sysroot = PathBuf::from(String::from_utf8_lossy(&out.stdout).trim());
    let root = sysroot.join("share").join("doc").join("rodet").join("book");
    root.is_dir().then_some(root)
}

pub fn info() -> BookInfo {
    let root = book_root();
    let mut languages: Vec<String> = root
        .as_ref()
        .and_then(|r| std::fs::read_dir(r).ok())
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.path().join("SUMMARY.md").is_file())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    languages.sort();
    BookInfo { root: if languages.is_empty() { None } else { root }, languages }
}

#[derive(Deserialize)]
pub struct FileParams {
    /// `en` or `ar`.
    pub language: String,
    /// A path inside the book, like `basics/functions.md`.
    pub path: String,
}

/// The file `params.path` of the book, refusing paths that leave it.
fn book_file(params: &FileParams) -> Result<PathBuf> {
    let root = book_root().context("this Rodet has no Doc Book: update Rodet")?;
    if !params.language.chars().all(|c| c.is_ascii_alphabetic() || c == '-') {
        bail!("unknown language");
    }
    let relative = Path::new(&params.path);
    if relative.components().any(|c| !matches!(c, Component::Normal(_))) {
        bail!("not a page of the book: {}", params.path);
    }
    Ok(root.join(&params.language).join(relative))
}

pub fn read(params: &FileParams) -> Result<String> {
    let path = book_file(params)?;
    std::fs::read_to_string(&path).with_context(|| format!("no page {}", params.path))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    /// A `data:` URL for an `<img>`.
    pub data_url: String,
}

/// An image of the book, as a `data:` URL.
pub fn asset(params: &FileParams) -> Result<Asset> {
    let path = book_file(params)?;
    let mime = match path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        _ => bail!("not an image: {}", params.path),
    };
    let bytes = std::fs::read(&path).with_context(|| format!("no image {}", params.path))?;
    Ok(Asset { data_url: format!("data:{mime};base64,{}", base64(&bytes)) })
}

fn base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = (chunk[0] as u32) << 16 | (*chunk.get(1).unwrap_or(&0) as u32) << 8 | *chunk.get(2).unwrap_or(&0) as u32;
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(TABLE[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{FileParams, base64, book_file};

    #[test]
    fn encodes_base64() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn stays_inside_the_book() {
        // SAFETY: tests in this module don't run other code that reads the variable.
        unsafe { std::env::set_var("DEVR_BOOK_DIR", std::env::temp_dir()) };
        let p = |path: &str| FileParams { language: "en".into(), path: path.into() };
        assert!(book_file(&p("basics/functions.md")).is_ok());
        assert!(book_file(&p("../secret")).is_err());
        assert!(book_file(&p("/etc/passwd")).is_err());
        assert!(book_file(&FileParams { language: "../x".into(), path: "a.md".into() }).is_err());
    }
}
