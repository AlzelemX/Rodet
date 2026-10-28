use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::source::snippet_with_applicability;
use rodetc_errors::Applicability;
use rodetc_hir::Expr;
use rodetc_lint::LateContext;
use rodetc_middle::ty::Ty;

use super::FN_TO_NUMERIC_CAST_ANY;

pub(super) fn check(cx: &LateContext<'_>, expr: &Expr<'_>, cast_expr: &Expr<'_>, cast_from: Ty<'_>, cast_to: Ty<'_>) {
    if cast_from.is_fn() {
        let mut applicability = Applicability::MaybeIncorrect;
        let from_snippet = snippet_with_applicability(cx, cast_expr.span, "..", &mut applicability);

        span_lint_and_then(
            cx,
            FN_TO_NUMERIC_CAST_ANY,
            expr.span,
            format!("casting function pointer `{from_snippet}` to `{cast_to}`"),
            |diag| {
                diag.span_suggestion_verbose(
                    expr.span,
                    "did you mean to invoke the function?",
                    format!("{from_snippet}() as {cast_to}"),
                    applicability,
                );
            },
        );
    }
}
