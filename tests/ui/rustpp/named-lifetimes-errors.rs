// Rust++: borrow errors with `_a` lifetimes.

fn mix<_a, _b>(x: &_a str, _y: &_b str) -> &_b str {
    x
    //~^ ERROR lifetime may not live long enough
}

struct Holder<_a> {
    s: &_a str,
}

fn make<_a>() -> Holder<_a> {
    let local = String::from("temp");
    Holder { s: &local }
    //~^ ERROR cannot return value referencing local variable `local`
}

fn main() {}
