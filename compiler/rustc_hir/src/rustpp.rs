//! Rust++: named arguments, `f(x = 1, y = 2)`.
//!
//! The parser reads `x = 1` in an argument list as an ordinary assignment
//! expression. Name resolution leaves the left-hand side unresolved and AST
//! lowering keeps it as `Assign(x, 1)` (see `lower_call_args`), so in HIR a
//! named argument is an `Assign` whose left side is a single-segment path with
//! `Res::Err`. Type checking and THIR building both use [`order_named_args`]
//! to put the values in parameter order; the call is then exactly like a
//! positional call, so there is no runtime cost.
//!
//! Rules: positional arguments come first; after the first named argument all
//! arguments must be named; each parameter can be given once; every parameter
//! must be given.

use rustc_span::{Ident, Span};

use crate::def::Res;
use crate::{Expr, ExprKind, QPath};

/// If `arg` is a named argument `name = value`, returns `(name, value)`.
pub fn named_arg<'a, 'hir>(arg: &'a Expr<'hir>) -> Option<(Ident, &'hir Expr<'hir>)> {
    let ExprKind::Assign(lhs, value, _) = arg.kind else { return None };
    let ExprKind::Path(QPath::Resolved(None, path)) = lhs.kind else { return None };
    match path.segments {
        [segment] if path.res == Res::Err && segment.args.is_none() => Some((segment.ident, value)),
        _ => None,
    }
}

/// Whether any of `args` is a named argument.
pub fn has_named_args(args: &[Expr<'_>]) -> bool {
    args.iter().any(|arg| named_arg(arg).is_some())
}

/// The arguments with every named argument replaced by its value, in the
/// order they were written. Used when the call cannot be reordered.
pub fn strip_named_args<'hir>(args: &'hir [Expr<'hir>]) -> Vec<&'hir Expr<'hir>> {
    args.iter().map(|arg| named_arg(arg).map_or(arg, |(_, value)| value)).collect()
}

/// A mistake in the use of named arguments.
#[derive(Debug, Clone, Copy)]
pub enum NamedArgsError {
    /// A positional argument after a named one.
    PositionalAfterNamed { span: Span },
    /// More positional arguments than parameters.
    TooManyPositional { span: Span },
    /// No parameter has this name.
    UnknownName { name: Ident },
    /// This parameter was already given (by position or by name).
    AlreadyGiven { name: Ident },
    /// This parameter was not given. `None` if the parameter has no name.
    Missing { name: Option<Ident>, index: usize },
}

/// Puts `args` in the order of `params` (the callee's parameter names).
/// Returns the argument expressions (the values of named arguments), one per
/// parameter.
pub fn order_named_args<'hir>(
    params: &[Option<Ident>],
    args: &'hir [Expr<'hir>],
) -> Result<Vec<&'hir Expr<'hir>>, NamedArgsError> {
    let mut slots: Vec<Option<&'hir Expr<'hir>>> = vec![None; params.len()];
    let mut seen_named = false;
    let mut next_positional = 0;

    for arg in args {
        match named_arg(arg) {
            None => {
                if seen_named {
                    return Err(NamedArgsError::PositionalAfterNamed { span: arg.span });
                }
                let Some(slot) = slots.get_mut(next_positional) else {
                    return Err(NamedArgsError::TooManyPositional { span: arg.span });
                };
                *slot = Some(arg);
                next_positional += 1;
            }
            Some((name, value)) => {
                seen_named = true;
                let Some(index) = params.iter().position(|p| p.is_some_and(|p| p.name == name.name))
                else {
                    return Err(NamedArgsError::UnknownName { name });
                };
                if slots[index].is_some() {
                    return Err(NamedArgsError::AlreadyGiven { name });
                }
                slots[index] = Some(value);
            }
        }
    }

    slots
        .into_iter()
        .enumerate()
        .map(|(index, slot)| slot.ok_or(NamedArgsError::Missing { name: params[index], index }))
        .collect()
}
