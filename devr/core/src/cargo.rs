//! Cargo projects: what is in `Cargo.toml`, new projects, crate versions.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::toolchain::{command, find_program};

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// The folder with `Cargo.toml`, if this is a Cargo project.
    pub root: Option<PathBuf>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub edition: Option<String>,
    /// Programs that `cargo run --bin <name>` can run.
    pub bins: Vec<String>,
    pub examples: Vec<String>,
    pub has_lib: bool,
    pub dependencies: Vec<Dependency>,
    /// Members of a workspace.
    pub members: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub name: String,
    /// The version asked for, e.g. `1.0` (empty for path or git dependencies).
    pub version: String,
    /// `dependencies`, `dev-dependencies` or `build-dependencies`.
    pub kind: String,
    /// `path = ...` or `git = ...` dependencies.
    pub source: Option<String>,
}

/// Reads `Cargo.toml` in `root` (no network, no `cargo` needed).
pub fn project(root: &Path) -> Result<Project> {
    let manifest = root.join("Cargo.toml");
    if !manifest.is_file() {
        return Ok(Project::default());
    }
    let text = std::fs::read_to_string(&manifest)?;
    let toml: toml::Table = text.parse().context("Cargo.toml is not valid TOML")?;
    let mut project = Project { root: Some(root.to_owned()), ..Default::default() };

    if let Some(package) = toml.get("package").and_then(|p| p.as_table()) {
        project.name = package.get("name").and_then(|v| v.as_str()).map(str::to_owned);
        project.version = package.get("version").and_then(|v| v.as_str()).map(str::to_owned);
        project.edition = package.get("edition").and_then(|v| v.as_str()).map(str::to_owned);
    }

    // Programs: [[bin]] entries, src/main.rs, src/bin/*.rs.
    if let Some(bins) = toml.get("bin").and_then(|b| b.as_array()) {
        for bin in bins {
            if let Some(name) = bin.get("name").and_then(|n| n.as_str()) {
                project.bins.push(name.to_owned());
            }
        }
    }
    if root.join("src/main.rs").is_file()
        && let Some(name) = &project.name
        && !project.bins.contains(name)
    {
        project.bins.insert(0, name.clone());
    }
    for (dir, list) in [("src/bin", &mut project.bins), ("examples", &mut project.examples)] {
        if let Ok(entries) = std::fs::read_dir(root.join(dir)) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = if path.extension().is_some_and(|e| e == "rs") {
                    path.file_stem().map(|s| s.to_string_lossy().into_owned())
                } else if path.join("main.rs").is_file() {
                    path.file_name().map(|s| s.to_string_lossy().into_owned())
                } else {
                    None
                };
                if let Some(name) = name
                    && !list.contains(&name)
                {
                    list.push(name);
                }
            }
        }
    }
    project.has_lib = toml.contains_key("lib") || root.join("src/lib.rs").is_file();

    for kind in ["dependencies", "dev-dependencies", "build-dependencies"] {
        let Some(deps) = toml.get(kind).and_then(|d| d.as_table()) else { continue };
        for (name, spec) in deps {
            let (version, source) = match spec {
                toml::Value::String(v) => (v.clone(), None),
                toml::Value::Table(t) => (
                    t.get("version").and_then(|v| v.as_str()).unwrap_or("").to_owned(),
                    t.get("path")
                        .or_else(|| t.get("git"))
                        .and_then(|v| v.as_str())
                        .map(str::to_owned),
                ),
                _ => (String::new(), None),
            };
            project.dependencies.push(Dependency { name: name.clone(), version, kind: kind.to_owned(), source });
        }
    }

    if let Some(members) = toml
        .get("workspace")
        .and_then(|w| w.get("members"))
        .and_then(|m| m.as_array())
    {
        project.members = members.iter().filter_map(|m| m.as_str().map(str::to_owned)).collect();
    }
    Ok(project)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewParams {
    /// The folder to create the project in.
    pub parent: PathBuf,
    pub name: String,
    /// A library instead of a program.
    #[serde(default)]
    pub lib: bool,
}

