# Ownership

Ownership is how Rust++ manages memory **without a garbage collector and
without you calling `free`**. It is the one idea that makes Rust++ both fast
and safe, so take your time with this chapter.

## Three rules

1. Every value has **one owner**: the variable that holds it.
2. When the owner goes away (its `{ }` block ends), the value is freed.
3. Giving the value to someone else **moves** it: the old variable can't
   use it any more.

```rust
fn main() {
    {
        let s = String::from("hello"); // s owns the text
        out!("{s}");
    }                                  // block ends: the text is freed
}
```

No leaks (the value is always freed) and no double frees (only one owner
frees it). The compiler checks this; it costs nothing at run time.

## Moving

```rust,compile_fail
fn main() {
    let a = String::from("hello");
    let b = a;           // the text moves from `a` to `b`
    out!("{a}");         // error: borrow of moved value: `a`
}
```

Why? If both `a` and `b` owned the text, both would free it. So `a` gives
the text away and becomes unusable.

The same happens when you pass a value to a function:

```rust
fn shout(text: String) {
    out!("{}!", text.to_uppercase());
}   // `text` is freed here

fn main() {
    let msg = String::from("hi");
    shout(msg);           // `msg` moves into the function
    // out!("{msg}");     // error: `msg` was moved
}
```

## Copy: small values are copied, not moved

Numbers, `bool`, `char`, and tuples/arrays of them are small, so they are
copied instead:

```rust
fn main() {
    let x = 5;
    let y = x;           // copy
    out!("{x} {y}");     // both fine
}
```

## clone: copy on purpose

To really have two copies of a `String` or a `Vec`, ask for it with
`.clone()`. It's visible in the code, so you always see where memory is
copied:

```rust
fn main() {
    let a = String::from("hello");
    let b = a.clone();
    out!("{a} {b}");
}
```

## String vs &str

- `String` **owns** its text. It can grow and change.
- `&str` **borrows** text that someone else owns (a `String`, or text
  written in the program).

Most functions only need to read text, so they take `&str`. That's
*borrowing*, the next chapter.
