//@ run-pass
//@ check-run-results
// Rust++: named arguments, `f(x = 1, y = 2)`.

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn describe(name: &str, age: u32, city: &str) -> String {
    format!("{name}, {age}, {city}")
}

struct Window {
    w: u32,
    h: u32,
}

impl Window {
    fn resize(&mut self, width: u32, height: u32) {
        self.w = width;
        self.h = height;
    }

    fn new(width: u32, height: u32) -> Self {
        Window { w: width, h: height }
    }
}

fn pair<T: std::fmt::Debug, U: std::fmt::Debug>(first: T, second: U) -> String {
    format!("{first:?}/{second:?}")
}

fn main() {
    out!("{}", area(width = 3, height = 4));
    out!("{}", area(height = 4, width = 5));
    out!("{}", area(6, height = 2));

    out!("{}", describe("Ali", city = "Riyadh", age = 30));

    let mut win = Window::new(height = 600, width = 800);
    out!("{}x{}", win.w, win.h);
    win.resize(height = 1080, width = 1920);
    out!("{}x{}", win.w, win.h);

    // the values can be any expression, including variables with the same name
    let width = 10;
    let height = width * 2;
    out!("{}", area(height = height, width = width));

    out!("{}", pair(second = "b", first = 1));

    // inside a closure
    let f = |h: u32| area(height = h, width = 2);
    out!("{}", f(21));

    // positional calls are unchanged
    out!("{}", area(7, 7));
}
