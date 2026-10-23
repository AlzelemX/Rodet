//! Documentation attribute related utilities.
use std::{
    borrow::Cow,
    sync::atomic::{AtomicBool, Ordering},
};

use either::Either;
use hir::{HasAttrs, db::HirDatabase, resolve_doc_path_on};

/// Holds documentation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Documentation<'db>(Cow<'db, str>);

impl<'db> Documentation<'db> {
    #[inline]
    pub fn new_owned(s: String) -> Self {
        Documentation(Cow::Owned(s))
    }

    #[inline]
    pub fn new_borrowed(s: &'db str) -> Self {
        Documentation(Cow::Borrowed(s))
    }

    #[inline]
    pub fn into_owned(self) -> Documentation<'static> {
        Documentation::new_owned(self.0.into_owned())
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// RUST++: the standard library documents its items in English and Arabic at
// the same time; the Arabic of a doc comment sits in a block
//
//     <div lang="ar" dir="rtl">
//
//     ...
//
//     </div>
//
// Hovers, completions and signature help show one language: English without
// the Arabic blocks, or the Arabic followed by the English code examples.

const ARABIC_OPEN: &str = r#"<div lang="ar" dir="rtl">"#;
const ARABIC_CLOSE: &str = "</div>";

static SHOW_ARABIC: AtomicBool = AtomicBool::new(false);

/// RUST++: show documentation in Arabic where it has an Arabic block.
pub fn set_docs_in_arabic(arabic: bool) {
    SHOW_ARABIC.store(arabic, Ordering::Relaxed);
}

/// The byte ranges of the Arabic blocks in `docs` (whole lines, with the blank
/// line before each block), and the ranges of their contents.
fn arabic_blocks(docs: &str) -> Vec<(std::ops::Range<usize>, std::ops::Range<usize>)> {
    let mut blocks = Vec::new();
    let mut offset = 0;
    let mut open: Option<(usize, usize)> = None;
    let mut last_text_end = 0;
    for line in docs.split_inclusive('\n') {
        let end = offset + line.len();
        match line.trim() {
            ARABIC_OPEN if open.is_none() => open = Some((last_text_end, end)),
            ARABIC_CLOSE => {
                if let Some((mut start, content_start)) = open.take() {
                    // At the very end, also take the line break before the block.
                    if !line.ends_with('\n') && start > 0 && docs[..start].ends_with('\n') {
                        start -= 1;
                    }
                    blocks.push((start..end, content_start..offset));
                }
            }
            "" => {}
            _ if open.is_none() => last_text_end = end,
            _ => {}
        }
        offset = end;
    }
    blocks
}

/// The fenced code blocks of `docs` outside the Arabic blocks.
fn english_code_blocks(docs: &str, skip: &[std::ops::Range<usize>]) -> String {
    let mut out = String::new();
    let mut offset = 0;
    let mut in_code = false;
    for line in docs.split_inclusive('\n') {
        let start = offset;
        offset += line.len();
        if skip.iter().any(|r| r.contains(&start)) {
            continue;
        }
        let fence = line.trim_start().starts_with("```");
        if fence && !in_code && !out.is_empty() {
            out.push('\n');
        }
        if in_code || fence {
            out.push_str(line);
        }
        if fence {
            in_code = !in_code;
        }
    }
    out
}

/// `docs` in the language the user chose, or `None` when there is nothing to change.
pub fn docs_in_language(docs: &str) -> Option<String> {
    let blocks = arabic_blocks(docs);
    if blocks.is_empty() {
        return None;
    }
    if SHOW_ARABIC.load(Ordering::Relaxed) {
        let mut out = String::new();
        for (_, content) in &blocks {
            let text = docs[content.clone()].trim_matches('\n');
            if !out.is_empty() {
                out.push_str("\n\n");
            }
            out.push_str(text);
        }
        let skip: Vec<_> = blocks.iter().map(|(all, _)| all.clone()).collect();
        let code = english_code_blocks(docs, &skip);
        if !code.is_empty() {
            out.push_str("\n\n");
            out.push_str(code.trim_end_matches('\n'));
        }
        Some(out)
    } else {
        let mut out = docs.to_owned();
        for (all, _) in blocks.iter().rev() {
            out.replace_range(all.clone(), "");
        }
        Some(out)
    }
}

/// Like [`docs_in_language`], for docs that come with a source map: in English the
/// map is kept (only whole lines are removed); in Arabic the text is rebuilt.
pub fn docs_with_rangemap_in_language<'db>(
    docs: Either<Cow<'db, hir::Docs>, Documentation<'db>>,
) -> Either<Cow<'db, hir::Docs>, Documentation<'db>> {
    match docs {
        Either::Left(map) => {
            let blocks = arabic_blocks(map.docs());
            if blocks.is_empty() {
                Either::Left(map)
            } else if SHOW_ARABIC.load(Ordering::Relaxed) {
                Either::Right(Documentation::new_owned(
                    docs_in_language(map.docs()).unwrap_or_default(),
                ))
            } else {
                let mut map = map.into_owned();
                for (all, _) in blocks.iter().rev() {
                    map.remove_lines(all.clone());
                }
                Either::Left(Cow::Owned(map))
            }
        }
        Either::Right(docs) => match docs_in_language(docs.as_str()) {
            Some(changed) => Either::Right(Documentation::new_owned(changed)),
            None => Either::Right(docs),
        },
    }
}

