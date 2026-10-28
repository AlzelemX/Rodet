use clippy_utils::diagnostics::span_lint;
use rodetc_hir::def::{CtorKind, CtorOf, DefKind, Res};
use rodetc_hir::{Expr, ExprKind};
use rodetc_lint::LateContext;
use rodetc_middle::ty::{self, Ty};

use super::CAST_ENUM_CONSTRUCTOR;

pub(super) fn check(cx: &LateContext<'_>, expr: &Expr<'_>, cast_expr: &Expr<'_>, cast_from: Ty<'_>) {
    if matches!(cast_from.kind(), ty::FnDef(..))
        && let ExprKind::Path(path) = &cast_expr.kind
        && let Res::Def(DefKind::Ctor(CtorOf::Variant, CtorKind::Fn), _) = cx.qpath_res(path, cast_expr.hir_id)
    {
        span_lint(
            cx,
            CAST_ENUM_CONSTRUCTOR,
            expr.span,
            "cast of an enum tuple constructor to an integer",
        );
    }
}
