#![deny(clippy::collapsible_span_lint_calls)]
#![allow(clippy::missing_clippy_version_attribute)]
#![feature(rustc_private)]

extern crate clippy_utils;
extern crate rodetc_ast;
extern crate rodetc_errors;
extern crate rodetc_lint;
extern crate rodetc_span;

use clippy_utils::diagnostics::{span_lint_and_help, span_lint_and_note, span_lint_and_sugg, span_lint_and_then};
use rodetc_ast::ast::Expr;
use rodetc_errors::Applicability;
use rodetc_lint::{EarlyContext, EarlyLintPass, declare_lint_pass, declare_tool_lint};

declare_tool_lint! {
    pub clippy::TEST_LINT,
    Warn,
    "",
    report_in_external_macro: true
}

declare_lint_pass!(Pass => [TEST_LINT]);

impl EarlyLintPass for Pass {
    fn check_expr(&mut self, cx: &EarlyContext, expr: &Expr) {
        let lint_msg = "lint message";
        let help_msg = "help message";
        let note_msg = "note message";
        let sugg = "new_call()";
        let predicate = true;

        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.span_suggestion(expr.span, help_msg, sugg.to_string(), Applicability::MachineApplicable);
        });
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.span_help(expr.span, help_msg);
        });
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.help(help_msg);
        });
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.span_note(expr.span, note_msg);
        });
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.note(note_msg);
        });

        // This expr shouldn't trigger this lint.
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            db.note(note_msg);
            if predicate {
                db.note(note_msg);
            }
        });

        // Issue #8798
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            db.help(help_msg).help(help_msg);
        });

        // Issue #15880
        #[expect(clippy::disallowed_names)]
        let foo = "foo";
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.span_suggestion(
                expr.span,
                format!("try using {foo}"),
                format!("{foo}.use"),
                Applicability::MachineApplicable,
            );
        });
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.span_help(expr.span, format!("try using {foo}"));
        });
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.help(format!("try using {foo}"));
        });
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.span_note(expr.span, format!("required because of {foo}"));
        });
        span_lint_and_then(cx, TEST_LINT, expr.span, lint_msg, |db| {
            //~^ collapsible_span_lint_calls
            db.note(format!("required because of {foo}"));
        });
    }
}

fn main() {}
