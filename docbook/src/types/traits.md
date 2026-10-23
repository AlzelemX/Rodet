# Traits and Generics

## Traits: what a type can do

A **trait** is a list of methods. A type that has those methods
*implements* the trait.

```rust
trait Speak {
    fn name(&self) -> String;

    // a method with a body is a default: types can keep or replace it
    fn greet(&self) -> String {
        format!("Hello from {}", self.name())
    }
}

struct Cat;
struct Robot {
    id: u32,
}

impl Speak for Cat {
    fn name(&self) -> String {
        String::from("a cat")
    }
}

impl Speak for Robot {
    fn name(&self) -> String {
        format!("robot #{}", self.id)
    }
    fn greet(&self) -> String {
        format!("BEEP. I AM {}", self.name().to_uppercase())
    }
}

fn main() {
    out!("{}", Cat.greet());
    out!("{}", Robot { id: 7 }.greet());
}
```

## Generics: one function, many types

`<T>` is a type parameter: a placeholder for "some type". `T: Speak` says
"any type that implements `Speak`":

```rust
trait Speak {
    fn name(&self) -> String;
}

struct Cat;
impl Speak for Cat {
    fn name(&self) -> String { String::from("cat") }
}

fn introduce<T: Speak>(thing: &T) {
    out!("This is {}", thing.name());
}

fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut best = items[0];
    for &item in items {
        if item > best {
            best = item;
        }
    }
    best
}

fn main() {
    introduce(&Cat);
    out!("{}", largest(&[3, 9, 2]));
    out!("{}", largest(&[1.5, 0.5]));
    out!("{}", largest(&['a', 'z', 'm']));
}
```

The compiler makes a separate, fully optimized copy of a generic function
for each type you use. Generics cost nothing at run time.

## Traits you'll use a lot

| Trait | Gives you | Usually from |
|---|---|---|
| `Debug` | `{:?}` printing | `#[derive(Debug)]` |
| `Display` | `{}` printing | `impl` by hand |
| `Clone` / `Copy` | `.clone()` / copying | `#[derive(...)]` |
| `PartialEq` | `==` and `!=` | `#[derive(PartialEq)]` |
| `PartialOrd` | `<`, `>` | `#[derive(PartialOrd)]` |
| `Default` | `Type::default()` | `#[derive(Default)]` |

```rust
use std::fmt;

struct Money {
    cents: u64,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}.{:02}", self.cents / 100, self.cents % 100)
    }
}

fn main() {
    let price = Money { cents: 1999 };
    out!("{price}"); // $19.99
}
```
