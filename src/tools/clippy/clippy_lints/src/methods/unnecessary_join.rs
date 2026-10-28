use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::res::MaybeDef as _;
use rodetc_ast::ast::LitKind;
use rodetc_attr_ir::lang_items::LangItem;
use rodetc_errors::Applicability;
use rodetc_hir::{Expr, ExprKind};
use rodetc_lint::LateContext;
use rodetc_middle::ty;
use rodetc_span::Span;

use super::UNNECESSARY_JOIN;

pub(super) fn check<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
    join_self_arg: &'tcx Expr<'tcx>,
    join_arg: &'tcx Expr<'tcx>,
    span: Span,
) {
    let applicability = Applicability::MachineApplicable;
    let collect_output_adjusted_type = cx.typeck_results().expr_ty_adjusted(join_self_arg);
    if let ty::Ref(_, ref_type, _) = collect_output_adjusted_type.kind()
        // the turbofish for collect is ::<Vec<String>>
        && let ty::Slice(slice) = *ref_type.kind()
        && slice.is_lang_item(cx, LangItem::String)
        // the argument for join is ""
        && let ExprKind::Lit(spanned) = &join_arg.kind
        && let LitKind::Str(symbol, _) = spanned.node
        && symbol.is_empty()
    {
        span_lint_and_sugg(
            cx,
            UNNECESSARY_JOIN,
            span.with_hi(expr.span.hi()),
            r#"called `.collect::<Vec<String>>().join("")` on an iterator"#,
            "consider using",
            "collect::<String>()".to_owned(),
            applicability,
        );
    }
}
