# Named Arguments

> **New in Rodet:** when calling a function, write `name = value` to say
> which parameter gets which value.

```rust
fn transfer(from: &str, to: &str, amount: u32) {
    out!("{amount} from {from} to {to}");
}

fn main() {
    transfer("Ali", "Sara", 100);                      // which is which?
    transfer(from = "Ali", to = "Sara", amount = 100); // clear
    transfer(amount = 100, to = "Sara", from = "Ali"); // any order
}
```

All three calls do the same thing. Named calls are clearer to read, and you
can't swap `from` and `to` by mistake.

## Positional first, then named

You can give the first values by position and name the rest:

```rust
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    out!("{}", area(3, height = 4));
}
```

After the first named value, all the rest must be named:

```rust,compile_fail
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    area(width = 3, 4); // error: positional argument after a named argument
}
```

## Methods and constructors

Named arguments work on methods and on `Type::new` too:

```rust
struct Window {
    width: u32,
    height: u32,
}

impl Window {
    fn new(width: u32, height: u32) -> Window {
        Window { width, height }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

fn main() {
    let mut w = Window::new(width = 800, height = 600);
    w.resize(height = 1080, width = 1920);
    out!("{}x{}", w.width, w.height);
}
```

## Mistakes the compiler catches

| You write | Error |
|---|---|
| `area(widht = 3, height = 4)` | this function has no parameter named `widht` |
| `area(3, width = 4)` | parameter `width` is given more than once |
| `area(width = 3)` | missing argument for parameter `height` |

## Why `=` and not `:`

In Rodet, `:` always means "has the type" (`let x: u32`) and `=` always
means "gets the value" (`let x = 5`). A named argument gives a value, so it
uses `=`.

## Cost

None. The compiler puts the values back in the right order before
building the program; the result is the same as a positional call.
