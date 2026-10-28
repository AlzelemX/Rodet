# إضافات DEVR+

الإضافة تستطيع تغيير أي شيء في DEVR+: إضافة أوامر واختصارات، ولوحات جانبية
وسفلية، وعناصر في شريط الحالة، وصفحات، وتغيير الألوان والشكل كله، وإضافة
ميزات جديدة لمحرر الكود، والتعامل مع المحلل (analyzer) والطرفية وملفاتك.

## استخدام الإضافات

افتح **الإضافات** (`Ctrl+Shift+X`، أيقونة قطعة البازل). لكل إضافة زر تشغيل
وإيقاف، و**إعادة تحميل** و**تعديل** و**حذف**.

- **تثبيت من مجلد…**: مجلد فيه `plugin.json`.
- **إنشاء إضافة**: ينشئ إضافة جديدة من قالب ويفتحها.

> الإضافة تستطيع فعل كل ما يفعله DEVR+، ومن ذلك قراءة ملفاتك وتغييرها وتشغيل
> البرامج. ثبّت فقط الإضافات التي تثق بها.

الإضافات محفوظة في مجلد إعدادات DEVR+، داخل `plugins/`.

## كتابة إضافة

**إنشاء إضافة** يصنع مجلدًا مثل هذا:

```text
my-plugin/
├── plugin.json   تعريف الإضافة
├── main.js       الكود
├── style.css     تنسيق CSS للمحرر كله
└── devr.d.ts     الواجهة كاملة مع الشرح (للإكمال التلقائي)
```

`plugin.json`:

```json
{
  "id": "my-plugin",
  "name": { "en": "My Plugin", "ar": "إضافتي" },
  "version": "0.1.0",
  "description": { "en": "What it does.", "ar": "ماذا تفعل." },
  "main": "main.js",
  "styles": ["style.css"],
  "themes": ["themes/night.json"],
  "devr": "1"
}
```

فقط `id` مطلوب. يمكن أن تكون الإضافة ثيمًا وCSS فقط بدون كود.

ملف `main.js` يصدّر الدالة `activate` التي تستقبل المحرر باسم `devr`:

```js
// @ts-check
/** @param {import('./devr').Devr} devr */
export function activate(devr) {
  devr.addTranslations({ 'Hello!': 'مرحبًا!' });

  devr.commands.register({
    id: 'my-plugin.hello',
    title: 'My Plugin: Say Hello',
    key: 'Ctrl+Alt+H',
    run: () => devr.ui.toast(devr.t('Hello!'), 'success'),
  });

  const lines = devr.ui.addStatusItem({ icon: 'list-ordered', align: 'right' });
  devr.editor.onChange(() => lines.set(String(devr.editor.model?.getLineCount() ?? '')));
}
```

**احفظ أي ملف من ملفات الإضافة فتُعاد تلقائيًا**: عدّل، احفظ، شاهد. كل ما
أضافته الإضافة يُزال عند إيقافها أو إعادة تحميلها؛ وما تصنعه بـ Monaco مباشرة
أضفه إلى `devr.subscriptions`.

## ماذا يوجد في `devr`

| الجزء | لماذا |
|---|---|
| `devr.commands`، `devr.keys` | أوامر في لوحة الأوامر، واختصارات (حتى لأوامر DEVR+ نفسه) |
| `devr.ui` | إشعارات، نوافذ حوار، قوائم اختيار، قوائم، لوحات جانبية وسفلية، عناصر الحالة، صفحات |
| `devr.layout` | كل أجزاء النافذة (شريط الأنشطة، الشريط الجانبي، التبويبات، اللوحة، شريط الحالة) |
| `devr.theme` | ثيمات الألوان، CSS، الألوان (`setColors({ '--accent': '#2f9e44' })`) |
| `devr.editor` | محرر الكود وواجهة Monaco كاملة |
| `devr.workspace` | فتح الملفات وقراءتها وكتابتها وحفظها؛ وأحداث فتح الملفات وحفظها وإغلاقها |
| `devr.terminal` | تشغيل البرامج و`rodet` |
| `devr.lsp` | طلبات إلى rodet-analyzer، والأخطاء والتحذيرات |
| `devr.settings`، `devr.storage` | إعدادات المحرر؛ وبيانات إضافتك المحفوظة |
| `devr.backend` | كل ما يستطيعه الجزء الخلفي من DEVR+ |
| `devr.t`، `devr.addTranslations` | نصوصك بالإنجليزي والعربي |

## الثيمات

ملف الثيم يحدد ألوان واجهة المحرر (متغيرات CSS)، وإن أردت ألوان محرر الكود
والطرفية:

```json
{
  "name": { "en": "Night", "ar": "الليل" },
  "base": "dark",
  "colors": { "--bg": "#0b1622", "--accent": "#38bdf8" },
  "editor": { "rules": [{ "token": "keyword", "foreground": "38bdf8" }] },
  "terminal": { "background": "#08111b" }
}
```

بعدها يظهر في **الإعدادات ← الثيم**.

## أمثلة

في مستودع Rodet، داخل `devr/plugin-api/examples/`:

- **ocean**: ثيم وشكل بزوايا دائرية، بدون كود.
- **todos**: لوحة جانبية بكل TODO وFIXME، وأمر، وعنصر في شريط الحالة.
