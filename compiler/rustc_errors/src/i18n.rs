//! RODET: the compiler's messages in other languages.
//!
//! Every message is written in English in the compiler's source. The
//! translations are kept next to this file, one per language (`i18n/ar.txt`),
//! keyed by the exact English text:
//!
//! ```text
//! en: cannot find value `{$name}` in this scope
//! ar: لا يوجد متغير أو قيمة اسمها `{$name}` هنا
//! ```
//!
//! Releases install them in `<sysroot>/share/rodet/i18n/`, where the
//! compiler reads them at run time (so they can grow without rebuilding the
//! compiler); `RODET_I18N_DIR` points elsewhere for development.
//! The language comes from `RODET_LANG` (`ar`, or `en`, the default).
//! Messages without a translation stay in English.
//! `rodet/i18n/compiler_messages.py` lists what is still missing.

use std::sync::OnceLock;

use rustc_data_structures::fx::FxHashMap;

struct Table {
    /// English message (Fluent template or plain text) → translation.
    exact: FxHashMap<&'static str, &'static str>,
    /// Messages built with `format!`: their English template split at the
    /// `{…}` holes, and the translation (`{0}`, `{1}`… or `{}` in order).
    patterns: Vec<(Vec<&'static str>, &'static str)>,
}

fn table() -> Option<&'static Table> {
    static TABLE: OnceLock<Option<Table>> = OnceLock::new();
    TABLE
        .get_or_init(|| {
            let language = std::env::var("RODET_LANG").ok()?.to_ascii_lowercase();
            let code = match language.as_str() {
                "ar" | "arabic" | "ar_sa" | "ar-sa" | "ar_sa.utf-8" => "ar",
                _ => return None,
            };
            let source = std::fs::read_to_string(table_dir()?.join(format!("{code}.txt"))).ok()?;
            Some(parse(Box::leak(source.into_boxed_str())))
        })
        .as_ref()
}

/// `RODET_I18N_DIR`, else `<sysroot>/share/rodet/i18n` next to this compiler.
fn table_dir() -> Option<std::path::PathBuf> {
    if let Some(dir) = std::env::var_os("RODET_I18N_DIR") {
        return Some(dir.into());
    }
    let exe = std::env::current_exe().ok()?;
    let sysroot = exe.parent()?.parent()?;
    Some(sysroot.join("share").join("rodet").join("i18n"))
}

fn parse(source: &'static str) -> Table {
    let mut exact = FxHashMap::default();
    let mut patterns = Vec::new();
    let mut english: Option<&'static str> = None;
    for line in source.lines() {
        if let Some(en) = line.strip_prefix("en: ") {
            english = Some(en);
        } else if let Some(ar) = line.strip_prefix("ar: ")
            && let Some(en) = english.take()
        {
            let en = unescape(en);
            let ar = unescape(ar);
            // A `format!` template: holes are `{}`, `{name}` or `{0}`, not Fluent's
            // `{$name}` / `{ $name -> … }`. Templates with too little fixed text
            // would match unrelated messages, so they are only used exactly.
            if en.contains('{') && !is_fluent(en) {
                let parts = split_holes(en);
                if parts.len() > 1 && parts.iter().map(|p| p.trim().len()).sum::<usize>() >= 8 {
                    patterns.push((parts, ar));
                }
            }
            exact.insert(en, ar);
        }
    }
    // The most specific templates (most fixed text) are tried first.
    patterns.sort_by_key(|(parts, _)| std::cmp::Reverse(parts.iter().map(|p| p.len()).sum::<usize>()));
    Table { exact, patterns }
}

/// Fluent placeables start with `$` after the brace: `{$x}`, `{ $x -> … }`.
fn is_fluent(template: &str) -> bool {
    template.match_indices('{').any(|(i, _)| template[i + 1..].trim_start().starts_with('$'))
}

/// `\n` and `\\` in the table file; leaked once per message, for the
/// lifetime of the compiler process.
fn unescape(s: &'static str) -> &'static str {
    if !s.contains('\\') {
        return s;
    }
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some(other) => out.push(other),
                None => {}
            }
        } else {
            out.push(c);
        }
    }
    Box::leak(out.into_boxed_str())
}

