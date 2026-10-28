//! Rodet syntax extensions handled by the parser.
//!
//! # Borrow sources: `&{x, y} T`
//!
//! In a function's return type, `&{x, y} T` says "this reference borrows from
//! the parameters `x` and `y`", instead of naming an abstract lifetime:
//!
//! ```text
//! fn longest(x: &str, y: &str) -> &{x, y} str
//! ```
//!
//! The parser rewrites this into ordinary lifetimes, so every later stage
//! (resolution, type checking, borrow checking) works unchanged and the
//! generated code is exactly what plain lifetimes would produce:
//!
//! ```text
//! fn longest<'x, 'y, 'x_y>(x: &'x str, y: &'y str) -> &'x_y str
//! where 'x: 'x_y, 'y: 'x_y
//! ```
//!
//! With a single source, `&{x} T` simply becomes `&'x T`.
//!
//! # Named lifetimes: `_a`
//!
//! For the cases that need a named lifetime (for example a struct that holds a
//! reference to data outside of it), Rodet writes lifetimes as identifiers that
//! start with `_` and a lowercase letter, in positions where a lifetime can
//! appear (generic parameters and arguments, references, bounds, `where`):
//!
//! ```text
//! struct Parser<_a> { input: &_a str }
//! impl<_a> Parser<_a> { fn rest(&self) -> &_a str { self.input } }
//! fn keep<T: _static>(t: T) {}
//! ```
//!
//! `_a` is the lifetime `'_a` and `_static` is `'static`. In expressions
//! (`let _a = 1;`) such names are still ordinary variables.
//!
//! # Parameter default values: `fn open(path: &str, write: bool = false)`
//!
//! Each default value becomes a hidden constant next to the function (a
//! module item for free functions, an associated constant for methods in an
//! inherent `impl`):
//!
//! ```text
//! const __rodet_default__open__write: bool = false;
//! ```
//!
//! Calls that leave the parameter out use that constant (see
//! `rustc_middle::ty::rodet::param_default`). Default values must therefore
//! be constant expressions.

use rustc_ast::mut_visit::{self, MutVisitor};
use rustc_ast::token::{IdentKind, Token};
use rustc_ast::visit::LifetimeCtxt;
use rustc_ast::attr;
use rustc_ast::*;
use rustc_errors::PResult;
use rustc_span::{Ident, Span, Symbol, kw, sym};
use thin_vec::ThinVec;

use super::{FnContext, FnParseMode, Parser};
use crate::{diagnostics, exp};

/// While parsing, `&{x, y}` is stored as a placeholder lifetime named
/// `'{x, y}`; [`Parser::desugar_borrow_sources`] replaces it once the whole
/// signature is known.
const PLACEHOLDER_PREFIX: &str = "'{";

impl<'a> Parser<'a> {
    /// Parses the `{x, y}` of `&{x, y} T`, right after the `&`.
    /// Returns `None` (consuming nothing) if the next token is not `{`.
    pub(super) fn parse_borrow_sources(&mut self) -> PResult<'a, Option<Lifetime>> {
        if self.token != token::OpenBrace {
            return Ok(None);
        }
        let lo = self.token.span;
        self.bump(); // `{`

        let mut names = Vec::new();
        while self.token != token::CloseBrace {
            let name = if self.token.is_keyword(kw::SelfLower) {
                self.bump();
                kw::SelfLower
            } else {
                self.parse_ident()?.name
            };
            names.push(name.to_string());
            if !self.eat(exp!(Comma)) {
                break;
            }
        }
        self.expect(exp!(CloseBrace))?;
        let span = lo.to(self.prev_token.span);

