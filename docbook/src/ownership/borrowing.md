# Borrowing

Moving a value into every function that reads it would be annoying. Instead
you **lend** it: `&value` makes a *reference*, a pointer that borrows the
value without owning it.

```rust
fn length(text: &String) -> usize {
    text.len()
}   // `text` is only borrowed: nothing is freed

fn main() {
    let s = String::from("hello");
    let n = length(&s);   // lend `s`
    out!("{s} has {n} bytes"); // `s` is still ours
}
```

## Changing through a borrow: &mut

`&` lends for reading. `&mut` lends for changing:

```rust
fn add_world(text: &mut String) {
    text.push_str(", world");
}

fn main() {
    let mut s = String::from("hello");
    add_world(&mut s);
    out!("{s}"); // hello, world
}
```

## The borrowing rule

At any moment you can have **either**

- any number of `&` (readers), **or**
- exactly one `&mut` (one writer),

but not both.

```rust,compile_fail
fn main() {
    let mut list = vec![1, 2, 3];
    let first = &list[0];   // reader
    list.push(4);           // writer while the reader still exists: error
    out!("{first}");
}
```

This is not fussiness: `push` may move the list to a bigger place in
memory, and `first` would then point at freed memory. In C++ this compiles
and crashes later; in Rodet it doesn't compile.

A borrow ends after its last use, so this is fine:

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    let first = list[0];    // a copy of the number, not a borrow
    list.push(4);
    out!("{first} {list:?}");
}
```

## A reference can't outlive its value

```rust,compile_fail
fn main() {
    let r;
    {
        let s = String::from("short life");
        r = &s;
    }               // `s` is freed here...
    out!("{r}");    // ...so `r` would point at nothing: error
}
```

When a **function returns a reference**, the compiler needs to know which
value it borrows from, to check this rule at the call. In Rust that's what
lifetimes (`'a`) are for. Rodet has an easier way: the next chapter.
