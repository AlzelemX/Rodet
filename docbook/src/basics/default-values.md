# Default Values

> **New in Rodet:** a parameter can have a default value: `name: Type = value`.
> Leave it out when calling, and the default is used.

```rust
fn connect(host: &str, port: u16 = 443, secure: bool = true) {
    out!("{host}:{port} secure={secure}");
}

fn main() {
    connect("example.com");                       // port 443, secure
    connect("example.com", 8080);                 // secure
    connect("localhost", port = 3000, secure = false);
    connect("localhost", secure = false);         // port 443
}
```

```text
example.com:443 secure=true
example.com:8080 secure=true
localhost:3000 secure=false
localhost:443 secure=false
```

With [named arguments](named-arguments.md) you can skip any parameter that
has a default, not only the last ones: `connect("localhost", secure = false)`
keeps the default port.

## In types

Defaults work in `impl` blocks too, which removes the need for many
`new_with_...` functions:

```rust
struct Counter {
    value: i64,
    step: i64,
}

impl Counter {
    fn new(start: i64 = 0, step: i64 = 1) -> Counter {
        Counter { value: start, step }
    }

    fn tick(&mut self, times: i64 = 1) {
        self.value += self.step * times;
    }
}

fn main() {
    let mut a = Counter::new();
    a.tick();
    a.tick(times = 5);
    out!("{}", a.value); // 6

    let b = Counter::new(step = 10);
    out!("{}", b.value + b.step); // 10
}
```

## Rules

- **The default must be a constant**: numbers, `"text"`, `true`, `'c'`,
  `None`, constants, and math on them (`MAX / 2`).

  ```rust,compile_fail
  fn hello(name: String = String::from("world")) {} // error: not a constant

  fn main() {}
  ```

  For a `String` default, take `&str` instead: `fn hello(name: &str = "world")`.

- Allowed on functions and in `impl Type { ... }`. Not yet in traits.
- Parameters without a default must still be given.

## Cost

None. Each default is stored once as a hidden constant, and a call that
leaves a parameter out uses that constant as if you had written it.
