//@ run-pass

#![allow(non_upper_case_globals)]
#![feature(rustc_private)]
extern crate rodetc_macros;
extern crate rodetc_serialize;
extern crate rodetc_span;

use rodetc_macros::{Decodable, Encodable};

// Necessary to pull in object code as the rest of the rustc crates are shipped only as rmeta
// files.
#[allow(unused_extern_crates)]
extern crate rodetc_driver;

pub const other: u8 = 1;
pub const f: u8 = 1;
pub const d: u8 = 1;
pub const s: u8 = 1;
pub const state: u8 = 1;
pub const cmp: u8 = 1;

#[allow(dead_code)]
#[derive(Ord, Eq, PartialOrd, PartialEq, Debug, Decodable, Encodable, Hash)]
struct Foo {}

fn main() {}
