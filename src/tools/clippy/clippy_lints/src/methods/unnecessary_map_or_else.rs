use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::is_expr_identity_function;
use clippy_utils::res::MaybeDef as _;
use clippy_utils::source::snippet_with_applicability;
use rodetc_errors::Applicability;
use rodetc_hir::Expr;
use rodetc_lint::LateContext;
use rodetc_span::Span;
use rodetc_span::symbol::sym;

use super::{UNNECESSARY_OPTION_MAP_OR_ELSE, UNNECESSARY_RESULT_MAP_OR_ELSE};

/// lint use of `_.map_or_else(|err| err, |n| n)` for `Result`s and `Option`s.
pub(super) fn check(
    cx: &LateContext<'_>,
    expr: &Expr<'_>,
    recv: &Expr<'_>,
    def_arg: &Expr<'_>,
    map_arg: &Expr<'_>,
    call_span: Span,
) {
    let (symbol, lint) = match cx.typeck_results().expr_ty(recv).opt_diag_name(cx) {
        Some(x @ sym::Result) => (x, UNNECESSARY_RESULT_MAP_OR_ELSE),
        Some(x @ sym::Option) => (x, UNNECESSARY_OPTION_MAP_OR_ELSE),
        _ => return,
    };

    if is_expr_identity_function(cx, map_arg) {
        let msg = format!("unused \"map closure\" when calling `{symbol}::map_or_else` value");

        span_lint_and_then(cx, lint, expr.span, msg, |diag| {
            let mut applicability = Applicability::MachineApplicable;
            let err_snippet = snippet_with_applicability(cx, def_arg.span, "..", &mut applicability);
            let sugg = format!("unwrap_or_else({err_snippet})");

            diag.span_suggestion_verbose(call_span, "consider using `unwrap_or_else`", sugg, applicability);
        });
    }
}
