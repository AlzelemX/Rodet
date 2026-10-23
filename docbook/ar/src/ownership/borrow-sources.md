# مصادر الاستعارة

> **جديد في Rust++:** عندما ترجع الدالة مرجعًا، اكتب من أي المعاملات تستعير:
> `&{x, y} T`.

```rust
fn longest(x: &str, y: &str) -> &{x, y} str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("apple");
    let b = String::from("fig");
    out!("{}", longest(&a, &b)); // apple
}
```

اقرأ `-> &{x, y} str` هكذا: *ترجع `&str` مستعارًا من `x` أو من `y`*. الآن
يعرف المترجم أن النتيجة صالحة فقط ما دام `a` و`b` موجودين.

## فقط المعاملات المهمة

```rust
fn pick(list: &Vec<i32>, index: &usize) -> &{list} i32 {
    &list[*index]
}

fn main() {
    let numbers = vec![10, 20, 30];
    let x = {
        let i = 1;
        pick(&numbers, &i)   // يُحرَّر `i` بعد هذه الكتلة...
    };
    out!("{x}");             // ...وهذا صحيح: x يستعير من `numbers` فقط
}
```

لأن النتيجة `&{list} i32`، يعرف المترجم أنها لا تستعير من `index`. المعلومة
الدقيقة تعني أخطاء خاطئة أقل.

## في أي مكان من نوع الإرجاع

```rust
fn first_word(text: &mut String) -> (&{text} str, usize) {
    let end = text.find(' ').unwrap_or(text.len());
    (&text[..end], end)
}

fn get<T>(items: &[T], i: usize) -> Option<&{items} T> {
    items.get(i)
}

fn main() {
    let mut s = String::from("hello world");
    let (w, n) = first_word(&mut s);
    out!("{w} {n}");
    out!("{:?}", get(&[1, 2, 3], 2)); // Some(3)
}
```

## self

في الدوال المرتبطة، `self` مصدر مثل أي مصدر آخر:

```rust
struct Library {
    books: Vec<String>,
}

impl Library {
    fn first(&self) -> &{self} str {
        &self.books[0]
    }

    fn first_or(&self, other: &str, use_other: bool) -> &{self, other} str {
        if use_other { other } else { &self.books[0] }
    }
}

fn main() {
    let lib = Library { books: vec![String::from("Dune")] };
    out!("{} {}", lib.first(), lib.first_or("none", true));
}
```

## هل أحتاجها دائمًا؟

لا. مثل Rust، عندما تأخذ الدالة مرجعًا **واحدًا**، أو تأخذ `&self`، يعرف
المترجم من أين تأتي النتيجة:

```rust
fn trim_dots(text: &str) -> &str {    // مصدر واحد فقط: لا حاجة لـ {..}
    text.trim_matches('.')
}

fn main() {
    out!("{}", trim_dots("..hi.."));
}
```

اكتب `&{...}` عندما يكون هناك مرجعان أو أكثر يمكن الاختيار بينها.

## القواعد

- كل اسم يجب أن يكون معاملًا من نوع مرجع (`&T` أو `&mut T`)، أو `self`.
- لا تكلفة وقت التشغيل: المترجم يحوّل `&{x, y}` إلى أعمار ويتحقق من نفس
  القواعد تمامًا.
