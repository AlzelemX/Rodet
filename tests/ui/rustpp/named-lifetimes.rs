//@ run-pass
//@ check-run-results
// Rust++: named lifetimes are written `_a` (and `_static`) instead of `'a`.

struct Parser<_a> {
    input: &_a str,
    pos: usize,
}

impl<_a> Parser<_a> {
    fn rest(&self) -> &_a str {
        &self.input[self.pos..]
    }
    fn word(&mut self) -> Option<&_a str> {
        let rest = self.rest();
        let end = rest.find(' ').unwrap_or(rest.len());
        if end == 0 { return None; }
        self.pos += end + 1;
        Some(&rest[..end])
    }
}

fn first_word<_a>(p: &mut Parser<_a>) -> &_a str {
    p.word().unwrap_or("")
}

fn keep<T: _static + std::fmt::Debug>(t: T) -> Box<dyn std::fmt::Debug + _static> {
    Box::new(t)
}

fn longest<_x>(a: &_x str, b: &_x str) -> &_x str where _x: _x {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    let text = String::from("hello rust plus plus");
    let mut p = Parser { input: &text, pos: 0 };
    let w = first_word(&mut p);
    let _a = 5; // still a normal variable here
    out!("{w} | {} | {_a}", p.rest());
    out!("{:?}", keep(42));
    out!("{}", longest("ab", "abc"));
}
