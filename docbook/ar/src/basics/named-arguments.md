# الوسائط بالاسم

> **جديد في Rust++:** عند استدعاء دالة، اكتب `name = value` لتحدد أي معامل
> يأخذ أي قيمة.

```rust
fn transfer(from: &str, to: &str, amount: u32) {
    out!("{amount} from {from} to {to}");
}

fn main() {
    transfer("Ali", "Sara", 100);                      // أيها أيها؟
    transfer(from = "Ali", to = "Sara", amount = 100); // واضح
    transfer(amount = 100, to = "Sara", from = "Ali"); // بأي ترتيب
}
```

الاستدعاءات الثلاثة تفعل نفس الشيء. الاستدعاء بالأسماء أوضح في القراءة، ولا
يمكنك أن تبدّل `from` و`to` بالخطأ.

## بالترتيب أولًا، ثم بالاسم

يمكنك إعطاء القيم الأولى بالترتيب وتسمية الباقي:

```rust
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    out!("{}", area(3, height = 4));
}
```

بعد أول قيمة بالاسم، يجب أن تكون كل القيم التالية بالاسم:

```rust,compile_fail
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    area(width = 3, 4); // خطأ: قيمة بالترتيب بعد قيمة بالاسم
}
```

## الدوال المرتبطة والمُنشئات

الوسائط بالاسم تعمل مع الدوال المرتبطة (methods) ومع `Type::new` أيضًا:

```rust
struct Window {
    width: u32,
    height: u32,
}

impl Window {
    fn new(width: u32, height: u32) -> Window {
        Window { width, height }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

fn main() {
    let mut w = Window::new(width = 800, height = 600);
    w.resize(height = 1080, width = 1920);
    out!("{}x{}", w.width, w.height);
}
```

## أخطاء يكتشفها المترجم

| ما تكتبه | الخطأ |
|---|---|
| `area(widht = 3, height = 4)` | هذه الدالة ليس لها معامل اسمه `widht` |
| `area(3, width = 4)` | المعامل `width` أُعطي أكثر من مرة |
| `area(width = 3)` | لم تُعطَ قيمة للمعامل `height` |

## لماذا `=` وليس `:`

في Rust++، `:` تعني دائمًا "نوعه" (`let x: u32`) و`=` تعني دائمًا "قيمته"
(`let x = 5`). الوسيط بالاسم يعطي قيمة، لذلك يستخدم `=`.

## التكلفة

لا شيء. المترجم يعيد القيم إلى ترتيبها الصحيح قبل بناء البرنامج؛ والنتيجة
مثل الاستدعاء بالترتيب تمامًا.
