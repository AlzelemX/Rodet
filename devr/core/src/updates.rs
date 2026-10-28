//! New versions of Rodet and DEVR+ on GitHub.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::toolchain::Toolchain;

const REPO: &str = "AlzelemX/rodet";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Updates {
    /// The newest Rodet release, e.g. `v1.101.0-dev-rodet.18`.
    pub rodet_latest: Option<Release>,
    /// The newest DEVR+ release, e.g. `devr-v0.1.3`.
    pub devr_latest: Option<Release>,
    pub devr_current: String,
    /// `rustc --version` of the installed Rodet.
    pub rustc_version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    #[serde(rename(deserialize = "tag_name"))]
    pub tag: String,
    #[serde(rename(deserialize = "html_url"))]
    pub url: String,
    #[serde(rename(deserialize = "published_at"), default)]
    pub published: Option<String>,
}

pub fn check() -> Result<Updates> {
    let releases: Vec<Release> = ureq::get(&format!("https://api.github.com/repos/{REPO}/releases?per_page=40"))
        .set("User-Agent", "DEVR+")
        .set("Accept", "application/vnd.github+json")
        .call()
        .context("can't reach GitHub")?
        .into_json()?;
    Ok(Updates {
        rodet_latest: releases.iter().find(|r| r.tag.contains("rodet")).cloned(),
        devr_latest: releases.iter().find(|r| r.tag.starts_with("devr-")).cloned(),
        devr_current: crate::VERSION.to_owned(),
        rustc_version: Toolchain::find().rustc_version,
    })
}