/// `cargo new`, with a Rust++ hello world.
pub fn new_project(params: &NewParams) -> Result<PathBuf> {
    let name = params.name.trim();
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        bail!("use letters, numbers, `-` and `_` for the project name");
    }
    let path = params.parent.join(name);
    if path.exists() {
        bail!("{} already exists", path.display());
    }
    let cargo = find_program("cargo").context("cargo was not found: install Rust++ first")?;
    let output = command(cargo)
        .arg("new")
        .arg(if params.lib { "--lib" } else { "--bin" })
        .arg(&path)
        .output()
        .context("can't run cargo")?;
    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr).trim());
    }
    if !params.lib {
        std::fs::write(
            path.join("src/main.rs"),
            "fn main() {\n    out!(\"Hello from Rust++!\");\n}\n",
        )?;
    }
    Ok(path)
}

#[derive(Deserialize)]
pub struct CrateParam {
    pub name: String,
}

/// The newest version of a crate on crates.io (not a pre-release, not yanked).
pub fn latest_version(name: &str) -> Result<Option<String>> {
    let lower = name.to_lowercase();
    let prefix = match lower.len() {
        1 => "1".to_owned(),
        2 => "2".to_owned(),
        3 => format!("3/{}", &lower[..1]),
        _ => format!("{}/{}", &lower[..2], &lower[2..4]),
    };
    let url = format!("https://index.crates.io/{prefix}/{lower}");
    let text = ureq::get(&url)
        .set("User-Agent", "DEVR+ (https://github.com/AlzelemX/Rust-)")
        .call()
        .with_context(|| format!("can't reach crates.io for {name}"))?
        .into_string()?;
    let mut latest: Option<(Vec<u64>, String)> = None;
    for line in text.lines() {
        #[derive(Deserialize)]
        struct Release {
            vers: String,
            #[serde(default)]
            yanked: bool,
        }
        let Ok(release) = serde_json::from_str::<Release>(line) else { continue };
        if release.yanked || release.vers.contains('-') {
            continue;
        }
        let key: Vec<u64> = release.vers.split('.').map(|p| p.parse().unwrap_or(0)).collect();
        if latest.as_ref().is_none_or(|(best, _)| key > *best) {
            latest = Some((key, release.vers));
        }
    }
    Ok(latest.map(|(_, version)| version))
}

#[cfg(test)]
mod tests {
    use super::project;

    #[test]
    fn reads_cargo_toml() {
        let dir = std::env::temp_dir().join(format!("devr-cargo-test-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("src/bin")).unwrap();
        std::fs::write(
            dir.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.2.0\"\nedition = \"2024\"\n\n[dependencies]\nrand = \"0.8\"\nlocal = { path = \"../local\" }\n\n[dev-dependencies]\nserde = { version = \"1\", features = [\"derive\"] }\n",
        )
        .unwrap();
        std::fs::write(dir.join("src/main.rs"), "fn main() {}").unwrap();
        std::fs::write(dir.join("src/bin/tool.rs"), "fn main() {}").unwrap();
        let p = project(&dir).unwrap();
        assert_eq!(p.name.as_deref(), Some("demo"));
        assert_eq!(p.bins, ["demo", "tool"]);
        assert_eq!(p.dependencies.len(), 3);
        let rand = p.dependencies.iter().find(|d| d.name == "rand").unwrap();
        assert_eq!(rand.version, "0.8");
        let local = p.dependencies.iter().find(|d| d.name == "local").unwrap();
        assert_eq!(local.source.as_deref(), Some("../local"));
        let serde = p.dependencies.iter().find(|d| d.name == "serde").unwrap();
        assert_eq!(serde.kind, "dev-dependencies");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
