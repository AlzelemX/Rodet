# السمات (trait) والأنواع العامة

## الـ trait: ما الذي يستطيع النوع فعله

**الـ trait** قائمة دوال. النوع الذي يملك تلك الدوال *يطبّق* الـ trait.

```rust
trait Speak {
    fn name(&self) -> String;

    // دالة لها جسم هي قيمة افتراضية: يمكن للأنواع أن تبقيها أو تستبدلها
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

## الأنواع العامة (Generics): دالة واحدة لأنواع كثيرة

`<T>` معامل نوع: مكان لـ "نوع ما". و`T: Speak` تعني "أي نوع يطبّق `Speak`":

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

المترجم يصنع نسخة منفصلة ومحسّنة بالكامل من الدالة العامة لكل نوع تستخدمه.
الأنواع العامة لا تكلف شيئًا وقت التشغيل.

## trait ستستخدمها كثيرًا

| الـ trait | يعطيك | يأتي عادة من |
|---|---|---|
| `Debug` | الطباعة بـ `{:?}` | `#[derive(Debug)]` |
| `Display` | الطباعة بـ `{}` | `impl` تكتبه بنفسك |
| `Clone` / `Copy` | `.clone()` / النسخ | `#[derive(...)]` |
| `PartialEq` | `==` و`!=` | `#[derive(PartialEq)]` |
| `PartialOrd` | `<` و`>` | `#[derive(PartialOrd)]` |
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
