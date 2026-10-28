//@ edition: 2024
//@ check-pass
//@ compile-flags: -Zunpretty=expanded

#![crate_type = "rlib"]
#![feature(rustc_private)]

extern crate rodetc_macros;
extern crate rodetc_serialize;
extern crate rodetc_span;

use rodetc_macros::Encodable;

#[derive(Encodable)]
struct UnitStruct;

#[derive(Encodable)]
struct EmptyStruct {}

#[derive(Encodable)]
enum EmptyEnum {}

#[derive(Encodable)]
enum SingleFieldlessEnum {
    A,
}

#[derive(Encodable)]
enum SingleEnum {
    A(u32),
}

#[derive(Encodable)]
enum FieldlessEnum {
    A,
    B,
}

#[derive(Encodable)]
enum PartlyFieldlessEnum {
    A,
    B(u32),
}
