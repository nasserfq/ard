<p align="center">
  <img src="assets/app-icon/hicolor/128x128/apps/ai.storyteller.deckcraft.png" width="128" height="128" alt="عرض">
</p>

<h1 align="center">عرض</h1>

<p align="center">
  <b>برنامج عروض تقديمية عربي، مجاني ومفتوح المصدر</b><br>
  An Arabic presentation app, free and open source
</p>

<p align="center">
  <a href="https://github.com/nasserfq/ard/releases/latest"><b>⬇︎ تنزيل · Download</b></a>
  &nbsp;·&nbsp; macOS &nbsp;·&nbsp; Windows &nbsp;·&nbsp; Linux
</p>

---

<div dir="rtl">

## عن عرض

عرض برنامج عروض تقديمية مصمم للعربية أولًا: الواجهة عربية ومن اليمين إلى اليسار، والعروض الجديدة تُكتب بالعربية ومن اليمين إلى اليسار.

- **واجهة عربية** من اليمين إلى اليسار، مع خيار الإنجليزية.
- **شرائح من اليمين إلى اليسار** افتراضيًا في العروض الجديدة.
- **نسق وقوالب وأشكال وصور وجداول ومخططات وانتقالات وحركات**، وعرض الشرائح بملء الشاشة.
- **ملفات PowerPoint** ‏(‎.pptx‎) فتحًا وحفظًا، والتصدير إلى PDF.

## التثبيت

| النظام | الملف | الخطوات |
|---|---|---|
| macOS ‏(Apple Silicon وIntel، الإصدار 11 فأحدث) | `ard-…-macos-universal.dmg` | افتح الملف واسحب عرض إلى «التطبيقات». موقّع وموثَّق من Apple. |
| Windows | `ard-…-windows-x64-portable.zip` | فك الضغط وشغّل `Ard.exe`. |
| Linux | `ard-…-linux-x86_64.tar.gz` | فك الضغط وشغّل `ard`. |

التحديث تلقائي: عند صدور إصدار جديد يعرض عرض «تحديث وإعادة تشغيل»، وتبقى إعداداتك وعروضك كما هي.

## ما أضافه ناصر القحطاني إلى DeckCraft

- الواجهة العربية افتراضيًا، مع ترجمة عربية للواجهة.
- الاتجاه من اليمين إلى اليسار في الواجهة كلها، وفي الشرائح الجديدة.
- خط Noto Sans Arabic بحجم مريح للقراءة.
- الاسم والشعار، وتطبيق Mac موقّع وموثَّق لأجهزة Apple Silicon وIntel، وتنزيلات Windows وLinux، والتحديث التلقائي بالعربية.

## شكر

عرض مبني على [DeckCraft](https://github.com/storytold/deckcraft)، مع الشكر لفريق ArtCraft ومجتمع DeckCraft على عملهم الأصلي. عرض مشروع مستقل، وليس من إنتاج فريق ArtCraft ولا برعايته.

</div>

---

## About

Ard (عرض) is an Arabic-first presentation app by Nasser Alqahtani: an Arabic, right-to-left
interface (English available), right-to-left Arabic slides by default in new presentations, and
PowerPoint `.pptx` files.

Nasser's additions to DeckCraft: the Arabic interface as the default with its Arabic catalog
(`i18n/locales/ar`), the right-to-left interface and new slides, the Noto Sans Arabic interface
font, the name and logo, the signed universal Mac app, Windows and Linux downloads, and Arabic
in-app updates.

It is based on [DeckCraft](https://github.com/storytold/deckcraft) by the ArtCraft team. Ard is an
independent project, not made, sponsored or endorsed by the ArtCraft team.

## Build from source

Install [Rust](https://rustup.rs). The Arabic catalog is in this repository (`i18n/`); the interface
fonts, including Noto Sans Arabic, come from [craft-fonts](https://github.com/storytold/craft-fonts):

```sh
git clone https://github.com/storytold/craft-fonts ../craft-fonts
(cd ../craft-fonts && git checkout 37827adc41fd165f9a9234ac40f94dbad353aa69)
CRAFT_FONTS_DIR="$(cd ../craft-fonts && pwd)" cargo run --release -p deckcraft
```

`CRAFT_FONTS_DIR` must be an absolute path.

Releases: pushing to the `release` branch builds, signs and publishes the downloads
(`.github/workflows/release.yml`).

## License

MIT OR Apache-2.0, like DeckCraft: see [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE),
[NOTICE](NOTICE) and [ATTRIBUTION.md](ATTRIBUTION.md).
