//! Rust++: named arguments, default values, `_a` lifetimes and borrow sources.

use super::{check_no_mismatches, check_types};

#[test]
fn named_arguments() {
    check_types(
        r#"
fn area(width: u32, height: u8) -> u32 { width * height as u32 }
struct Window { w: u32 }
impl Window {
    fn new(width: u32, height: u8) -> Window { Window { w: width } }
    fn resize(&mut self, width: u32, height: u8) {}
}
fn test() {
    let a = area(height = 4, width = 3);
      //^ u32
    let b = area(3, height = 4);
      //^ u32
    let mut w = Window::new(height = 1, width = 2);
      //^^^^^ Window
    w.resize(height = 1, width = 2);
}
"#,
    );
}

#[test]
fn default_values() {
    check_no_mismatches(
        r#"
fn open(path: &str, read: bool = true, write: bool = false) -> u32 { 0 }
struct Counter { n: i64 }
impl Counter {
    fn new(start: i64 = 0) -> Counter { Counter { n: start } }
    fn add(&mut self, by: i64 = 1) -> &mut Counter { self }
}
fn test() {
    let a: u32 = open("a");
    let b: u32 = open("b", false);
    let c: u32 = open("c", write = true);
    let mut d = Counter::new();
    d.add().add(by = 10);
}
"#,
    );
}

#[test]
fn named_lifetimes_and_borrow_sources() {
    check_types(
        r#"
struct Parser<_a> { input: &_a str }
impl<_a> Parser<_a> {
    fn rest(&self) -> &_a str { self.input }
}
fn longest(x: &str, y: &str) -> &{x, y} str { x }
const NAME: &_static str = "Rust++";
fn test(p: Parser<'_>) {
    let r = p.rest();
      //^ &'? str
    let l = longest("a", "b");
      //^ &'? str
    NAME;
  //^^^^ &'static str
}
"#,
    );
}
