# المجموعات

## Vec: قائمة تكبر

```rust
fn main() {
    let mut scores: Vec<u32> = Vec::new();
    scores.push(90);
    scores.push(75);
    scores.push(60);

    let names = vec!["Ali", "Sara", "Omar"]; // vec! ينشئ قائمة بقيمها

    out!("{} scores, first {}", scores.len(), scores[0]);
    out!("{:?}", scores.get(10));            // None: بدون انهيار

    for s in &scores {                       // & = اقرأ ولا تأخذ
        out!("score {s}");
    }
    for s in &mut scores {                   // &mut = غيّر كل واحد
        *s += 5;
    }
    out!("{scores:?} {names:?}");

    let last = scores.pop();                 // Some(65)
    out!("{last:?}");
}
```

`v[i]` يوقف البرنامج إذا كان `i` بعد النهاية؛ و`v.get(i)` يعطي `Option` بدلًا
من ذلك.

## العمل على القوائم: المكررات (iterators)

بدل كتابة الحلقات بيدك، اربط خطوات صغيرة:

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

`|p| p * 2` **إغلاق (closure)**: دالة صغيرة بدون اسم. سلاسل المكررات تُترجم إلى
نفس كود الآلة الذي تنتجه حلقة مكتوبة باليد.

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

## HashMap: ابحث عن قيمة بمفتاح

```rust
use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<&str, u32> = HashMap::new();
    stock.insert("apple", 3);
    stock.insert("fig", 10);

    *stock.entry("apple").or_insert(0) += 2;  // أضف، أو ابدأ من 0
    *stock.entry("kiwi").or_insert(0) += 1;

    if let Some(n) = stock.get("fig") {
        out!("figs: {n}");
    }
    let mut items: Vec<_> = stock.iter().collect();
    items.sort();
    out!("{items:?}");
}
```

الـ `HashMap` ليس لها ترتيب. رتّب العناصر، أو استخدم `BTreeMap`، عندما يهمك
الترتيب.
