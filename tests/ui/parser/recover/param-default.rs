// RODET: functions may have parameter default values.
fn foo(x: i32 = 1) {}

type Foo = fn(i32 = 0); //~ ERROR parameter defaults are not supported

fn main() {}
