This crate is currently developed in-tree together with the compiler.

Our goal is to start publishing `rodetc_public` into crates.io.
Until then, users will use this as any other rustc crate, by installing
the rustup component `rustc-dev`, and declaring `rustc-public` as an external crate.

See the rodetc_public ["Getting Started"](https://rust-lang.github.io/rustc_public/getting-started.html)
guide for more information.

## Design

The `rodetc_public` crate will follow a similar approach to [`proc-macro2`](https://crates.io/crates/proc-macro2). Its
implementation is split between two main crates:

- `rodetc_public`: Public crate, to be published on crates.io, which will contain
the "stable" data structure as well as calls to `rodetc_public_bridge` APIs. The
translation between public and internal constructs is also done in this crate.
- `rodetc_public_bridge`: This crate implements the public APIs to the compiler.
It is responsible for gathering all the information requested, and providing
the data in its unstable internal form.

I.e.,
tools will depend on `rodetc_public` crate,
which will invoke the compiler using APIs defined in `rodetc_public_bridge`.

I.e.:

```
    ┌────────────────────────────┐           ┌───────────────────────────┐
    │      External Tool         │           │         Rust Compiler     │
    │            ┌────────────┐  │           │ ┌────────┐                │
    │            │            │  │           │ │        │                │
    │            │rodetc_public│  │           │ │rustc   │                │
    │            │            │  ├──────────►| │public  │                │
    │            │            │  │◄──────────┤ │bridge  │                │
    │            │            │  │           │ │        │                │
    │            │            │  │           │ │        │                │
    │            └────────────┘  │           │ └────────┘                │
    └────────────────────────────┘           └───────────────────────────┘
```

More details can be found here:
https://hackmd.io/XhnYHKKuR6-LChhobvlT-g?view
