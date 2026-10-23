//! Text search across the project ("Search" panel).

use std::path::PathBuf;

use anyhow::{Context, Result};
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchParams {
    pub root: PathBuf,
    pub query: String,
    #[serde(default)]
    pub regex: bool,
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default)]
    pub whole_word: bool,
    /// Only files whose path contains this text, e.g. `.rs`.
    #[serde(default)]
    pub include: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMatches {
    pub path: PathBuf,
    pub matches: Vec<Match>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    /// 1-based line.
    pub line: u32,
    /// 1-based columns (in characters) of the match on that line.
    pub start: u32,
    pub end: u32,
    /// The line, trimmed to at most 200 characters around the match.
    pub preview: String,
    /// Where the match starts in `preview` (characters).
    pub preview_start: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub files: Vec<FileMatches>,
    pub total: usize,
    /// More matches exist than were returned.
    pub truncated: bool,
}

const MAX_MATCHES: usize = 5000;
const MAX_FILE_SIZE: u64 = 4 * 1024 * 1024;

pub fn search(params: SearchParams) -> Result<SearchResult> {
    let pattern = if params.regex { params.query.clone() } else { regex::escape(&params.query) };
    let pattern = if params.whole_word { format!(r"\b(?:{pattern})\b") } else { pattern };
    let re = RegexBuilder::new(&pattern)
        .case_insensitive(!params.case_sensitive)
        .build()
        .context("the search is not a valid regular expression")?;

    let mut result = SearchResult { files: Vec::new(), total: 0, truncated: false };
    if params.query.is_empty() {
        return Ok(result);
    }
    let walker = ignore::WalkBuilder::new(&params.root)
        .hidden(false)
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            name != ".git" && name != "target" && name != "node_modules"
        })
        .build();
    for entry in walker.filter_map(|e| e.ok()) {
        if !entry.file_type().is_some_and(|t| t.is_file()) {
            continue;
        }
        let path = entry.path();
        if !params.include.is_empty() && !path.to_string_lossy().contains(&params.include) {
            continue;
        }
        if entry.metadata().map_or(true, |m| m.len() > MAX_FILE_SIZE) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let mut matches = Vec::new();
        for (index, line) in text.lines().enumerate() {
            for m in re.find_iter(line) {
                if result.total >= MAX_MATCHES {
                    result.truncated = true;
                    break;
                }
                result.total += 1;
                let start = line[..m.start()].chars().count();
                let len = line[m.start()..m.end()].chars().count();
                let (preview, preview_start) = preview(line, start);
                matches.push(Match {
                    line: index as u32 + 1,
                    start: start as u32 + 1,
                    end: (start + len) as u32 + 1,
                    preview,
                    preview_start: preview_start as u32,
                });
            }
        }
        if !matches.is_empty() {
            result.files.push(FileMatches { path: path.to_owned(), matches });
        }
        if result.truncated {
            break;
        }
    }
    Ok(result)
}

/// The line around character `at`, at most 200 characters, without leading spaces.
fn preview(line: &str, at: usize) -> (String, usize) {
    let chars: Vec<char> = line.chars().collect();
    let first = chars.iter().position(|c| !c.is_whitespace()).unwrap_or(0).min(at);
    let from = if at > first + 60 { at - 40 } else { first };
    let to = (from + 200).min(chars.len());
    (chars[from..to].iter().collect(), at - from)
}

#[cfg(test)]
mod tests {
    use super::{SearchParams, search};

    #[test]
    fn finds_matches_with_columns() {
        let dir = std::env::temp_dir().join(format!("devr-search-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.rs"), "fn main() {\n    out!(\"héllo\"); out!(\"x\");\n}\n").unwrap();
        let result = search(SearchParams {
            root: dir.clone(),
            query: "out!".into(),
            regex: false,
            case_sensitive: true,
            whole_word: false,
            include: ".rs".into(),
        })
        .unwrap();
        assert_eq!(result.total, 2);
        let m = &result.files[0].matches;
        assert_eq!((m[0].line, m[0].start, m[0].end), (2, 5, 9));
        assert_eq!((m[1].line, m[1].start), (2, 20));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
