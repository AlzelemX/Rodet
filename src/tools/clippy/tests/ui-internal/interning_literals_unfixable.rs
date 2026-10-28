//@no-rustfix: paths that don't exist yet
#![feature(rustc_private)]

extern crate rodetc_span;

use rodetc_span::Symbol;

fn main() {
    // Not yet defined
    let _ = Symbol::intern("xyz123");
    //~^ interning_literals
    let _ = Symbol::intern("with-dash");
    //~^ interning_literals
    let _ = Symbol::intern("with.dot");
    //~^ interning_literals
}
