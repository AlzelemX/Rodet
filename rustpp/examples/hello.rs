// Rust++ example: compile with the Rust++ compiler.
//
//     rustc +rustpp hello.rs && ./hello

fn main() {
    out!("Hello from Rust++!");

    let lang = "Rust++";
    let year = 2026;
    out!("{lang} was born in {year}.");

    let nums = [3, 1, 4, 1, 5];
    let sum: i32 = nums.iter().sum();
    out!("sum of {:?} = {}", nums, sum);

    out!(); // empty line, just like println!()
    println!("println! still works too.");
}
