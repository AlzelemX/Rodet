//@ compile-flags: -Z unstable-options
//@ ignore-stage1

#![feature(rustc_private)]
#![deny(rustc::direct_use_of_rustc_type_ir)]

extern crate rodetc_middle;
extern crate rodetc_type_ir;

use rodetc_middle::ty::*; // OK, we have to accept rodetc_middle::ty::*

// We have to deny direct import of type_ir
use rodetc_type_ir::*;
//~^ ERROR: do not use `rodetc_type_ir` unless you are implementing type system internals

// We have to deny direct types usages which resolves to type_ir
fn foo<I: rodetc_type_ir::Interner>(cx: I, did: I::DefId) {
//~^ ERROR: do not use `rodetc_type_ir` unless you are implementing type system internals
}

fn main() {
    let _ = rodetc_type_ir::InferConst::Fresh(42);
//~^ ERROR: do not use `rodetc_type_ir` unless you are implementing type system internals
    let _: rodetc_type_ir::InferConst;
//~^ ERROR: do not use `rodetc_type_ir` unless you are implementing type system internals
}