/// `cannot find {} `{}` in {}` → `["cannot find ", " `", "` in ", ""]`.
fn split_holes(template: &'static str) -> Vec<&'static str> {
    let mut parts = Vec::new();
    let mut rest = template;
    loop {
        match rest.find('{') {
            // `{{` is a literal brace in `format!`.
            Some(i) if rest[i..].starts_with("{{") => {
                // Keep it simple: such templates are matched only exactly.
                return vec![template];
            }
            Some(i) => {
                parts.push(&rest[..i]);
                match rest[i..].find('}') {
                    Some(j) => rest = &rest[i + j + 1..],
                    None => return vec![template],
                }
            }
            None => {
                parts.push(rest);
                return parts;
            }
        }
    }
}

/// The values in `text` for the holes of a template, if it matches.
fn match_holes<'t>(parts: &[&str], text: &'t str) -> Option<Vec<&'t str>> {
    if parts.len() < 2 {
        return None;
    }
    let mut rest = text.strip_prefix(parts[0])?;
    let mut values = Vec::with_capacity(parts.len() - 1);
    for (k, part) in parts[1..].iter().enumerate() {
        let last = k == parts.len() - 2;
        let at = if last {
            if !rest.ends_with(part) {
                return None;
            }
            rest.len() - part.len()
        } else if part.is_empty() {
            // Two holes side by side (`{}{}`): the first gets nothing, the
            // next one everything up to the following text.
            values.push("");
            continue;
        } else {
            rest.find(part)?
        };
        values.push(&rest[..at]);
        rest = &rest[at + part.len()..];
    }
    Some(values)
}

/// Fills `{0}`, `{1}`… (or `{}` in order) in a translated template.
fn fill(template: &str, values: &[&str]) -> String {
    let mut out = String::with_capacity(template.len() + 16);
    let mut next = 0;
    let mut rest = template;
    while let Some(i) = rest.find('{') {
        out.push_str(&rest[..i]);
        let Some(j) = rest[i..].find('}') else { break };
        let inner = &rest[i + 1..i + j];
        let index = if inner.is_empty() {
            next += 1;
            Some(next - 1)
        } else {
            inner.parse::<usize>().ok()
        };
        match index.and_then(|n| values.get(n)) {
            Some(v) => out.push_str(v),
            None => out.push_str(&rest[i..i + j + 1]),
        }
        rest = &rest[i + j + 1..];
    }
    out.push_str(rest);
    out
}

/// The translation of a Fluent template (`DiagMessage::Inline`).
pub(crate) fn template(english: &str) -> Option<&'static str> {
    table()?.exact.get(english).copied()
}

/// The translation of an already formatted message (`DiagMessage::Str`).
pub(crate) fn text(english: &str) -> Option<String> {
    let table = table()?;
    if let Some(t) = table.exact.get(english) {
        return Some((*t).to_owned());
    }
    // Multi-line messages: translate line by line, keep what has no translation.
    if english.contains('\n') {
        let mut changed = false;
        let lines: Vec<String> = english
            .split('\n')
            .map(|line| match text_line(table, line) {
                Some(t) => {
                    changed = true;
                    t
                }
                None => line.to_owned(),
            })
            .collect();
        return changed.then(|| lines.join("\n"));
    }
    if let Some(t) = text_line(table, english) {
        return Some(t);
    }
    // "aborting due to 2 previous errors; 1 warning emitted"
    if english.contains("; ") {
        let parts: Option<Vec<String>> = english.split("; ").map(|p| text_line(table, p)).collect();
        return parts.map(|p| p.join("؛ "));
    }
    None
}

