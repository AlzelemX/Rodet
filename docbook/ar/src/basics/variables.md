# المتغيرات

أنشئ متغيرًا بـ `let`:

```rust
fn main() {
    let x = 5;
    out!("x = {x}");
}
```

## المتغيرات لا تتغير إلا إذا قلت ذلك

```rust,compile_fail
fn main() {
    let x = 5;
    x = 6; // خطأ: لا يمكن تغيير `x` مرتين: المتغير ليس `mut`
}
```

أضف `mut` (قابل للتغيير) ليسمح للمتغير أن يتغير:

```rust
fn main() {
    let mut count = 0;
    count = count + 1;
    count += 1;
    out!("count = {count}"); // 2
}
```

لماذا ليست `mut` هي الأصل؟ عندما لا تتغير القيمة، لا تحتاج أبدًا أن تبحث
عمّن غيّرها. معظم المتغيرات لا تتغير أبدًا، فتبرز القليلة التي عليها `mut`.

## كتابة النوع بنفسك

Rodet تستنتج النوع عنك، لكن يمكنك كتابته بعد `:`:

```rust
fn main() {
    let age: u32 = 30;
    let price: f64 = 9.99;
    let name: &str = "Ali";
    out!("{name}, {age}, {price}");
}
```

تذكّر: **`:` تعني "نوعه"**، و**`=` تعني "قيمته"**. Rodet تحافظ على هذين
المعنيين في كل مكان، أيضًا في [الوسائط بالاسم](named-arguments.md)
و[القيم الافتراضية](default-values.md).

## التظليل (Shadowing)

`let` جديد بنفس الاسم يصنع متغيرًا جديدًا يخفي القديم. ويمكن أن يكون من نوع
مختلف:

```rust
fn main() {
    let input = "42";
    let input: i32 = input.parse().unwrap();
    out!("{}", input + 1); // 43
}
```

## الثوابت

قيم لا تتغير أبدًا في البرنامج كله. النوع مطلوب، والاسم بأحرف كبيرة:

```rust
const MAX_PLAYERS: u32 = 4;
const GAME: &_static str = "Space";

fn main() {
    out!("{GAME}: up to {MAX_PLAYERS} players");
}
```

`&_static str` تعني "نص يعيش طوال البرنامج"؛ انظر
[الأعمار المسماة](../ownership/named-lifetimes.md).
