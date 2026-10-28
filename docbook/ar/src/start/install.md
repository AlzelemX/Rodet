# تثبيت Rodet

## ويندوز

1. نزّل **`rodet_init.exe`** من
   [آخر إصدار](https://github.com/AlzelemX/rodet/releases/latest).
2. افتحه بضغطتين. يثبّت Rodet في `%USERPROFILE%\.rodet` ويضيفها إلى
   متغير PATH.
3. إذا كانت أدوات Visual Studio C++ Build Tools غير موجودة، يعرض عليك
   تثبيتها. وافق: برامج ويندوز تُربط بأداة الربط من مايكروسوفت.

## لينكس وماك

```sh
curl -fsSL https://github.com/AlzelemX/rodet/releases/latest/download/rodet_init.sh | sh
```

تحتاج أيضًا أداة ربط للغة C: `xcode-select --install` على الماك، أو
`build-essential` (أوبونتو/ديبيان) / `gcc` على لينكس.

## تأكد أنها تعمل

افتح طرفية **جديدة**:

```text
> rustc --version
rustc 1.101.0-dev (Rodet #19)
> cargo --version
cargo 1.100.0-beta.1 (3d7cf6e93 2026-09-25)
```

إذا ظهر `(Rodet #…)` فأنت جاهز. الرقم هو رقم البناء: الإصدار الأحدث رقمه
أكبر.

## التحديث والإزالة

- **التحديث:** شغّل المثبّت مرة أخرى.
- **الإزالة:** `rodet_init.exe --uninstall` على ويندوز، أو

  ```sh
  curl -fsSL https://github.com/AlzelemX/rodet/releases/latest/download/rodet_init.sh | sh -s -- --uninstall
  ```

## المحرر

استخدم **[DEVR+](devr.md)**، المحرر المصنوع لـ Rodet: يعرف كل الكتابة
الجديدة، ويعرض الأخطاء أثناء الكتابة، ويشغّل مشروعك بزر `F5`.

تأتي Rodet أيضًا مع **`rodet-analyzer`**، محلل اللغة الذي يستخدمه DEVR+.
المحررات الأخرى يمكنها استخدامه أيضًا: في VS Code ثبّت إضافة
*rust-analyzer* واجعل الإعداد `rust-analyzer.server.path` يشير إلى
`rodet-analyzer` في `~/.rodet/bin` (`%USERPROFILE%\.rodet\bin` على
ويندوز).
