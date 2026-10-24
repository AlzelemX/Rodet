//! DEVR+ plugins: folders with a `plugin.json`, kept in the DEVR+ config
//! folder (`<config>/devr/plugins/<id>/`, or `DEVR_PLUGINS_DIR`).
//!
//! The backend only lists, reads, installs, creates and removes them, and
//! keeps their saved data. Running them is the editor's job: a plugin is a
//! JavaScript module with the whole editor API (see `devr/plugin-api`).

use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// The API types for plugin authors, copied into every new plugin.
const API_TYPES: &str = include_str!("../../plugin-api/devr.d.ts");

pub fn dir() -> PathBuf {
    match std::env::var_os("DEVR_PLUGINS_DIR") {
        Some(dir) => PathBuf::from(dir),
        None => dirs::config_dir().unwrap_or_else(std::env::temp_dir).join("devr").join("plugins"),
    }
}

fn data_dir() -> PathBuf {
    dir().with_file_name("plugin-data")
}

#[derive(Deserialize)]
pub struct IdParam {
    pub id: String,
}

#[derive(Deserialize)]
pub struct ReadParams {
    pub id: String,
    pub file: String,
}

#[derive(Deserialize)]
pub struct InstallParams {
    pub from: PathBuf,
    #[serde(default)]
    pub replace: bool,
}

#[derive(Deserialize)]
pub struct CreateParams {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct DataParams {
    pub id: String,
    pub value: Value,
}

#[derive(Serialize)]
pub struct Plugin {
    pub id: String,
    pub path: PathBuf,
    /// The `plugin.json` as written (the editor reads the fields it knows).
    pub manifest: Value,
    /// Why the plugin can't be loaded, if it can't.
    pub error: Option<String>,
}

/// A plugin id: lowercase letters, digits, `-`, `_` and `.`.
pub fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && !id.starts_with('.')
        && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || "-_.".contains(c))
}

fn read_manifest(folder: &Path) -> Result<Value> {
    let path = folder.join("plugin.json");
    let text = fs::read_to_string(&path).with_context(|| format!("no plugin.json in {}", folder.display()))?;
    let manifest: Value = serde_json::from_str(&text).context("plugin.json is not valid JSON")?;
    let Some(id) = manifest.get("id").and_then(Value::as_str) else {
        bail!("plugin.json has no \"id\"");
    };
    if !valid_id(id) {
        bail!("plugin id `{id}` may only use a-z, 0-9, `-`, `_` and `.`");
    }
    Ok(manifest)
}

/// The installed plugins, by id.
pub fn list() -> Vec<Plugin> {
    let Ok(entries) = fs::read_dir(dir()) else { return Vec::new() };
    let mut plugins: Vec<Plugin> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| {
            let path = e.path();
            let folder = e.file_name().to_string_lossy().into_owned();
            match read_manifest(&path) {
                Ok(manifest) => {
                    let id = manifest["id"].as_str().unwrap_or_default().to_owned();
                    let error = (id != folder).then(|| {
                        format!("the folder is `{folder}` but plugin.json says id `{id}`")
                    });
                    Plugin { id, path, manifest, error }
                }
                Err(err) => Plugin { id: folder, path, manifest: json!({}), error: Some(format!("{err:#}")) },
            }
        })
        .collect();
    plugins.sort_by(|a, b| a.id.cmp(&b.id));
    plugins
}

/// `file` inside the plugin's folder (no `..`, no absolute paths).
fn plugin_file(id: &str, file: &str) -> Result<PathBuf> {
    if !valid_id(id) {
        bail!("bad plugin id `{id}`");
    }
    let rel = Path::new(file);
    if file.is_empty() || rel.components().any(|c| !matches!(c, Component::Normal(_))) {
        bail!("`{file}` is not a file inside the plugin");
    }
    Ok(dir().join(id).join(rel))
}

/// A text file of a plugin (its code, styles, themes…).
pub fn read(p: &ReadParams) -> Result<String> {
    let path = plugin_file(&p.id, &p.file)?;
    fs::read_to_string(&path).with_context(|| format!("can't read {}", path.display()))
}

