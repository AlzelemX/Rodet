//@ run-pass
//@ check-run-results
// Rust++: `out!` must print exactly what `println!` would print.

fn main() {
    out!();
    out!("hello from Rust++");
    let name = "out!";
    out!("{name} works like println!: {} + {} = {}", 1, 2, 1 + 2);
    out!("{:>5}|{:<5}|{:^5}", "a", "b", "c");
}
