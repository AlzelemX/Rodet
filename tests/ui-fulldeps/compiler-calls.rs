//@ run-pass
// Test that the Callbacks interface to the compiler works.

//@ ignore-cross-compile
//@ ignore-remote

#![feature(rustc_private)]

extern crate rodetc_driver;
extern crate rodetc_interface;

use rodetc_interface::interface;

struct TestCalls<'a> {
    count: &'a mut u32,
}

impl rodetc_driver::Callbacks for TestCalls<'_> {
    fn config(&mut self, _config: &mut interface::Config) {
        *self.count *= 2;
    }
}

fn main() {
    let mut count = 1;
    let args = vec!["compiler-calls".to_string(), "foo.rs".to_string()];
    rodetc_driver::catch_fatal_errors(|| -> interface::Result<()> {
        rodetc_driver::compiler_entrypoint(&args, &mut TestCalls { count: &mut count });
        Ok(())
    })
    .ok();
    assert_eq!(count, 2);
}