        if names.is_empty() {
            self.dcx().emit_err(diagnostics::EmptyBorrowSources { span });
        }
        let placeholder = format!("{PLACEHOLDER_PREFIX}{}}}", names.join(", "));
        Ok(Some(Lifetime { id: DUMMY_NODE_ID, ident: Ident::new(Symbol::intern(&placeholder), span) }))
    }

    /// Replaces every `&{...}` placeholder in the return type of a function with
    /// a real lifetime, and gives the listed parameters matching lifetimes.
    pub(super) fn desugar_borrow_sources(&mut self, generics: &mut Generics, decl: &mut FnDecl) {
        let FnDecl { inputs, output } = decl;
        let FnRetTy::Ty(ret_ty) = output else { return };

        let mut placeholders = Placeholders::default();
        placeholders.visit_ty(ret_ty);
        if placeholders.found.is_empty() {
            return;
        }

        let mut new_params: Vec<GenericParam> = Vec::new();
        for (placeholder, span) in std::mem::take(&mut placeholders.found) {
            if placeholders.renames.iter().any(|(p, _)| *p == placeholder) {
                continue;
            }
            let text = placeholder.as_str();
            let sources: Vec<&str> = text[PLACEHOLDER_PREFIX.len()..text.len() - 1]
                .split(", ")
                .filter(|s| !s.is_empty())
                .collect();

            let mut source_lifetimes = Vec::new();
            let mut failed = sources.is_empty();
            for &source in &sources {
                // `'self` is not a valid lifetime name.
                let lifetime = if source == "self" {
                    "'self_".to_string()
                } else {
                    format!("'{source}")
                };
                let lifetime = Symbol::intern(&lifetime);

                let Some(param) = inputs.iter_mut().find(|p| {
                    matches!(p.pat.kind, PatKind::Ident(_, ident, _) if ident.name.as_str() == source)
                }) else {
                    self.dcx().emit_err(diagnostics::BorrowSourceNotAParam {
                        span,
                        name: source.to_string(),
                    });
                    failed = true;
                    continue;
                };

                match &mut param.ty.kind {
                    TyKind::Ref(lt @ None, ..) | TyKind::PinnedRef(lt @ None, ..) => {
                        let at = param.ty.span.shrink_to_lo();
                        *lt = Some(Lifetime { id: DUMMY_NODE_ID, ident: Ident::new(lifetime, at) });
                    }
                    TyKind::Ref(Some(lt), ..) | TyKind::PinnedRef(Some(lt), ..)
                        if lt.ident.name == lifetime => {}
                    _ => {
                        self.dcx().emit_err(diagnostics::BorrowSourceNotARef {
                            span: param.ty.span,
                            name: source.to_string(),
                        });
                        failed = true;
                        continue;
                    }
                }
                if !new_params.iter().any(|p| p.ident.name == lifetime) {
                    new_params.push(lifetime_param(lifetime, span));
                }
                source_lifetimes.push(lifetime);
            }

            let target = match source_lifetimes.as_slice() {
                // An error was already reported; fall back to an elided lifetime
                // so the placeholder does not cause follow-up errors.
                _ if failed => kw::UnderscoreLifetime,
                [] => kw::UnderscoreLifetime,
                [only] => *only,
                many => {
                    // The result lives no longer than any of its sources:
                    // a new lifetime that every source lifetime outlives.
                    let joined = Symbol::intern(&format!("'{}", sources.join("_")));
                    if !new_params.iter().any(|p| p.ident.name == joined) {
                        new_params.push(lifetime_param(joined, span));
                    }
                    for param in new_params.iter_mut().filter(|p| many.contains(&p.ident.name)) {
                        let bound = Lifetime { id: DUMMY_NODE_ID, ident: Ident::new(joined, span) };
                        param.bounds.push(GenericBound::Outlives(bound));
                    }
                    joined
                }
            };
            placeholders.renames.push((placeholder, Ident::new(target, span)));
        }

        placeholders.visit_ty(ret_ty);

        // Lifetime parameters must come before type and const parameters.
        let mut params: ThinVec<GenericParam> = new_params.into_iter().collect();
        params.extend(std::mem::take(&mut generics.params));
        generics.params = params;
    }
}

fn lifetime_param(name: Symbol, span: Span) -> GenericParam {
    GenericParam {
        id: DUMMY_NODE_ID,
        ident: Ident::new(name, span),
        attrs: AttrVec::new(),
        bounds: ThinVec::new(),
        is_placeholder: false,
        kind: GenericParamKind::Lifetime,
        colon_span: None,
    }
}

/// Finds `&{...}` placeholders (while `renames` is empty) and then replaces
/// them with their real lifetimes.
#[derive(Default)]
struct Placeholders {
    found: Vec<(Symbol, Span)>,
    renames: Vec<(Symbol, Ident)>,
}

impl MutVisitor for Placeholders {
    fn visit_lifetime(&mut self, lt: &mut Lifetime, ctxt: LifetimeCtxt) {
        if lt.ident.as_str().starts_with(PLACEHOLDER_PREFIX) {
            match self.renames.iter().find(|(p, _)| *p == lt.ident.name) {
                Some((_, real)) => lt.ident = *real,
                None => self.found.push((lt.ident.name, lt.ident.span)),
            }
        }
        let _ = ctxt;
        mut_visit::walk_lifetime(self, lt);
    }
}

/// RODET: is `token` a lifetime written as `_name` (see the module docs)?
pub(super) fn is_named_lifetime(token: &Token) -> bool {
    named_lifetime(token).is_some()
}

impl<'a> Parser<'a> {
    /// RODET: whether the token `n` tokens ahead is a lifetime written `_a`.
    ///
    /// `_a` followed by `::`, `!` or `<` is a path, a macro or a generic type
    /// (crates use names like `_serde::Deserialize` in generated code), not
    /// a lifetime.
    pub(super) fn is_named_lifetime_ahead(&self, n: usize) -> bool {
        self.look_ahead(n, is_named_lifetime)
            && !self.look_ahead(n + 1, |t| {
                matches!(t.kind, token::PathSep | token::Bang | token::Lt | token::Shl)
            })
    }
}

