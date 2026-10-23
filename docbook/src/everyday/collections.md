# Collections

## Vec: a list that grows

```rust
fn main() {
    let mut scores: Vec<u32> = Vec::new();
    scores.push(90);
    scores.push(75);
    scores.push(60);

    let names = vec!["Ali", "Sara", "Omar"]; // vec! makes one with values

    out!("{} scores, first {}", scores.len(), scores[0]);
    out!("{:?}", scores.get(10));            // None: no crash

    for s in &scores {                       // & = read, don't take
        out!("score {s}");
    }
    for s in &mut scores {                   // &mut = change each one
        *s += 5;
    }
    out!("{scores:?} {names:?}");

    let last = scores.pop();                 // Some(65)
    out!("{last:?}");
}
```

`v[i]` stops the program if `i` is past the end; `v.get(i)` gives an
`Option` instead.

## Working with lists: iterators

Instead of writing loops by hand, chain small steps:

```rust
fn main() {
    let prices = vec![12, 5, 40, 8, 25];

    let total: u32 = prices.iter().sum();
    let cheap: Vec<u32> = prices.iter().filter(|&&p| p < 20).cloned().collect();
    let doubled: Vec<u32> = prices.iter().map(|p| p * 2).collect();
    let max = prices.iter().max();

    out!("{total} {cheap:?} {doubled:?} {max:?}");

    for (i, p) in prices.iter().enumerate() {
        if i < 2 { out!("#{i}: {p}"); }
    }
}
```

`|p| p * 2` is a **closure**: a small function without a name. Iterator
chains compile to the same machine code as a hand-written loop.

## String

```rust
fn main() {
    let mut s = String::from("Rust");
    s.push_str("++");
    s.push('!');

    let joined = format!("{s} is {}", "fast");
    out!("{joined}");

    for word in "one two three".split(' ') {
        out!("{word}");
    }
    out!("{}", "  padded  ".trim());
    out!("{}", "hello".to_uppercase());
    out!("{}", "hello".contains("ell"));
}
```

## HashMap: look up a value by a key

```rust
use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<&str, u32> = HashMap::new();
    stock.insert("apple", 3);
    stock.insert("fig", 10);

    *stock.entry("apple").or_insert(0) += 2;  // add, or start at 0
    *stock.entry("kiwi").or_insert(0) += 1;

    if let Some(n) = stock.get("fig") {
        out!("figs: {n}");
    }
    let mut items: Vec<_> = stock.iter().collect();
    items.sort();
    out!("{items:?}");
}
```

A `HashMap` has no order. Sort the items, or use `BTreeMap`, when order
matters.
