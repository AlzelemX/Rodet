use super::INLINE_ALWAYS;
use super::utils::is_relevant_expr;
use clippy_utils::diagnostics::span_lint;
use rodetc_attr_ir::{Attribute, InlineAttr, find_attr};
use rodetc_hir::BodyId;
use rodetc_lint::LateContext;
use rodetc_span::Span;
use rodetc_span::symbol::Symbol;

pub(super) fn check(cx: &LateContext<'_>, span: Span, name: Symbol, attrs: &[Attribute], body: Option<BodyId>) {
    if span.from_expansion() {
        return;
    }

    if let Some(span) = find_attr!(attrs, Inline(InlineAttr::Always, span) => *span)
        && body.is_none_or(|body| is_relevant_expr(cx, cx.tcx.hir_body(body).value))
    {
        span_lint(
            cx,
            INLINE_ALWAYS,
            span,
            format!("you have declared `#[inline(always)]` on `{name}`. This is usually a bad idea"),
        );
    }
}
