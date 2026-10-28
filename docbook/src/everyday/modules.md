# Modules

As a program grows, split it into **modules**. A module is a named group of
functions, types and other modules.

```rust
mod shapes {
    pub struct Circle {
        pub radius: f64,
    }

    impl Circle {
        pub fn area(&self) -> f64 {
            PI * self.radius * self.radius
        }
    }

    const PI: f64 = 3.14159; // private: only usable inside `shapes`
}

use shapes::Circle;

fn main() {
    let c = Circle { radius: 2.0 };
    out!("{:.2}", c.area());
}
```

- Everything is **private** by default. `pub` makes it usable outside the
  module.
- `use` brings a name into scope so you don't have to write the full path.
- `::` separates path parts: `shapes::Circle`, `std::collections::HashMap`.

## One module per file

In a Rodet project, `mod shapes;` (with `;`) loads the module from a file:

```text
src/
├── main.rd        mod shapes;  use shapes::Circle;
└── shapes.rd      pub struct Circle { ... }
```

A folder works too: `src/shapes/mod.rd`, or `src/shapes.rd` next to a
`src/shapes/` folder for its sub-modules.

## Paths

| Path | Means |
|---|---|
| `crate::shapes::Circle` | from the top of your project |
| `super::helper` | from the parent module |
| `self::inner` | from this module |
| `std::fs` | from the standard library |
| `rand::random` | from a library in `Silk.toml` |