/// Copies a plugin folder into the plugins folder. Returns its id.
pub fn install(p: &InstallParams) -> Result<String> {
    let manifest = read_manifest(&p.from)?;
    let id = manifest["id"].as_str().unwrap_or_default().to_owned();
    let target = dir().join(&id);
    let from = p.from.canonicalize().unwrap_or_else(|_| p.from.clone());
    if target.exists() {
        if target.canonicalize().ok().as_deref() == Some(from.as_path()) {
            return Ok(id); // Already in place.
        }
        if !p.replace {
            bail!("a plugin with the id `{id}` is already installed");
        }
        fs::remove_dir_all(&target)?;
    }
    copy_dir(&from, &target)?;
    Ok(id)
}

fn copy_dir(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let name = entry.file_name();
        if [".git", "node_modules", "target"].contains(&name.to_string_lossy().as_ref()) {
            continue;
        }
        let kind = entry.file_type()?;
        if kind.is_dir() {
            copy_dir(&entry.path(), &to.join(&name))?;
        } else if kind.is_file() {
            fs::copy(entry.path(), to.join(&name))?;
        }
    }
    Ok(())
}

pub fn remove(id: &str) -> Result<()> {
    if !valid_id(id) {
        bail!("bad plugin id `{id}`");
    }
    let path = dir().join(id);
    if path.exists() {
        fs::remove_dir_all(&path).with_context(|| format!("can't remove {}", path.display()))?;
    }
    let _ = fs::remove_file(data_dir().join(format!("{id}.json")));
    Ok(())
}

/// A new plugin from the template: a command, a status bar item, styles and
/// the API types. Returns its folder.
pub fn create(p: &CreateParams) -> Result<PathBuf> {
    if !valid_id(&p.id) {
        bail!("plugin id `{}` may only use a-z, 0-9, `-`, `_` and `.`", p.id);
    }
    let folder = dir().join(&p.id);
    if folder.exists() {
        bail!("a plugin with the id `{}` already exists", p.id);
    }
    fs::create_dir_all(&folder)?;
    let manifest = json!({
        "id": p.id,
        "name": p.name,
        "version": "0.1.0",
        "description": { "en": "My DEVR+ plugin.", "ar": "إضافتي لـ DEVR+." },
        "author": "",
        "main": "main.js",
        "styles": ["style.css"],
        "devr": "1",
    });
    fs::write(folder.join("plugin.json"), serde_json::to_string_pretty(&manifest)? + "\n")?;
    fs::write(folder.join("main.js"), template_main(&p.id, &p.name))?;
    fs::write(folder.join("style.css"), TEMPLATE_STYLE)?;
    fs::write(folder.join("devr.d.ts"), API_TYPES)?;
    Ok(folder)
}

fn template_main(id: &str, name: &str) -> String {
    let name = name.replace('\\', "\\\\").replace('\'', "\\'");
    TEMPLATE_MAIN.replace("__ID__", id).replace("__NAME__", &name)
}

const TEMPLATE_MAIN: &str = r#"// @ts-check
// A DEVR+ plugin. `activate` runs when the plugin starts; everything you add
// through `devr` is removed again when it stops or reloads.
// The whole API, with explanations: devr.d.ts (next to this file).

/** @param {import('./devr').Devr} devr */
export function activate(devr) {
  // Texts in English, with their Arabic: `devr.t()` picks the editor's language.
  devr.addTranslations({
    'Hello from {name}!': 'مرحبًا من {name}!',
    '{name}: Say Hello': '{name}: قل مرحبًا',
  });

  devr.commands.register({
    id: '__ID__.hello',
    title: devr.t('{name}: Say Hello', { name: '__NAME__' }),
    icon: 'smiley',
    key: 'Ctrl+Alt+H',
    run: () => devr.ui.toast(devr.t('Hello from {name}!', { name: '__NAME__' }), 'success'),
  });

  // A status bar item that shows the number of lines of the open file.
  const lines = devr.ui.addStatusItem({ icon: 'list-ordered', align: 'right', tooltip: 'Lines' });
  const update = () => {
    const model = devr.editor.model;
    lines.set(model ? String(model.getLineCount()) : '');
  };
  devr.editor.onChange(update);
  devr.workspace.onActive(update);
  update();
}