fn text_line(table: &Table, english: &str) -> Option<String> {
    if let Some(t) = table.exact.get(english) {
        return Some((*t).to_owned());
    }
    // The first template that leaves no English words behind wins. Short,
    // general templates ("cannot {} {} in {}") would otherwise turn a message
    // into half Arabic, half English; a long template is still worth using
    // when a hole keeps an English word.
    let mut fallback = None;
    for (parts, translation) in &table.patterns {
        let Some(values) = match_holes(parts, english) else { continue };
        let mut clean = true;
        let values: Vec<String> = values
            .iter()
            .enumerate()
            .map(|(k, v)| {
                if in_code(parts, k) {
                    return (*v).to_owned();
                }
                let t = fragment_in(table, v).unwrap_or_else(|| (*v).to_owned());
                // A value the translation leaves out (an English plural `s`,
                // "was"/"were"…) does not count.
                let used = translation.contains("{}") || translation.contains(&format!("{{{k}}}"));
                clean &= !(used && has_english(&t));
                t
            })
            .collect();
        let values: Vec<&str> = values.iter().map(|v| v.as_str()).collect();
        if clean {
            return Some(fill(translation, &values));
        }
        if fallback.is_none() && parts.iter().map(|p| p.len()).sum::<usize>() >= 30 {
            fallback = Some(fill(translation, &values));
        }
    }
    fallback
}

/// Whether hole `k` sits between backticks (code, never translated).
fn in_code(parts: &[&str], k: usize) -> bool {
    parts[k].ends_with('`') && parts.get(k + 1).is_some_and(|p| p.starts_with('`'))
}

/// Whether `s` still has an English word outside of `code`.
fn has_english(s: &str) -> bool {
    s.split('`').step_by(2).any(|plain| {
        plain.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '\'')).any(|w| {
            w.len() >= 2 && w.bytes().all(|b| b.is_ascii_lowercase())
        })
    })
}

fn fragment_in(table: &Table, value: &str) -> Option<String> {
    let word = value.trim();
    if word.is_empty() {
        return None;
    }
    let start = &value[..value.len() - value.trim_start().len()];
    let end = &value[value.trim_end().len()..];
    let t = match table.exact.get(word) {
        Some(t) => (*t).to_owned(),
        // "3 arguments", "module `a`"…: a phrase built from a template.
        None if word.len() <= 80 => {
            let t = text_line(table, word)?;
            if has_english(&t) {
                return None;
            }
            t
        }
        None => return None,
    };
    Some(format!("{start}{t}{end}"))
}

/// The translation of a word or phrase that the compiler puts into a message
/// ("function", "this scope"…), keeping the spaces around it.
pub(crate) fn fragment(value: &str) -> Option<String> {
    fragment_in(table()?, value)
}

#[cfg(test)]
mod tests {
    use super::{fill, match_holes, parse, split_holes, text_line};

    #[test]
    fn format_templates() {
        let parts = split_holes("cannot find {} `{}` in {}");
        assert_eq!(parts, ["cannot find ", " `", "` in ", ""]);
        let values = match_holes(&parts, "cannot find value `x` in this scope").unwrap();
        assert_eq!(values, ["value", "x", "this scope"]);
        assert_eq!(fill("لا يوجد {0} اسمه `{1}` في {2}", &values), "لا يوجد value اسمه `x` في this scope");
        assert_eq!(fill("{} و {}", &["a", "b"]), "a و b");
        assert!(match_holes(&parts, "something else").is_none());
        let parts = split_holes("cannot borrow {}{} as mutable more than once at a time");
        let values = match_holes(&parts, "cannot borrow `v` as mutable more than once at a time").unwrap();
        assert_eq!(values, ["", "`v`"]);
    }

    #[test]
    fn values_are_translated() {
        let table = parse(
            "en: cannot find {} `{}` in {}\nar: لا يوجد {0} اسمه `{1}` في {2}\n\n\
             en: this {} takes {}{} but {} {} supplied\nar: هذه الـ {0} تأخذ {1}{2} لكن أُعطيت {3} {4}\n\n\
             en: {} arguments\nar: {0} وسائط\n\nen: arguments were\nar: وسائط\n\n\
             en: function\nar: دالة\n\nen: this scope\nar: هذا النطاق\n",
        );
        assert_eq!(
            text_line(&table, "cannot find function `f` in this scope").as_deref(),
            Some("لا يوجد دالة اسمه `f` في هذا النطاق")
        );
        // An English word left in a hole: better all English than half.
        assert_eq!(text_line(&table, "cannot find thing `f` in this scope"), None);
        assert_eq!(
            text_line(&table, "this function takes 2 arguments but 3 arguments were supplied").as_deref(),
            Some("هذه الـ دالة تأخذ 2 وسائط لكن أُعطيت 3 وسائط")
        );
    }
}
