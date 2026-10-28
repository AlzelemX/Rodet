# Borrow Sources

> **New in Rodet:** when a function returns a reference, write which
> parameters it borrows from: `&{x, y} T`.

```rust
fn longest(x: &str, y: &str) -> &{x, y} str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("apple");
    let b = String::from("fig");
    out!("{}", longest(&a, &b)); // apple
}
```

Read `-> &{x, y} str` as: *returns a `&str` that is borrowed from `x` or
`y`*. Now the compiler knows the result is only valid while both `a` and `b`
are alive.

## Only the parameters that matter

```rust
fn pick(list: &Vec<i32>, index: &usize) -> &{list} i32 {
    &list[*index]
}

fn main() {
    let numbers = vec![10, 20, 30];
    let x = {
        let i = 1;
        pick(&numbers, &i)   // `i` is freed after this block...
    };
    out!("{x}");             // ...and that's fine: x borrows only `numbers`
}
```

Because the result is `&{list} i32`, the compiler knows it doesn't borrow
from `index`. Precise information means fewer false errors.

## Anywhere in the return type

```rust
fn first_word(text: &mut String) -> (&{text} str, usize) {
    let end = text.find(' ').unwrap_or(text.len());
    (&text[..end], end)
}

fn get<T>(items: &[T], i: usize) -> Option<&{items} T> {
    items.get(i)
}

fn main() {
    let mut s = String::from("hello world");
    let (w, n) = first_word(&mut s);
    out!("{w} {n}");
    out!("{:?}", get(&[1, 2, 3], 2)); // Some(3)
}
```

## self

In methods, `self` is a source like any other:

```rust
struct Library {
    books: Vec<String>,
}

impl Library {
    fn first(&self) -> &{self} str {
        &self.books[0]
    }

    fn first_or(&self, other: &str, use_other: bool) -> &{self, other} str {
        if use_other { other } else { &self.books[0] }
    }
}

fn main() {
    let lib = Library { books: vec![String::from("Dune")] };
    out!("{} {}", lib.first(), lib.first_or("none", true));
}
```

## Do I always need it?

No. Like in Rust, when a function takes **one** reference, or takes
`&self`, the compiler already knows where the result comes from:

```rust
fn trim_dots(text: &str) -> &str {    // only one source: no {..} needed
    text.trim_matches('.')
}

fn main() {
    out!("{}", trim_dots("..hi.."));
}
```

Write `&{...}` when there are two or more references to choose from.

## Rules

- Each name must be a parameter that is a reference (`&T` or `&mut T`), or
  `self`.
- There's no runtime cost: the compiler turns `&{x, y}` into lifetimes and
  checks exactly the same rules.