export function deactivate() {}
"#;

const TEMPLATE_STYLE: &str = "/* CSS for the whole editor, active while the plugin is on.\n   \
     Every color is a variable: try `:root { --accent: #2f9e44; }` */\n";

/// The data a plugin saved with `devr.storage.set`.
pub fn data_get(id: &str) -> Result<Value> {
    if !valid_id(id) {
        bail!("bad plugin id `{id}`");
    }
    Ok(fs::read_to_string(data_dir().join(format!("{id}.json")))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or(Value::Null))
}

pub fn data_set(p: &DataParams) -> Result<()> {
    if !valid_id(&p.id) {
        bail!("bad plugin id `{}`", p.id);
    }
    fs::create_dir_all(data_dir())?;
    fs::write(data_dir().join(format!("{}.json", p.id)), serde_json::to_string_pretty(&p.value)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // One test: the plugins folder comes from an environment variable.
    #[test]
    fn plugins_folder() {
        let root = std::env::temp_dir().join(format!("devr-plugins-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        // SAFETY: the only test in this crate that reads or sets this variable.
        unsafe { std::env::set_var("DEVR_PLUGINS_DIR", root.join("plugins")) };

        assert!(list().is_empty());
        let folder = create(&CreateParams { id: "hello".into(), name: "Hello".into() }).unwrap();
        assert!(folder.join("devr.d.ts").exists());
        let plugins = list();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].id, "hello");
        assert!(plugins[0].error.is_none());
        assert_eq!(plugins[0].manifest["main"], "main.js");
        assert!(read(&ReadParams { id: "hello".into(), file: "main.js".into() }).unwrap().contains("hello.hello"));

        // Only files inside the plugin can be read.
        for bad in ["../hello/main.js", "/etc/passwd", "", "a/../../x"] {
            assert!(read(&ReadParams { id: "hello".into(), file: bad.into() }).is_err(), "{bad}");
        }
        assert!(read(&ReadParams { id: "../x".into(), file: "main.js".into() }).is_err());
        assert!(create(&CreateParams { id: "Bad Id".into(), name: "x".into() }).is_err());
        assert!(create(&CreateParams { id: "hello".into(), name: "again".into() }).is_err());

        // Install from another folder; a second install needs `replace`.
        let source = root.join("source");
        fs::create_dir_all(source.join("themes")).unwrap();
        fs::write(source.join("plugin.json"), r#"{"id": "ocean", "name": "Ocean"}"#).unwrap();
        fs::write(source.join("themes/ocean.json"), "{}").unwrap();
        let install_params = |replace| InstallParams { from: source.clone(), replace };
        assert_eq!(install(&install_params(false)).unwrap(), "ocean");
        assert!(install(&install_params(false)).is_err());
        assert_eq!(install(&install_params(true)).unwrap(), "ocean");
        assert_eq!(read(&ReadParams { id: "ocean".into(), file: "themes/ocean.json".into() }).unwrap(), "{}");

        // A folder whose plugin.json is broken is listed with the reason.
        fs::create_dir_all(dir().join("broken")).unwrap();
        fs::write(dir().join("broken/plugin.json"), "{").unwrap();
        let broken = list().into_iter().find(|p| p.id == "broken").unwrap();
        assert!(broken.error.unwrap().contains("not valid JSON"));

        // Saved data, and removing a plugin removes it too.
        data_set(&DataParams { id: "hello".into(), value: json!({"count": 3}) }).unwrap();
        assert_eq!(data_get("hello").unwrap()["count"], 3);
        remove("hello").unwrap();
        assert_eq!(data_get("hello").unwrap(), Value::Null);
        assert_eq!(list().iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), ["broken", "ocean"]);

        let _ = fs::remove_dir_all(&root);
    }
}
