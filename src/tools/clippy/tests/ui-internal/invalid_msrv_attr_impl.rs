#![deny(clippy::missing_msrv_attr_impl)]
#![allow(clippy::missing_clippy_version_attribute)]
#![feature(rustc_private)]

extern crate rodetc_ast;
extern crate rodetc_hir;
#[macro_use]
extern crate rodetc_lint;
extern crate rodetc_middle;
use clippy_utils::extract_msrv_attr;
use clippy_utils::msrvs::MsrvStack;
use rodetc_hir::Expr;
use rodetc_lint::{EarlyContext, EarlyLintPass, LateContext, LateLintPass};

declare_lint! {
    pub TEST_LINT,
    Warn,
    ""
}

struct Pass {
    msrv: MsrvStack,
}

impl_lint_pass!(Pass => [TEST_LINT]);

impl EarlyLintPass for Pass {
    //~^ missing_msrv_attr_impl
    fn check_expr(&mut self, _: &EarlyContext<'_>, _: &rodetc_ast::Expr) {}
}

fn main() {}
