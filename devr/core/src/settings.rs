//! Settings and recent projects, kept in the DEVR+ config folder
//! (`~/.config/devr` on Linux, `%APPDATA%\devr` on Windows,
//! `~/Library/Application Support/devr` on macOS).

use std::path::{Path, PathBuf};

use anyhow::Result;
use serde_json::{Value, json};

fn dir() -> PathBuf {
    dirs::config_dir().unwrap_or_else(std::env::temp_dir).join("devr")
}

fn read_json(file: &str) -> Value {
    std::fs::read_to_string(dir().join(file))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or(Value::Null)
}

fn write_json(file: &str, value: &Value) -> Result<()> {
    std::fs::create_dir_all(dir())?;
    std::fs::write(dir().join(file), serde_json::to_string_pretty(value)?)?;
    Ok(())
}

pub fn load() -> Value {
    match read_json("settings.json") {
        Value::Object(map) => Value::Object(map),
        _ => json!({}),
    }
}

pub fn save(settings: &Value) -> Result<()> {
    write_json("settings.json", settings)
}

pub fn recent() -> Vec<PathBuf> {
    serde_json::from_value::<Vec<PathBuf>>(read_json("recent.json"))
        .unwrap_or_default()
        .into_iter()
        .filter(|p| p.is_dir())
        .collect()
}

pub fn add_recent(path: &Path) -> Result<()> {
    let mut list = recent();
    list.retain(|p| p != path);
    list.insert(0, path.to_owned());
    list.truncate(12);
    write_json("recent.json", &serde_json::to_value(list)?)
}
