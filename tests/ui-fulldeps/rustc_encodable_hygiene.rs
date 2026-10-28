//@ check-pass

#![feature(rustc_private)]

extern crate rodetc_macros;
extern crate rodetc_serialize;
extern crate rodetc_span;

// Necessary to pull in object code as the rest of the rustc crates are shipped only as rmeta
// files.
#[allow(unused_extern_crates)]
extern crate rodetc_driver;

use rodetc_macros::{Decodable, Encodable};

#[derive(Decodable, Encodable, Debug)]
struct A {
    a: String,
}

trait Trait {
    fn encode(&self);
}

impl<T> Trait for T {
    fn encode(&self) {
        unimplemented!()
    }
}

fn main() {}
