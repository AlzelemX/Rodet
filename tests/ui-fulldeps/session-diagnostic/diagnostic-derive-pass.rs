//@ check-pass
//@ compile-flags: -Ztrack-diagnostics

// The proc_macro2 crate handles spans differently when on beta/stable release rather than nightly,
// changing the output of this test. Since Diagnostic is strictly internal to the compiler
// the test is just ignored on stable and beta:
//@ ignore-stage1
//@ ignore-beta
//@ ignore-stable

#![feature(rustc_private)]
#![crate_type = "lib"]

extern crate rodetc_span;
use rodetc_span::Span;

extern crate rodetc_macros;
use rodetc_macros::Diagnostic;

extern crate rodetc_errors;
use rodetc_errors::ErrCode;

extern crate rodetc_session;

extern crate core;

// E0123 is no longer used, so we define our own constant here just for this test.
const E0123: ErrCode = ErrCode::from_u32(0123);

#[derive(Diagnostic)]
#[diag("message", code = E0123)]
struct Diagnostic {
    #[primary_span]
    #[label("label text")]
    span: Span,
    #[context]
    context: Span,
}
