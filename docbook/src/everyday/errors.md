# Errors

Rodet has two kinds of errors:

- **Bugs**, which should never happen: the program stops with `panic!`.
- **Expected failures**, like a missing file or bad input: the function
  returns a `Result` and the caller decides what to do.

## Result

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("can't divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 4.0) {
        Ok(v) => out!("result {v}"),
        Err(e) => out!("error: {e}"),
    }
    match divide(1.0, 0.0) {
        Ok(v) => out!("result {v}"),
        Err(e) => out!("error: {e}"),
    }
}
```

`Result<T, E>` is either `Ok(value)` or `Err(error)`. Like `Option`, you
can't use the value without handling the error first.

## ? : pass the error up

Inside a function that returns `Result`, `?` after a call means: *if this is
an error, return it now; otherwise give me the value*.

```rust
fn parse_pair(text: &str) -> Result<(i32, i32), std::num::ParseIntError> {
    let mut parts = text.split(',');
    let a = parts.next().unwrap_or("").trim().parse::<i32>()?;
    let b = parts.next().unwrap_or("").trim().parse::<i32>()?;
    Ok((a, b))
}

fn main() {
    out!("{:?}", parse_pair("3, 4"));   // Ok((3, 4))
    out!("{:?}", parse_pair("3, x"));   // Err(ParseIntError { kind: InvalidDigit })
}
```

## Quick and simple

| Call | When it's an error |
|---|---|
| `.unwrap()` | stop the program |
| `.expect("message")` | stop the program with your message |
| `.unwrap_or(value)` | use `value` instead |
| `.unwrap_or_default()` | use `0`, `""`, `false`, ... |
| `?` | return the error to the caller |

Use `unwrap` in small programs and tests. In real code, prefer `?` or a
`match`.

## main can return a Result

```rust
use std::fs;

fn main() -> Result<(), std::io::Error> {
    fs::write("hello.txt", "salam")?;
    let text = fs::read_to_string("hello.txt")?;
    out!("{text}");
    fs::remove_file("hello.txt")?;
    Ok(())
}
```

If an error reaches `main`, the program prints it and exits with an error
code.
