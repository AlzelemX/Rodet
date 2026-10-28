//@ run-pass
//@ check-run-results
// Rodet: `out!` must print exactly what `println!` would print.

fn main() {
    out!();
    out!("hello from Rodet");
    let name = "out!";
    out!("{name} works like println!: {} + {} = {}", 1, 2, 1 + 2);
    out!("{:>5}|{:<5}|{:^5}", "a", "b", "c");
}
