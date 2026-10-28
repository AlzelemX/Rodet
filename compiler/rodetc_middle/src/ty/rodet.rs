//! Rodet: finding the default value of a function parameter.
//!
//! The parser turns `fn open(path: &str, write: bool = false)` into the
//! function plus a hidden constant next to it,
//! `const __rodet_default__open__write: bool = false;` (an associated
//! constant for methods of an inherent `impl`). See `rodetc_parse`.

use rodetc_hir::def::{DefKind, Res};
use rodetc_span::Symbol;
use rodetc_span::def_id::DefId;

use crate::ty::{AssocTag, TyCtxt};

/// The constant holding the default value of parameter `param` of the
/// function `fn_def_id`, if it has one.
pub fn param_default(tcx: TyCtxt<'_>, fn_def_id: DefId, param: Symbol) -> Option<DefId> {
    let name =
        Symbol::intern(&format!("__rodet_default__{}__{}", tcx.item_name(fn_def_id), param));
    let parent = tcx.parent(fn_def_id);
    match tcx.def_kind(parent) {
        DefKind::Impl { of_trait: false } => tcx
            .associated_items(parent)
            .filter_by_name_unhygienic_and_kind(name, AssocTag::Const)
            .next()
            .map(|item| item.def_id),
        DefKind::Mod => {
            let children = match parent.as_local() {
                Some(local) => tcx.module_children_local(local),
                None => tcx.module_children(parent),
            };
            children.iter().find_map(|child| match child.res {
                Res::Def(DefKind::Const, def_id) if child.ident.name == name => Some(def_id),
                _ => None,
            })
        }
        _ => None,
    }
}