fn in_language(docs: Documentation<'_>) -> Documentation<'_> {
    match docs_in_language(docs.as_str()) {
        Some(changed) => Documentation::new_owned(changed),
        None => docs,
    }
}

pub trait HasDocs: HasAttrs + Copy {
    fn docs(self, db: &dyn HirDatabase) -> Option<Documentation<'_>> {
        let docs = match self.docs_with_rangemap(db)? {
            Cow::Borrowed(docs) => Documentation::new_borrowed(docs.docs()),
            Cow::Owned(docs) => Documentation::new_owned(docs.into_docs()),
        };
        Some(in_language(docs))
    }
    fn docs_with_rangemap(self, db: &dyn HirDatabase) -> Option<Cow<'_, hir::Docs>> {
        self.hir_docs(db).map(Cow::Borrowed)
    }
    fn resolve_doc_path(
        self,
        db: &dyn HirDatabase,
        link: &str,
        ns: Option<hir::Namespace>,
        is_inner_doc: hir::IsInnerDoc,
    ) -> Option<hir::DocLinkDef> {
        resolve_doc_path_on(db, self, link, ns, is_inner_doc)
    }
}

macro_rules! impl_has_docs {
    ($($def:ident,)*) => {$(
        impl HasDocs for hir::$def {}
    )*};
}

impl_has_docs![
    EnumVariant,
    Field,
    Static,
    Const,
    Trait,
    TypeAlias,
    Macro,
    Function,
    Adt,
    Module,
    Impl,
    Crate,
    AssocItem,
    Struct,
    Union,
    Enum,
];

impl HasDocs for hir::ExternCrateDecl {
    fn docs(self, db: &dyn HirDatabase) -> Option<Documentation<'_>> {
        let crate_docs = self.resolved_crate(db)?.hir_docs(db);
        let decl_docs = self.hir_docs(db);
        match (decl_docs, crate_docs) {
            (None, None) => None,
            (Some(docs), None) | (None, Some(docs)) => {
                Some(in_language(Documentation::new_borrowed(docs.docs())))
            }
            (Some(decl_docs), Some(crate_docs)) => {
                let mut docs = String::with_capacity(
                    decl_docs.docs().len() + "\n\n".len() + crate_docs.docs().len(),
                );
                docs.push_str(decl_docs.docs());
                docs.push_str("\n\n");
                docs.push_str(crate_docs.docs());
                Some(in_language(Documentation::new_owned(docs)))
            }
        }
    }

    fn docs_with_rangemap(self, db: &dyn HirDatabase) -> Option<Cow<'_, hir::Docs>> {
        let crate_docs = self.resolved_crate(db)?.hir_docs(db);
        let decl_docs = self.hir_docs(db);
        match (decl_docs, crate_docs) {
            (None, None) => None,
            (Some(docs), None) | (None, Some(docs)) => Some(Cow::Borrowed(docs)),
            (Some(decl_docs), Some(crate_docs)) => {
                let mut docs = decl_docs.clone();
                docs.append_str("\n\n");
                docs.append(crate_docs);
                Some(Cow::Owned(docs))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The language is global; tests that change it take turns.
    static LANGUAGE: std::sync::Mutex<()> = std::sync::Mutex::new(());

    const DOCS: &str = "Returns the length.\n\n# Examples\n\n```\nassert_eq!(v.len(), 3);\n```\n\n<div lang=\"ar\" dir=\"rtl\">\n\nيرجع الطول.\n\n</div>";

    #[test]
    fn docs_without_arabic_are_unchanged() {
        assert_eq!(docs_in_language("Returns the length."), None);
    }

    #[test]
    fn english_hides_the_arabic_block() {
        let _language = LANGUAGE.lock().unwrap();
        set_docs_in_arabic(false);
        assert_eq!(
            docs_in_language(DOCS).as_deref(),
            Some("Returns the length.\n\n# Examples\n\n```\nassert_eq!(v.len(), 3);\n```")
        );
    }

    #[test]
    fn arabic_keeps_the_english_examples() {
        let _language = LANGUAGE.lock().unwrap();
        set_docs_in_arabic(true);
        let docs = docs_in_language(DOCS);
        set_docs_in_arabic(false);
        assert_eq!(docs.as_deref(), Some("يرجع الطول.\n\n```\nassert_eq!(v.len(), 3);\n```"));
    }
}
