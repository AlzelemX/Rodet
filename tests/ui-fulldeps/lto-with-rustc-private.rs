//! Regression test for <https://github.com/rust-lang/rust/issues/45689>.

//@ build-fail
//@ compile-flags: -Clto
//@ normalize-stderr: "error: crate .* required.*\n(   .*\n)*\n" -> ""
//@ normalize-stderr: "aborting due to [0-9]+" -> "aborting due to NUMBER"
//@ dont-require-annotations: ERROR

#![feature(rustc_private)]

extern crate rodetc_errors;
//~? ERROR crate `rodetc_errors` required to be available in rlib format

fn main() {}
