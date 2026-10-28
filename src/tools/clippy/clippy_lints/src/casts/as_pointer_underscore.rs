use rodetc_errors::Applicability;
use rodetc_lint::LateContext;
use rodetc_middle::ty::Ty;

pub fn check<'tcx>(cx: &LateContext<'tcx>, ty_into: Ty<'_>, cast_to_hir: &'tcx rodetc_hir::Ty<'tcx>) {
    if let rodetc_hir::TyKind::Ptr(ty, _) = cast_to_hir.kind
        && matches!(ty.kind, rodetc_hir::TyKind::Infer(()))
    {
        clippy_utils::diagnostics::span_lint_and_sugg(
            cx,
            super::AS_POINTER_UNDERSCORE,
            cast_to_hir.span,
            "using inferred pointer cast",
            "use explicit type",
            ty_into.to_string(),
            Applicability::MachineApplicable,
        );
    }
}
