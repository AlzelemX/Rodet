// Arabic for texts that come from the compiler (rodetc, rodet check) and
// from rodet-analyzer: errors and warnings, quick fixes, Run lenses.
//
// They are matched by pattern; the parts in `backticks` (names, types) stay
// as they are. A message with no pattern stays in English.
import { arabic, t } from './index';

type Rule = [RegExp, string];

/** Error and warning messages (the first line of a diagnostic). */
const DIAGNOSTICS: Rule[] = [
  // ---- types ----
  [/^mismatched types$/, 'الأنواع غير متطابقة'],
  [/^expected (.+), found (.+)$/, 'المتوقَّع $1، لكن الموجود $2'],
  [/^expected `(.+)`, found `(.+)`$/, 'المتوقَّع `$1`، لكن الموجود `$2`'],
  [/^expected due to this$/, 'النوع المتوقَّع جاء من هنا'],
  [/^type annotations needed(; type must be known at this point)?$/, 'يلزم كتابة النوع: لم يتمكن Rodet من معرفته وحده'],
  [/^type annotations needed for `(.+)`$/, 'يلزم كتابة نوع `$1`'],
  [/^arguments to this function are incorrect$/, 'الوسائط المعطاة لهذه الدالة غير صحيحة'],
  [/^cannot add `(.+)` to `(.+)`$/, 'لا يمكن جمع `$1` مع `$2`'],
  [/^cannot subtract `(.+)` from `(.+)`$/, 'لا يمكن طرح `$1` من `$2`'],
  [/^cannot multiply `(.+)` by `(.+)`$/, 'لا يمكن ضرب `$1` في `$2`'],
  [/^cannot divide `(.+)` by `(.+)`$/, 'لا يمكن قسمة `$1` على `$2`'],
  [/^binary operation `(.+)` cannot be applied to type `(.+)`$/, 'العملية `$1` لا تعمل على النوع `$2`'],
  [/^the trait bound `(.+)` is not satisfied$/, 'الشرط `$1` غير متحقق (trait غير مطبَّق)'],
  [/^the trait `(.+)` is not implemented for `(.+)`$/, 'الـ trait `$1` غير مطبَّق على `$2`'],
  [/^`(.+)` doesn't implement `(.+)`$/, '`$1` لا يطبِّق `$2`'],
  [/^`(.+)` doesn't implement `std::fmt::Display`$/, '`$1` لا يمكن طباعته بـ `{}`: لا يطبِّق `Display`'],
  [/^`(.+)` doesn't implement `Debug`$/, '`$1` لا يمكن طباعته بـ `{:?}`: لا يطبِّق `Debug`'],
  [/^the `\?` operator can only be used in a function that returns `Result` or `Option`.*$/, 'المعامل `?` يعمل فقط داخل دالة ترجع `Result` أو `Option`'],
  [/^integer literal is too large$/, 'الرقم أكبر من الحد المسموح'],
  [/^literal out of range for `(.+)`$/, 'الرقم خارج حدود النوع `$1`'],
  [/^cannot index into a value of type `(.+)`$/, 'لا يمكن الوصول بالفهرس `[ ]` إلى قيمة من النوع `$1`'],
  [/^can't compare `(.+)` with `(.+)`$/, 'لا يمكن مقارنة `$1` مع `$2`'],
  [/^`if` and `else` have incompatible types$/, 'فرعا `if` و`else` يرجعان نوعين مختلفين'],
  [/^`match` arms have incompatible types$/, 'فروع `match` ترجع أنواعًا مختلفة'],

  // ---- names ----
  [/^cannot find value `(.+)` in this scope$/, 'لا يوجد متغير أو قيمة اسمها `$1` هنا'],
  [/^cannot find function `(.+)` in this scope$/, 'لا توجد دالة اسمها `$1` هنا'],
  [/^cannot find type `(.+)` in this scope$/, 'لا يوجد نوع اسمه `$1` هنا'],
  [/^cannot find macro `(.+)` in this scope$/, 'لا يوجد ماكرو اسمه `$1!` هنا'],
  [/^cannot find trait `(.+)` in this scope$/, 'لا يوجد trait اسمه `$1` هنا'],
  [/^cannot find struct, variant or union type `(.+)` in this scope$/, 'لا يوجد struct أو نوع اسمه `$1` هنا'],
  [/^no such value in this scope$/, 'لا توجد قيمة بهذا الاسم هنا'],
  [/^not found in this scope$/, 'غير موجود هنا'],
  [/^unresolved import$/, 'استيراد غير موجود'],
  [/^unresolved import `(.+)`$/, 'الاستيراد `$1` غير موجود'],
  [/^unresolved module, can't find module file: (.+)$/, 'لم يُعثر على ملف الوحدة: $1'],
  [/^unresolved macro `(.+)`$/, 'لم يُعثر على الماكرو `$1`'],
  [/^failed to resolve: use of undeclared type `(.+)`$/, 'النوع `$1` غير معروف (هل نسيت `use`؟)'],
  [/^failed to resolve: use of undeclared crate or module `(.+)`$/, 'المكتبة أو الوحدة `$1` غير معروفة'],
  [/^failed to resolve: (.+)$/, 'تعذّر إيجاد المسار: $1'],
  [/^use of undeclared type `(.+)`$/, 'النوع `$1` غير معروف'],
  [/^use of undeclared crate or module `(.+)`$/, 'المكتبة أو الوحدة `$1` غير معروفة'],

  // ---- functions, methods, fields ----
  [/^this function takes (\d+) arguments? but (\d+) arguments? (?:was|were) supplied$/, 'هذه الدالة تأخذ $1 وسيط، لكن أُعطيت $2'],
  [/^this method takes (\d+) arguments? but (\d+) arguments? (?:was|were) supplied$/, 'هذه الدالة تأخذ $1 وسيط، لكن أُعطيت $2'],
  [/^expected (\d+) arguments?, found (\d+)$/, 'المتوقَّع $1 وسيط، والموجود $2'],
  [/^missing argument for parameter `(.+)`$/, 'لم تُعطَ قيمة للمعامل `$1`'],
  [/^this function has no parameter named `(.+)`$/, 'هذه الدالة ليس لها معامل اسمه `$1`'],
  [/^parameter `(.+)` is given twice$/, 'المعامل `$1` أُعطي مرتين'],
  [/^no method named `(.+)` found for (.+) `(.+)` in the current scope$/, 'لا توجد دالة `$1` على `$3`'],
  [/^no method `(.+)` on type `(.+)`$/, 'لا توجد دالة `$1` على النوع `$2`'],
  [/^no function or associated item named `(.+)` found for (.+) `(.+)` in the current scope$/, 'لا توجد دالة `$1` في `$3`'],
  [/^no field `(.+)` on type `(.+)`$/, 'لا يوجد حقل `$1` في النوع `$2`'],
  [/^struct `(.+)` has no field named `(.+)`$/, 'الـ struct `$1` ليس فيه حقل اسمه `$2`'],
  [/^missing fields? (.+) in initializer of `(.+)`$/, 'حقول ناقصة عند إنشاء `$2`: $1'],
  [/^missing structure fields:?$/, 'حقول ناقصة في الـ struct:'],
  [/^function `(.+)` is never used$/, 'الدالة `$1` غير مستخدمة'],
  [/^method `(.+)` is never used$/, 'الدالة `$1` غير مستخدمة'],
  [/^associated function `(.+)` is never used$/, 'الدالة `$1` غير مستخدمة'],
  [/^struct `(.+)` is never constructed$/, 'الـ struct `$1` لا يُنشأ أبدًا'],
  [/^enum `(.+)` is never used$/, 'الـ enum `$1` غير مستخدم'],
  [/^variant `(.+)` is never constructed$/, 'الحالة `$1` لا تُنشأ أبدًا'],
  [/^field `(.+)` is never read$/, 'الحقل `$1` لا يُقرأ أبدًا'],
  [/^fields (.+) are never read$/, 'الحقول $1 لا تُقرأ أبدًا'],
  [/^constant `(.+)` is never used$/, 'الثابت `$1` غير مستخدم'],

  // ---- variables, mutability ----
  [/^unused variable: `(.+)`$/, 'المتغير `$1` غير مستخدم'],
  [/^unused variable$/, 'متغير غير مستخدم'],
  [/^unused import: (.+)$/, 'استيراد غير مستخدم: $1'],
  [/^unused imports: (.+)$/, 'استيرادات غير مستخدمة: $1'],
  [/^variable does not need to be mutable$/, 'المتغير لا يحتاج `mut`'],
  [/^value assigned to `(.+)` is never read$/, 'القيمة المعطاة لـ `$1` لا تُقرأ أبدًا'],
  [/^cannot assign twice to immutable variable `(.+)`$/, 'لا يمكن تغيير `$1` مرتين: المتغير ليس `mut`'],
  [/^cannot mutate immutable variable `(.+)`$/, 'لا يمكن تغيير `$1`: المتغير ليس `mut`'],
  [/^cannot borrow `(.+)` as mutable, as it is not declared as mutable$/, 'لا يمكن استعارة `$1` للتعديل: لم يُعرَّف بـ `mut`'],
  [/^cannot borrow `(.+)` as mutable, as it is behind a `&` reference$/, 'لا يمكن تعديل `$1`: هو خلف مرجع للقراءة فقط `&`'],
  [/^cannot assign to `(.+)`, which is behind a `&` reference$/, 'لا يمكن تغيير `$1`: هو خلف مرجع للقراءة فقط `&`'],
  [/^used binding `(.+)` isn't initialized$/, 'المتغير `$1` استُخدم قبل أن تُعطى له قيمة'],
  [/^unused `(.+)` that must be used$/, 'نتيجة `$1` يجب استخدامها'],
  [/^unused result of type `(.+)`$/, 'نتيجة من النوع `$1` غير مستخدمة'],
  [/^unreachable statement$/, 'هذا السطر لن يُنفَّذ أبدًا'],
  [/^unreachable expression$/, 'هذا التعبير لن يُنفَّذ أبدًا'],
  [/^unreachable pattern$/, 'هذه الحالة لن تُطابق أبدًا'],

  // ---- ownership and borrowing ----
  [/^use of moved value: `(.+)`$/, 'استخدام `$1` بعد نقله: القيمة لم تعد هنا'],
  [/^borrow of moved value: `(.+)`$/, 'استعارة `$1` بعد نقله: القيمة لم تعد هنا'],
  [/^value moved here$/, 'هنا نُقلت القيمة'],
  [/^value moved here, in previous iteration of loop$/, 'نُقلت القيمة هنا في دورة سابقة من الحلقة'],
  [/^value used here after move$/, 'هنا استُخدمت القيمة بعد نقلها'],
  [/^value borrowed here after move$/, 'هنا استُعيرت القيمة بعد نقلها'],
  [/^move occurs because `(.+)` has type `(.+)`, which does not implement the `Copy` trait$/, 'حدث النقل لأن `$1` من النوع `$2`، وهو لا يُنسخ تلقائيًا (لا يطبِّق `Copy`)'],
  [/^cannot borrow `(.+)` as mutable because it is also borrowed as immutable$/, 'لا يمكن استعارة `$1` للتعديل وهو مستعار للقراءة'],
  [/^cannot borrow `(.+)` as immutable because it is also borrowed as mutable$/, 'لا يمكن استعارة `$1` للقراءة وهو مستعار للتعديل'],
  [/^cannot borrow `(.+)` as mutable more than once at a time$/, 'لا يمكن استعارة `$1` للتعديل أكثر من مرة في نفس الوقت'],
  [/^first mutable borrow occurs here$/, 'أول استعارة للتعديل هنا'],
  [/^second mutable borrow occurs here$/, 'ثاني استعارة للتعديل هنا'],
  [/^first borrow later used here$/, 'الاستعارة الأولى ما زالت مستخدمة هنا'],
  [/^immutable borrow occurs here$/, 'استعارة للقراءة هنا'],
  [/^mutable borrow occurs here$/, 'استعارة للتعديل هنا'],
  [/^immutable borrow later used here$/, 'الاستعارة للقراءة ما زالت مستخدمة هنا'],
  [/^mutable borrow later used here$/, 'الاستعارة للتعديل ما زالت مستخدمة هنا'],
  [/^`(.+)` does not live long enough$/, '`$1` لا يعيش مدة كافية'],
  [/^borrowed value does not live long enough$/, 'القيمة المستعارة لا تعيش مدة كافية'],
  [/^`(.+)` dropped here while still borrowed$/, 'هنا انتهى `$1` وهو ما زال مستعارًا'],
  [/^cannot move out of `(.+)` which is behind a shared reference$/, 'لا يمكن نقل `$1`: هو خلف مرجع مشترك `&`'],
  [/^cannot move out of `(.+)` which is behind a mutable reference$/, 'لا يمكن نقل `$1`: هو خلف مرجع `&mut`'],
  [/^cannot return reference to local variable `(.+)`$/, 'لا يمكن إرجاع مرجع إلى المتغير المحلي `$1`'],
  [/^cannot return value referencing local variable `(.+)`$/, 'لا يمكن إرجاع قيمة تشير إلى المتغير المحلي `$1`'],
  [/^returns a value referencing data owned by the current function$/, 'ترجع قيمة تشير إلى بيانات تملكها هذه الدالة (ستُحذف عند خروجها)'],
  [/^missing lifetime specifier$/, 'يلزم توضيح من أين يُستعار المرجع: اكتب `&{x} T` أو `_a`'],
  [/^lifetime may not live long enough$/, 'المرجع قد لا يعيش مدة كافية'],
  [/^closure may outlive the current function, but it borrows `(.+)`, which is owned by the current function$/, 'الإغلاق (closure) قد يعيش أطول من الدالة لكنه يستعير `$1` منها: استخدم `move`'],

  // ---- patterns and control flow ----
  [/^non-exhaustive patterns: (.+) not covered$/, 'فروع `match` لا تغطي كل الحالات: $1 غير مغطّاة'],
  [/^missing match arm: (.+) not covered$/, 'فرع ناقص في `match`: $1 غير مغطّى'],
  [/^missing match arm: (.+) and (.+) not covered$/, 'فروع ناقصة في `match`: $1 و$2 غير مغطّاة'],
  [/^`break` outside of a loop.*$/, '`break` خارج حلقة'],
  [/^`continue` outside of a loop.*$/, '`continue` خارج حلقة'],
  [/^remove this semicolon( to return this value)?$/, 'احذف الفاصلة المنقوطة لترجع هذه القيمة'],
  [/^implicitly returns `\(\)` as its body has no tail or `return` expression$/, 'الدالة لا ترجع شيئًا: آخر سطر فيها ينتهي بـ `;`'],

  // ---- syntax ----
  [/^Syntax Error: (.+)$/, 'خطأ في الكتابة: $1'],
  [/^expected `(.+)`, found `(.+)`$/, 'المتوقَّع `$1`، لكن الموجود `$2`'],
  [/^expected one of (.+), found (.+)$/, 'المتوقَّع واحد من $1، لكن الموجود $2'],
  [/^expected an item$/, 'المتوقَّع تعريف (fn أو struct أو use…)'],
  [/^expected expression, found (.+)$/, 'المتوقَّع تعبير، لكن الموجود $1'],
  [/^expected SEMICOLON$/, 'ينقص `;`'],
  [/^expected expression$/, 'ينقص تعبير هنا'],
  [/^expected type$/, 'ينقص نوع هنا'],
  [/^expected identifier$/, 'ينقص اسم هنا'],
  [/^expected R_CURLY$/, 'ينقص `}`'],
  [/^expected R_PAREN$/, 'ينقص `)`'],
  [/^expected COMMA$/, 'تنقص فاصلة `,`'],
  [/^expected (.+)$/, 'المتوقَّع $1'],
  [/^this file contains an unclosed delimiter$/, 'في الملف قوس مفتوح لم يُغلق'],
  [/^unclosed delimiter$/, 'قوس مفتوح لم يُغلق'],
  [/^unexpected closing delimiter: `(.+)`$/, 'قوس إغلاق `$1` في غير مكانه'],
  [/^mismatched closing delimiter: `(.+)`$/, 'قوس الإغلاق `$1` لا يطابق قوس الفتح'],
  [/^unknown start of token: (.+)$/, 'رمز غير معروف: $1'],
  [/^unterminated double quote string$/, 'نص بين علامتي تنصيص لم يُغلق'],

  // ---- traits and impls ----
  [/^not all trait items implemented, missing: (.+)$/, 'لم تُكتب كل دوال الـ trait، الناقص: $1'],
  [/^conflicting implementations of trait `(.+)` for type `(.+)`$/, 'الـ trait `$1` مطبَّق مرتين على `$2`'],
  [/^the size for values of type `(.+)` cannot be known at compilation time$/, 'حجم القيم من النوع `$1` غير معروف وقت الترجمة'],
  [/^`(.+)` is a private (field|function|method|struct)$/, '`$1` خاص (private)'],
  [/^(field|function|struct|method) `(.+)` is private$/, '`$2` خاص (private)'],

  // ---- notes and help ----
  [/^unexpected argument #(\d+)(?: of type `(.+)`)?$/, 'وسيط زائد رقم $1'],
  [/^unexpected argument of type `(.+)`$/, 'وسيط زائد من النوع `$1`'],
  [/^remove the extra arguments?$/, 'احذف الوسيط الزائد'],
  [/^argument #(\d+) of type `(.+)` is missing$/, 'ينقص الوسيط رقم $1 من النوع `$2`'],
  [/^an argument of type `(.+)` is missing$/, 'ينقص وسيط من النوع `$1`'],
  [/^provide the arguments?$/, 'أعطِ الوسيط الناقص'],
  [/^(function|method|associated function|struct|enum|trait|field|variant|constant|macro) defined here$/, 'معرَّف هنا'],
  [/^value defined here$/, 'القيمة معرَّفة هنا'],
  [/^expected `(.+)` because of return type$/, 'المتوقَّع `$1` بسبب نوع الإرجاع'],
  [/^expected `(.+)` because of default return type$/, 'المتوقَّع `$1` لأن الدالة لا ترجع شيئًا'],
  [/^a local variable with a similar name exists: `(.+)`$/, 'يوجد متغير محلي باسم مشابه: `$1`'],
  [/^a function with a similar name exists: `(.+)`$/, 'توجد دالة باسم مشابه: `$1`'],
  [/^a (?:struct|type|trait|method|field) with a similar name exists: `(.+)`$/, 'يوجد اسم مشابه: `$1`'],
  [/^there is a method `(.+)` with a similar name$/, 'توجد دالة باسم مشابه: `$1`'],
  [/^consider removing this semicolon$/, 'جرّب حذف هذه الفاصلة المنقوطة'],
  [/^try adding a semicolon:? `;`?$/, 'جرّب إضافة فاصلة منقوطة `;`'],
  [/^consider using a semicolon here$/, 'جرّب إضافة فاصلة منقوطة هنا'],
  [/^cannot borrow as mutable$/, 'لا يمكن الاستعارة للتعديل'],
  [/^this function takes (\d+) parameters?$/, 'هذه الدالة تأخذ $1 معامل'],
  [/^`(.+)` is a function, perhaps you wish to call it$/, '`$1` دالة، ربما تقصد استدعاءها `()`'],
  [/^you might have meant to (.+)$/, 'ربما تقصد: $1'],
  [/^for more information about this error, try `(?:rodetc|rustc) --explain (E\d+)`\.?$/, 'لمزيد من الشرح جرّب `rodetc --explain $1`'],
  [/^consider borrowing here: `(.+)`$/, 'جرّب الاستعارة: `$1`'],
  [/^consider borrowing here$/, 'جرّب الاستعارة هنا'],
  [/^consider cloning the value if the performance cost is acceptable$/, 'يمكنك نسخ القيمة بـ `clone()` إن كانت التكلفة مقبولة'],
  [/^consider changing this to be mutable$/, 'اجعله قابلًا للتعديل (`mut`)'],
  [/^consider making this binding mutable(?:: (.+))?$/, 'اجعل المتغير قابلًا للتعديل: `mut`'],
  [/^consider importing (?:this|one of these) (?:items?|struct|function|trait|enum)s?$/, 'جرّب استيراده بـ `use`'],
  [/^help: (.+)$/, 'مساعدة: $1'],
  [/^required by a bound in `(.+)`$/, 'مطلوب بسبب شرط في `$1`'],
  [/^`#\[warn\((.+)\)\]` on by default$/, 'تحذير `$1` مفعّل افتراضيًا'],
  [/^first assignment to `(.+)`$/, 'أول قيمة لـ `$1` هنا'],
  [/^cannot assign twice to immutable variable$/, 'لا يمكن التغيير مرتين: المتغير ليس `mut`'],
];

/** Quick fixes and refactorings (Ctrl+.). */
const ACTIONS: Rule[] = [
  [/^Import `(.+)`$/, 'استيراد `$1`'],
  [/^Import (.+)$/, 'استيراد $1'],
  [/^Qualify `(.+)`$/, 'كتابة المسار الكامل لـ `$1`'],
  [/^Qualify as `(.+)`$/, 'كتابة المسار الكامل `$1`'],
  [/^Add missing match arms$/, 'إضافة فروع `match` الناقصة'],
  [/^Fill struct fields$/, 'تعبئة حقول الـ struct'],
  [/^Fill match arms$/, 'تعبئة فروع `match`'],
  [/^Implement missing members$/, 'كتابة دوال الـ trait الناقصة'],
  [/^Implement default members$/, 'كتابة الدوال الافتراضية للـ trait'],
  [/^Change it to be mutable$/, 'جعله قابلًا للتعديل (`mut`)'],
  [/^Remove unnecessary `mut`$/, 'حذف `mut` غير اللازم'],
  [/^Remove this `mut`$/, 'حذف `mut`'],
  [/^Remove unused import$/, 'حذف الاستيراد غير المستخدم'],
  [/^Remove all unused imports$/, 'حذف كل الاستيرادات غير المستخدمة'],
  [/^Remove unused imports$/, 'حذف الاستيرادات غير المستخدمة'],
  [/^Remove this semicolon$/, 'حذف الفاصلة المنقوطة'],
  [/^Insert `(.+)`$/, 'إدراج `$1`'],
  [/^Insert explicit type `(.+)`$/, 'كتابة النوع صراحة `$1`'],
  [/^Add explicit type `(.+)`$/, 'كتابة النوع صراحة `$1`'],
  [/^Rename to (.+)$/, 'إعادة التسمية إلى $1'],
  [/^Wrap in `(.+)`$/, 'تغليف بـ `$1`'],
  [/^Wrap return type in (.+)$/, 'تغليف نوع الإرجاع بـ $1'],
  [/^Add reference here$/, 'إضافة `&` هنا'],
  [/^Add `&` reference$/, 'إضافة مرجع `&`'],
  [/^Add `&mut` reference$/, 'إضافة مرجع `&mut`'],
  [/^Add this function's return type$/, 'إضافة نوع الإرجاع للدالة'],
  [/^Add return type$/, 'إضافة نوع الإرجاع'],
  [/^Extract into variable$/, 'استخراج إلى متغير'],
  [/^Extract into function$/, 'استخراج إلى دالة'],
  [/^Extract into constant$/, 'استخراج إلى ثابت'],
  [/^Extract into static$/, 'استخراج إلى static'],
  [/^Extract into module$/, 'استخراج إلى وحدة'],
  [/^Extract type as type alias$/, 'استخراج النوع كاسم مستعار'],
  [/^Inline variable$/, 'استبدال المتغير بقيمته'],
  [/^Inline call$/, 'استبدال الاستدعاء بجسم الدالة'],
  [/^Inline `(.+)`$/, 'استبدال `$1` بقيمته'],
  [/^Generate `new`$/, 'إنشاء دالة `new`'],
  [/^Generate `Default` impl$/, 'إنشاء `impl Default`'],
  [/^Generate `Default` impl from a `new` fn$/, 'إنشاء `impl Default` من `new`'],
  [/^Generate impl$/, 'إنشاء `impl`'],
  [/^Generate trait impl$/, 'إنشاء `impl` لـ trait'],
  [/^Generate a getter method$/, 'إنشاء دالة قراءة (getter)'],
  [/^Generate a mut getter method$/, 'إنشاء دالة قراءة للتعديل'],
  [/^Generate a setter method$/, 'إنشاء دالة تغيير (setter)'],
  [/^Generate getter$/, 'إنشاء دالة قراءة (getter)'],
  [/^Generate setter$/, 'إنشاء دالة تغيير (setter)'],
  [/^Generate `(.+)` function$/, 'إنشاء الدالة `$1`'],
  [/^Generate `(.+)` method$/, 'إنشاء الدالة `$1`'],
  [/^Generate function `(.+)`$/, 'إنشاء الدالة `$1`'],
  [/^Generate documentation template$/, 'إنشاء قالب توثيق'],
  [/^Add `#\[derive\]`$/, 'إضافة `#[derive]`'],
  [/^Add `#\[derive\((.+)\)\]`$/, 'إضافة `#[derive($1)]`'],
  [/^Convert to guarded return$/, 'تحويل إلى إرجاع مبكر'],
  [/^Replace `if let` with `match`$/, 'استبدال `if let` بـ `match`'],
  [/^Replace `match` with `if let`$/, 'استبدال `match` بـ `if let`'],
  [/^Replace `(.+)` with `(.+)`$/, 'استبدال `$1` بـ `$2`'],
  [/^Replace for loop with `for_each`$/, 'استبدال حلقة `for` بـ `for_each`'],
  [/^Replace this `for_each` with a `for` loop$/, 'استبدال `for_each` بحلقة `for`'],
  [/^Add braces$/, 'إضافة أقواس `{}`'],
  [/^Remove braces$/, 'حذف الأقواس `{}`'],
  [/^Flip comma$/, 'تبديل الطرفين حول الفاصلة'],
  [/^Flip binary expression$/, 'تبديل طرفي العملية'],
  [/^Apply De Morgan's law$/, 'تطبيق قانون دي مورغان'],
  [/^Invert if$/, 'عكس `if`'],
  [/^Merge imports$/, 'دمج الاستيرادات'],
  [/^Split import$/, 'فصل الاستيراد'],
  [/^Sort items by trait definition$/, 'ترتيب العناصر حسب الـ trait'],
  [/^Sort (.+)$/, 'ترتيب $1'],
  [/^Change visibility to (.+)$/, 'تغيير الوصول إلى $1'],
  [/^Make `(.+)` public$/, 'جعل `$1` عامًا (pub)'],
  [/^Convert to named struct$/, 'تحويل إلى struct بأسماء حقول'],
  [/^Convert to tuple struct$/, 'تحويل إلى struct بلا أسماء حقول'],
  [/^Convert `(.+)` to `(.+)`$/, 'تحويل `$1` إلى `$2`'],
  [/^Convert (.+) to (.+)$/, 'تحويل $1 إلى $2'],
  [/^Change type to `(.+)`$/, 'تغيير النوع إلى `$1`'],
  [/^Change return type to `(.+)`$/, 'تغيير نوع الإرجاع إلى `$1`'],
  [/^Use `(.+)` instead$/, 'استخدم `$1` بدلًا منه'],
  [/^Remove dbg!\(\)$/, 'حذف `dbg!()`'],
  [/^Remove `(.+)`$/, 'حذف `$1`'],
  [/^Remove (.+)$/, 'حذف $1'],
  [/^Add (.+)$/, 'إضافة $1'],
];

/** Lenses above code: ▶ Run, implementations, references. */
const LENSES: Rule[] = [
  [/^▶︎? Run $/, '▶ تشغيل'],
  [/^▶︎? Run (.+ )?Tests$/, '▶ تشغيل الاختبارات $1'],
  [/^▶︎? Run (.+ )?Test$/, '▶ تشغيل الاختبار $1'],
  [/^▶︎? Run (.+ )?Doctest$/, '▶ تشغيل مثال التوثيق $1'],
  [/^▶︎? Run (.+ )?Bench$/, '▶ تشغيل قياس السرعة $1'],
  [/^▶︎? Run (.+)$/, '▶ تشغيل $1'],
  [/^⚙︎? Debug$/, '⚙ تتبُّع'],
  [/^1 implementation$/, 'تطبيق واحد (impl)'],
  [/^(\d+) implementations$/, '$1 تطبيقات (impl)'],
  [/^1 reference$/, 'مرجع واحد'],
  [/^(\d+) references$/, '$1 مراجع'],
];

function apply(rules: Rule[], text: string) {
  for (const [pattern, replacement] of rules) {
    if (pattern.test(text)) return text.replace(pattern, replacement).replace(/\s+$/, '');
  }
  return null;
}

/** A compiler or analyzer message in the interface language. Only the
 * first line is matched; the rest (lists, notes) is kept. */
export function translateDiagnostic(message: string): string {
  if (!arabic) return message;
  const [first, ...rest] = message.split('\n');
  const done = apply(DIAGNOSTICS, first.trim());
  if (done === null) return message;
  return [done, ...rest.map((line) => apply(DIAGNOSTICS, line.trim()) ?? line)].join('\n');
}

export function translateAction(title: string): string {
  if (!arabic) return title;
  return apply(ACTIONS, title) ?? apply(DIAGNOSTICS, title) ?? title;
}

export function translateLens(title: string): string {
  if (!arabic) return title;
  return apply(LENSES, title) ?? title;
}

/** The speed tips from rodet-analyzer (fixed sentences, in ar.ts). */
export function translateTip(text: string): string {
  return t(text);
}