/// RODET: the lifetime written as `_name`, if `token` is one:
/// `_a` is `'_a`, `_static` is `'static`.
pub(super) fn named_lifetime(token: &Token) -> Option<Lifetime> {
    let (ident, IdentKind::Normal) = token.ident()? else { return None };
    let name = ident.name.as_str();
    let mut chars = name.chars();
    if chars.next() != Some('_') || !chars.next().is_some_and(|c| c.is_ascii_lowercase()) {
        return None;
    }
    let lifetime =
        if name == "_static" { kw::StaticLifetime } else { Symbol::intern(&format!("'{name}")) };
    Some(Lifetime { id: DUMMY_NODE_ID, ident: Ident::new(lifetime, ident.span) })
}

/// RODET: a parameter default value, `name: Type = value`.
#[derive(Clone)]
pub(super) struct ParamDefault {
    name: Ident,
    ty: Box<Ty>,
    value: Box<Expr>,
}

/// RODET: the name of the hidden constant holding the default value of
/// parameter `param` of function `func`. Keep in sync with
/// `rustc_middle::ty::rodet::param_default`.
fn param_default_name(func: Symbol, param: Symbol) -> Symbol {
    Symbol::intern(&format!("__rodet_default__{func}__{param}"))
}

impl<'a> Parser<'a> {
    /// Parses the `= value` of a parameter `name: Type = value`.
    pub(super) fn parse_param_default(
        &mut self,
        pat: &Pat,
        ty: &Ty,
        fn_parse_mode: &FnParseMode,
    ) -> PResult<'a, ()> {
        let eq_span = self.token.span;
        self.bump(); // `=`
        let value = self.parse_expr()?;
        let span = eq_span.to(value.span);

        if !matches!(fn_parse_mode.context, FnContext::Free | FnContext::Impl) {
            self.dcx().emit_err(diagnostics::ParamDefaultNotAllowed { span });
            return Ok(());
        }
        let PatKind::Ident(BindingMode::NONE | BindingMode::MUT, name, None) = pat.kind else {
            self.dcx().emit_err(diagnostics::ParamDefaultNeedsName { span: pat.span });
            return Ok(());
        };
        self.rodet_param_defaults.push(ParamDefault { name, ty: Box::new(ty.clone()), value });
        Ok(())
    }

    /// Turns the default values collected while parsing a function's
    /// parameters into hidden constants, waiting in `rodet_generated_items`
    /// to be added next to the function.
    pub(super) fn generate_param_defaults(
        &mut self,
        func: Ident,
        vis: &Visibility,
        fn_parse_mode: &FnParseMode,
    ) {
        let defaults = std::mem::take(&mut self.rodet_param_defaults);
        if defaults.is_empty() {
            return;
        }
        if self.in_fn_body || fn_parse_mode.context == FnContext::Trait {
            for default in &defaults {
                self.dcx().emit_err(diagnostics::ParamDefaultNotAllowed { span: default.value.span });
            }
            return;
        }
        let g = &self.psess.attr_id_generator;
        for ParamDefault { name, ty, value } in defaults {
            let span = value.span;
            let attrs = [
                attr::mk_attr_nested_word(g, AttrStyle::Outer, sym::allow, sym::dead_code, span),
                attr::mk_attr_nested_word(
                    g,
                    AttrStyle::Outer,
                    sym::allow,
                    sym::non_upper_case_globals,
                    span,
                ),
                attr::mk_attr_nested_word(g, AttrStyle::Outer, sym::doc, sym::hidden, span),
            ];
            let kind = ItemKind::Const(Box::new(ConstItem {
                defaultness: Defaultness::Implicit,
                ident: Ident::new(param_default_name(func.name, name.name), name.span),
                generics: Generics::default(),
                ty,
                body: Some(value),
                define_opaque: None,
            }));
            self.rodet_generated_items.push(Box::new(Item {
                attrs: attrs.into_iter().collect(),
                id: DUMMY_NODE_ID,
                span,
                vis: vis.clone(),
                kind,
                tokens: None,
            }));
        }
    }

    /// Takes the generated items, to add them to a module.
    pub(super) fn take_generated_items(&mut self) -> ThinVec<Box<Item>> {
        std::mem::take(&mut self.rodet_generated_items)
    }

    /// Takes the generated items as associated items, to add them to an
    /// inherent `impl`. In a trait or trait `impl` they are not allowed.
    pub(super) fn take_generated_assoc_items(&mut self, allowed: bool) -> ThinVec<Box<AssocItem>> {
        let mut assoc = ThinVec::new();
        for item in self.take_generated_items() {
            let Item { attrs, id, span, vis, kind, tokens } = *item;
            match AssocItemKind::try_from(kind) {
                Ok(kind) if allowed => {
                    assoc.push(Box::new(Item { attrs, id, span, vis, kind, tokens }))
                }
                _ => {
                    self.dcx().emit_err(diagnostics::ParamDefaultNotAllowed { span });
                }
            }
        }
        assoc
    }
}
