//@ compile-flags: -Z unstable-options
//@ ignore-stage1

#![feature(rustc_private)]
#![deny(rustc::usage_of_type_ir_traits)]

extern crate rodetc_type_ir;

use rodetc_type_ir::Interner;

fn foo<I: Interner>(cx: I, did: I::TraitId) {
    let _ = cx.trait_is_unsafe(did);
    //~^ ERROR do not use `rodetc_type_ir::Interner` or `rodetc_type_ir::InferCtxtLike` unless you're inside of the trait solver
}

fn main() {}
