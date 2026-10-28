//@ compile-flags: -Z unstable-options

#![feature(rustc_private)]
#![deny(rustc::usage_of_type_ir_inherent)]

extern crate rodetc_type_ir;

use rodetc_type_ir::inherent::*;
//~^ ERROR do not use `rodetc_type_ir::inherent` unless you're inside of the trait solver
use rodetc_type_ir::inherent;
//~^ ERROR do not use `rodetc_type_ir::inherent` unless you're inside of the trait solver
use rodetc_type_ir::inherent::Predicate;
//~^ ERROR do not use `rodetc_type_ir::inherent` unless you're inside of the trait solver

fn main() {}
