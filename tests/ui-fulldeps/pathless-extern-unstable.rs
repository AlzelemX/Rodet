//@ edition:2018
//@ compile-flags:--extern rodetc_middle

// Test that `--extern rodetc_middle` fails with `rustc_private`.

pub use rodetc_middle;
//~^ ERROR use of unstable library feature `rustc_private`

fn main() {}
