# الأعمار المسماة

[مصادر الاستعارة](borrow-sources.md) تكفي للدوال. يبقى مكان واحد يحتاج اسمًا
لـ "كم تعيش الاستعارة": **struct يحمل مرجعًا**.

> **جديد في Rodet:** أسماء الأعمار تُكتب مثل الأسماء العادية التي تبدأ بـ
> `_`: `_a`، `_text`. و`'static` أصبحت `_static`.

```rust
struct Parser<_a> {
    input: &_a str,
    pos: usize,
}

impl<_a> Parser<_a> {
    fn new(input: &_a str) -> Parser<_a> {
        Parser { input, pos: 0 }
    }

    fn next_word(&mut self) -> Option<&_a str> {
        let rest = &self.input[self.pos..];
        let rest = rest.trim_start();
        if rest.is_empty() {
            return None;
        }
        let start = self.input.len() - rest.len();
        let end = rest.find(' ').unwrap_or(rest.len());
        self.pos = start + end;
        Some(&rest[..end])
    }
}

fn main() {
    let text = String::from("borrow without fear");
    let mut p = Parser::new(&text);
    while let Some(word) = p.next_word() {
        out!("{word}");
    }
}
```

اقرأ `Parser<_a>` هكذا: *`Parser` يستعير شيئًا لمدة اسمها `_a`*، و
`input: &_a str` هكذا: *`input` مستعار لتلك المدة*. الـ `Parser` لا يمكن أن
يعيش أطول من النص الذي يقرؤه؛ والمترجم يتحقق من ذلك.

## _static

`_static` تعني "طوال البرنامج". النص المكتوب في البرنامج نوعه `&_static str`:

```rust
const GREETING: &_static str = "salam";

fn pick_color(dark: bool) -> &_static str {
    if dark { "black" } else { "white" }
}

fn main() {
    out!("{GREETING} {}", pick_color(true));
}
```

وتعمل أيضًا كشرط، بمعنى "لا يحمل استعارات قصيرة العمر":

```rust
fn keep<T: _static + std::fmt::Debug>(value: T) -> Box<dyn std::fmt::Debug + _static> {
    Box::new(value)
}

fn main() {
    out!("{:?}", keep(42));
}
```

## القواعد

- اسم العمر هو `_` يتبعها حرف إنجليزي صغير: `_a`، `_x`، `_input`.
- يُكتب داخل `<...>` وبعد `&`.
- *المتغير* الذي اسمه `_a` يبقى متغيرًا عاديًا.
- اسم يبدأ بـ `_` ويتبعه `::` أو `!` أو `<` هو مسار عادي وليس عمرًا:
  `_paths::Thing`.
- `'a` من Rust ما زالت تعمل الآن. ستُزال؛ انظر [قريبًا](../appendix/coming-soon.md).
