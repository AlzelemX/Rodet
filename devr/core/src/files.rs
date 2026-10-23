//! Files and folders of the open project.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use notify::{RecursiveMode, Watcher as _};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::EventSink;

#[derive(Deserialize)]
pub struct PathParam {
    pub path: PathBuf,
}

#[derive(Deserialize)]
pub struct WriteParams {
    pub path: PathBuf,
    pub text: String,
}

#[derive(Deserialize)]
pub struct RenameParams {
    pub from: PathBuf,
    pub to: PathBuf,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

/// Folders that are never shown: build output and version control.
const HIDDEN: &[&str] = &[".git", "target", "node_modules", ".DS_Store"];

/// The entries of a folder: folders first, then files, by name.
pub fn read_dir(path: &Path) -> Result<Vec<Entry>> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(path).with_context(|| format!("can't open {}", path.display()))? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if HIDDEN.contains(&name.as_str()) {
            continue;
        }
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false)
            || (entry.file_type().map(|t| t.is_symlink()).unwrap_or(false) && entry.path().is_dir());
        entries.push(Entry { name, path: entry.path(), is_dir });
    }
    entries.sort_by(|a, b| {
        b.is_dir.cmp(&a.is_dir).then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    Ok(entries)
}

#[derive(Serialize)]
pub struct FileText {
    pub text: String,
    /// The file is not text (an image, a program, ...).
    pub binary: bool,
}

/// Files bigger than this are not opened in the editor.
const MAX_OPEN_SIZE: u64 = 20 * 1024 * 1024;

pub fn read_file(path: &Path) -> Result<FileText> {
    let size = fs::metadata(path).with_context(|| format!("can't open {}", path.display()))?.len();
    if size > MAX_OPEN_SIZE {
        bail!("{} is too big to open ({} MB)", path.display(), size / 1024 / 1024);
    }
    let bytes = fs::read(path)?;
    match String::from_utf8(bytes) {
        Ok(text) => Ok(FileText { text, binary: false }),
        Err(err) => {
            let bytes = err.into_bytes();
            if bytes.iter().take(8000).any(|&b| b == 0) {
                Ok(FileText { text: String::new(), binary: true })
            } else {
                Ok(FileText { text: String::from_utf8_lossy(&bytes).into_owned(), binary: false })
            }
        }
    }
}

pub fn write_file(path: &Path, text: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, text).with_context(|| format!("can't save {}", path.display()))
}

pub fn create_file(path: &Path) -> Result<()> {
    if path.exists() {
        bail!("{} already exists", path.display());
    }
    write_file(path, "")
}

pub fn create_dir(path: &Path) -> Result<()> {
    if path.exists() {
        bail!("{} already exists", path.display());
    }
    fs::create_dir_all(path).with_context(|| format!("can't create {}", path.display()))
}

pub fn rename(from: &Path, to: &Path) -> Result<()> {
    if to.exists() {
        bail!("{} already exists", to.display());
    }
    fs::rename(from, to).with_context(|| format!("can't rename {}", from.display()))
}

pub fn delete(path: &Path) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
    .with_context(|| format!("can't delete {}", path.display()))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    pub exists: bool,
    pub is_dir: bool,
    /// Milliseconds since 1970.
    pub modified: u64,
}

pub fn stat(path: &Path) -> Stat {
    let meta = fs::metadata(path).ok();
    Stat {
        exists: meta.is_some(),
        is_dir: meta.as_ref().is_some_and(|m| m.is_dir()),
        modified: meta
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map_or(0, |d| d.as_millis() as u64),
    }
}

/// Every file of the project (for "Go to File"), skipping ignored ones.
pub fn list_files(root: &Path) -> Vec<PathBuf> {
    const LIMIT: usize = 50_000;
    ignore::WalkBuilder::new(root)
        .hidden(false)
        .filter_entry(|e| !HIDDEN.contains(&e.file_name().to_string_lossy().as_ref()))
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_some_and(|t| t.is_file()))
        .map(|e| e.into_path())
        .take(LIMIT)
        .collect()
}

/// Watches the project folder and sends `fs.changed` events.
pub struct Watcher {
    sink: EventSink,
    current: Mutex<Option<notify::RecommendedWatcher>>,
}

impl Watcher {
    pub fn new(sink: EventSink) -> Watcher {
        Watcher { sink, current: Mutex::new(None) }
    }

    pub fn watch(&self, root: &Path) -> Result<()> {
        let sink = self.sink.clone();
        let (tx, rx) = std::sync::mpsc::channel::<notify::Event>();
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        })?;
        watcher.watch(root, RecursiveMode::Recursive)?;
        // Group the changes of 150 ms into one event.
        std::thread::spawn(move || {
            while let Ok(first) = rx.recv() {
                let mut paths: Vec<PathBuf> = first.paths;
                while let Ok(more) = rx.recv_timeout(Duration::from_millis(150)) {
                    paths.extend(more.paths);
                }
                paths.retain(|p| {
                    !p.components().any(|c| {
                        let c = c.as_os_str().to_string_lossy();
                        c == "target" || c == ".git"
                    })
                });
                paths.sort();
                paths.dedup();
                if !paths.is_empty() {
                    sink("fs.changed", json!({ "paths": paths }));
                }
            }
        });
        *self.current.lock().unwrap() = Some(watcher);
        Ok(())
    }
}
