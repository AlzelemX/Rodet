#![feature(rustc_private)]
#![deny(clippy::disallowed_methods)]

extern crate rodetc_errors;
extern crate rodetc_hir;
extern crate rodetc_lint;
extern crate rodetc_middle;

use rodetc_errors::{DiagDecorator, DiagMessage, MultiSpan};
use rodetc_hir::HirId;
use rodetc_lint::{Lint, LintContext};
use rodetc_middle::ty::TyCtxt;

pub fn a(cx: impl LintContext, lint: &'static Lint, span: impl Into<MultiSpan>, msg: impl Into<DiagMessage>) {
    cx.emit_span_lint(
        //~^ disallowed_methods
        lint,
        span,
        DiagDecorator(|lint| {
            lint.primary_message(msg);
        }),
    );
}

pub fn b(tcx: TyCtxt<'_>, lint: &'static Lint, hir_id: HirId, span: impl Into<MultiSpan>, msg: impl Into<DiagMessage>) {
    tcx.emit_node_span_lint(
        //~^ disallowed_methods
        lint,
        hir_id,
        span,
        DiagDecorator(|lint| {
            lint.primary_message(msg);
        }),
    );
}

fn main() {}
