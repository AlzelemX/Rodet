# Named Lifetimes

[Borrow sources](borrow-sources.md) cover functions. One place still needs
a name for "how long a borrow lives": **a struct that holds a reference**.

> **New in Rust++:** lifetime names are written like normal names that start
> with `_`: `_a`, `_text`. `'static` is `_static`.

```rust
struct Parser<_a> {
    input: &_a str,
    pos: usize,
}

impl<_a> Parser<_a> {
    fn new(input: &_a str) -> Parser<_a> {
        Parser { input, pos: 0 }
    }

    fn next_word(&mut self) -> Option<&_a str> {
        let rest = &self.input[self.pos..];
        let rest = rest.trim_start();
        if rest.is_empty() {
            return None;
        }
        let start = self.input.len() - rest.len();
        let end = rest.find(' ').unwrap_or(rest.len());
        self.pos = start + end;
        Some(&rest[..end])
    }
}

fn main() {
    let text = String::from("borrow without fear");
    let mut p = Parser::new(&text);
    while let Some(word) = p.next_word() {
        out!("{word}");
    }
}
```

Read `Parser<_a>` as: *a `Parser` that borrows something for a time called
`_a`*, and `input: &_a str` as: *`input` is borrowed for that time*. A
`Parser` can't outlive the text it reads; the compiler checks it.

## _static

`_static` means "for the whole program". Text written in the program is
`&_static str`:

```rust
const GREETING: &_static str = "salam";

fn pick_color(dark: bool) -> &_static str {
    if dark { "black" } else { "white" }
}

fn main() {
    out!("{GREETING} {}", pick_color(true));
}
```

It also works as a bound, meaning "holds no short-lived borrows":

```rust
fn keep<T: _static + std::fmt::Debug>(value: T) -> Box<dyn std::fmt::Debug + _static> {
    Box::new(value)
}

fn main() {
    out!("{:?}", keep(42));
}
```

## Rules

- A lifetime name is `_` followed by a small letter: `_a`, `_x`, `_input`.
- It goes in `<...>` and after `&`.
- A *variable* called `_a` is still just a variable.
- Rust's `'a` still works for now. It will be removed; see
  [Coming Soon](../appendix/coming-soon.md).
