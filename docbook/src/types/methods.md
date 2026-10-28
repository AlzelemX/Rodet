# Methods

Functions that belong to a type go in an `impl` block:

```rust
struct Rect {
    width: f64,
    height: f64,
}

impl Rect {
    // no `self`: called as Rect::new(...)
    fn new(width: f64, height: f64 = 1.0) -> Rect {
        Rect { width, height }
    }

    fn square(size: f64) -> Rect {
        Rect::new(width = size, height = size)
    }

    // &self: reads the value
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // &mut self: changes the value
    fn scale(&mut self, by: f64 = 2.0) {
        self.width *= by;
        self.height *= by;
    }

    // self: takes the value (it is moved into the method)
    fn into_square(self) -> Rect {
        Rect::square(self.width.max(self.height))
    }
}

fn main() {
    let mut r = Rect::new(width = 3.0, height = 4.0);
    out!("{}", r.area());        // 12
    r.scale();
    out!("{}", r.area());        // 48
    r.scale(by = 0.5);
    let s = r.into_square();
    out!("{}", s.area());        // 16
    // r can't be used any more: into_square took it
}
```

| First parameter | Means | Call |
|---|---|---|
| (none) | belongs to the type | `Rect::new(...)` |
| `&self` | reads | `r.area()` |
| `&mut self` | changes | `r.scale()` |
| `self` | takes | `r.into_square()` |

Pick the weakest one that works: `&self` if you only read.

`Self` (capital S) is a short name for the type of the `impl`:
`fn new(...) -> Self` means `-> Rect`.

> **Rodet:** [named arguments](../basics/named-arguments.md) and
> [default values](../basics/default-values.md) work on methods:
> `Rect::new(width = 3.0)`, `r.scale(by = 0.5)`.
