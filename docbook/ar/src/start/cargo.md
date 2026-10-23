# المشاريع مع Cargo

`rustc` يترجم ملفًا واحدًا. للبرامج الحقيقية استخدم **Cargo**: ينشئ المشروع،
ويبنيه، ويشغّله، وينزّل المكتبات.

```text
> cargo new hello
> cd hello
```

```text
hello/
├── Cargo.toml      المشروع: الاسم، الإصدار، المكتبات
└── src/
    └── main.rs     الكود الخاص بك
```

عدّل `src/main.rs`:

```rust
fn main() {
    out!("Hello from Cargo!");
}
```

## الأوامر

| الأمر | ماذا يفعل |
|---|---|
| `cargo run` | يبني ويشغّل |
| `cargo build` | يبني فقط (في `target/debug/`) |
| `cargo build --release` | يبني برنامجًا محسّنًا وسريعًا (في `target/release/`) |
| `cargo check` | يبحث عن الأخطاء بدون بناء — الأسرع |
| `cargo test` | يشغّل الاختبارات |

قِس السرعة دائمًا مع `--release`: نسخة التطوير قد تكون أبطأ من 10 إلى 50 مرة.

## المكتبات (crates)

المكتبات اسمها *crates*. أضف واحدة في `Cargo.toml` تحت `[dependencies]`:

```toml
[dependencies]
rand = "0.8"
```

ينزّلها Cargo في المرة القادمة التي تبني فيها. المكتبات المكتوبة لـ Rust
تعمل في Rust++.
