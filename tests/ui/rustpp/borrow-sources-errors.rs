// Rust++: mistakes with `&{...}` borrow sources.

fn not_a_param(x: &str) -> &{y} str {
    //~^ ERROR cannot borrow from `y`: this function has no parameter with that name
    x
}

fn not_a_ref(x: String, y: &str) -> &{x} str {
    //~^ ERROR cannot borrow from `x` because it is not a reference
    drop(x);
    y
}

fn wrong_source(x: &str, y: &str) -> &{x} str {
    let _ = x;
    y
    //~^ ERROR explicit lifetime required in the type of `y`
}

fn main() {}
