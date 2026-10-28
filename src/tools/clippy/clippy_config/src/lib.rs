#![feature(io_error_more, rustc_private)]
#![warn(
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_lifetimes,
    unused_qualifications
)]
#![expect(clippy::must_use_candidate)]

extern crate rodetc_attr_ir;
extern crate rodetc_attr_parsing;
extern crate rodetc_data_structures;
extern crate rodetc_errors;
extern crate rodetc_hir;
extern crate rodetc_middle;
extern crate rodetc_session;
extern crate rodetc_span;

#[macro_use]
mod de;
mod conf;
mod metadata;
pub mod types;

pub use conf::{Conf, sanitize_explanation};
pub use metadata::ConfMetadata;
