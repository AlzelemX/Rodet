# Hello, World!

Make a file called `main.rd` (Rodet files end in `.rd`):

```rust
fn main() {
    out!("Hello, world!");
}
```

Compile it and run it:

```text
> rodetc main.rd
> ./main          (on Windows: .\main.exe)
Hello, world!
```

## What each part means

- `fn main()` — a function called `main`. Every program starts here.
- `{ ... }` — the body of the function.
- `out!("...")` — prints a line. The `!` means `out!` is a *macro*: a piece
  of code that writes code for you.
- `;` — ends the statement.

> **New in Rodet:** `out!` prints a line. It works exactly like Rust's
> `println!`, which still works too.

## Printing values

Put `{}` where a value should go, or the name of a variable inside `{}`:

```rust
fn main() {
    let name = "Sara";
    let age = 25;

    out!("{name} is {age} years old");
    out!("{} + {} = {}", 2, 3, 2 + 3);
    out!();                        // empty line
    out!("{:?}", [1, 2, 3]);       // {:?} prints lists and other data
    out!("{:.2}", 3.14159);        // 2 digits after the point: 3.14
}
```

```text
Sara is 25 years old
2 + 3 = 5

[1, 2, 3]
3.14
```

Other printing macros:

| Macro | Does |
|---|---|
| `out!` | print a line |
| `print!` | print without a new line |
| `eprintln!` | print a line to the error output |
| `format!` | make a `String` instead of printing |
