# المشاريع مع Silk

`rodetc` يترجم ملفًا واحدًا. للبرامج الحقيقية استخدم **Silk**، مدير حزم
Rodet: أمره `rodet`. ينشئ المشروع، ويبنيه، ويشغّله، وينزّل المكتبات.

```text
> rodet new hello
> cd hello
```

```text
hello/
├── Silk.toml       المشروع: الاسم، الإصدار، المكتبات
└── src/
    └── main.rd     الكود الخاص بك
```

ملفات Rodet تنتهي بـ `.rd`. عدّل `src/main.rd`:

```rust
fn main() {
    out!("Hello from Silk!");
}
```

## الأوامر

| الأمر | ماذا يفعل |
|---|---|
| `rodet run` | يبني ويشغّل |
| `rodet build` | يبني فقط (في `target/debug/`) |
| `rodet build --release` | يبني برنامجًا محسّنًا وسريعًا (في `target/release/`) |
| `rodet check` | يبحث عن الأخطاء بدون بناء — الأسرع |
| `rodet test` | يشغّل الاختبارات |
| `rodet add rand` | يضيف مكتبة |

قِس السرعة دائمًا مع `--release`: نسخة التطوير قد تكون أبطأ من 10 إلى 50 مرة.

## المكتبات

أضف مكتبة بالأمر `rodet add`، أو اكتبها في `Silk.toml` تحت `[dependencies]`:

```toml
[dependencies]
rand = "0.8"
```

ينزّلها Silk في المرة القادمة التي تبني فيها، ويكتب الإصدارات بالضبط في
`Silk.lock`. المكتبات تأتي من crates.io، فكل مكتبة كُتبت لـ Rust تعمل في
Rodet أيضًا (Silk يقرأ ملفاتها `Cargo.toml` و`.rs`).

## الملفات والمجلدات

| | |
|---|---|
| `Silk.toml` | مشروعك |
| `Silk.lock` | إصدارات المكتبات بالضبط (احفظه في git للبرامج) |
| `src/main.rd` | برنامج؛ و`src/lib.rd` مكتبة |
| `src/bin/*.rd` و`examples/*.rd` و`tests/*.rd` | برامج وأمثلة واختبارات إضافية |
| `build.rd` | يعمل قبل البناء، إن وُجد |
| `~/.rodet` | Rodet نفسها والمكتبات المنزّلة (`RODET_HOME` يغيّر المكان) |
| `.rodet/config.toml` | إعدادات مشروع واحد |
