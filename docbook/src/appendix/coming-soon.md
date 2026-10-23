# Coming Soon

Ideas that are planned but not built yet. **None of the code on this page
compiles today**, and the final syntax may change. The live list is the
[status board](https://github.com/AlzelemX/Rust-#status).

## `data`: one type for struct and enum

One keyword for "this and that" and "this or that", with fields shared by
all kinds:

```rust,ignore
data Shape {
    color: Color,              // every Shape has a color
    | Circle { radius: f64 }
    | Rect { w: f64, h: f64 }
}
```

`struct` and `enum` stay; `data` is for when you need both.

## Named match

A `match` that is also a function, for rules you use in several places:

```rust,ignore
match price(size: Size) -> u32 {
    Size::Small => 10,
    Size::Big => 20,
}
```

## No more `'a`

Borrow sources and `_a` already replace most lifetimes. The plan is to
remove `'a` completely, including in the standard library and in error
messages.

## Error messages in Rust++ style

Compiler errors that show Rust++ syntax (`&{x}`, `_a`) in their
suggestions.

## `on_ready()`

A declarative constructor: code that runs when a value is ready.

## Android and iOS

The standard library for phones.
