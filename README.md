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

## التحكم بعرض من الذكاء الاصطناعي (MCP)

عرض جاهز للذكاء الاصطناعي: يأتي مع خادم [MCP](https://modelcontextprotocol.io)، وهو معيار مفتوح تستخدمه تطبيقات الذكاء الاصطناعي لتشغيل البرامج. بعد ربطه تطلب بكلامك، مثلًا «اعمل عرضًا من خمس شرائح عن مشروعنا، بعنوان في كل شريحة ونقاط مختصرة، واحفظه PowerPoint»، فينفّذه الذكاء الاصطناعي في عرض خطوة بخطوة: إضافة الشرائح والنصوص والأشكال والصور والتنسيق وقراءة العرض والحفظ بصيغة PowerPoint أو PDF.

يعمل مع أي تطبيق يدعم MCP، مثل Claude ‏(التطبيق وClaude Code)، وCodex من OpenAI، وCursor، وVS Code مع Copilot، وGemini CLI، وLM Studio ‏(نماذج تعمل على جهازك دون إنترنت). الخادم مجاني ويعمل على جهازك، أما الذكاء الاصطناعي فمن التطبيق الذي تختاره وبحسب اشتراكك فيه.

**الطريقة الأسهل، بنقرة:** في عرض افتح «ملف» ← «حول» ← «ربط مع الذكاء الاصطناعي» (أو اكتبها في لوحة الأوامر). تظهر تطبيقات الذكاء الاصطناعي المثبتة على جهازك، مثل Claude وClaude Code وCodex وCursor وVS Code وWindsurf وGemini CLI وLM Studio، فاضغط «ربط» بجانب التطبيق ثم أعد تشغيله. ولأي تطبيق آخر يدعم MCP اضغط «نسخ الإعداد» والصقه في إعداداته.

اذكر **ard** في طلبك، مثل «استخدم أدوات ard لـ…»، لأن «عرض» كلمة عامة أيضًا، فقد يفتح بعض التطبيقات (مثل Codex) برنامج PowerPoint بدلًا منه.

على Mac يمكنك أيضًا تنزيل ملف `ard-claude-extension-macos.mcpb` من [صفحة الإصدارات](https://github.com/nasserfq/ard/releases/latest) وفتحه، فيعرض تطبيق Claude نافذة «تثبيت» ويضيف عرض بنقرة.

**الإعداد اليدوي (إن أردت):**

**1. مكان الخادم:** يُثبَّت مع عرض ويُحدَّث معه.

| النظام | المسار |
|---|---|
| macOS | `/Applications/عرض.app/Contents/MacOS/ard-cli` |
| Windows | `ard-cli.exe` في مجلد عرض، بجانب `Ard.exe` |
| Linux | `ard-cli` في مجلد عرض، بجانب `ard` |

**2. أضفه إلى تطبيق الذكاء الاصطناعي:**

- **تطبيق Claude:** الإعدادات ← «Developer» ← «Edit Config»، ثم أضف ما يلي إلى الملف `claude_desktop_config.json` وأعد تشغيل Claude:

<div dir="ltr">

```json
{
  "mcpServers": {
    "ard": {
      "command": "/Applications/عرض.app/Contents/MacOS/ard-cli",
      "args": ["mcp"]
    }
  }
}
```

</div>

- في Windows ضع مسار `ard-cli.exe` بدل المسار السابق، مع مضاعفة الشرطة المائلة، مثل `"C:\\Users\\اسمك\\Ard\\ard-cli.exe"`.
- **Cursor وGemini CLI وLM Studio** وأغلب التطبيقات الأخرى: الإعداد نفسه (`mcpServers`) في ملف MCP الخاص بكل تطبيق (`~/.cursor/mcp.json`، ‏`~/.gemini/settings.json`، ‏`mcp.json` في LM Studio).
- **Claude Code:** <code dir="ltr">claude mcp add ard -- "/Applications/عرض.app/Contents/MacOS/ard-cli" mcp</code>
- **Codex:** <code dir="ltr">codex mcp add ard -- "/Applications/عرض.app/Contents/MacOS/ard-cli" mcp</code>

بهذا يعمل الذكاء الاصطناعي على الملفات في الخلفية دون نافذة، ثم يحفظها حيث تطلب.

**التحكم بنافذة عرض المفتوحة (اختياري):** لتشاهد الذكاء الاصطناعي يعمل في النافذة نفسها، شغّل عرض مع منفذ تحكم، واجعل `args` هي <code dir="ltr">["mcp", "--connect", "7979"]</code>:

<div dir="ltr">

```sh
"/Applications/عرض.app/Contents/MacOS/DeckCraft" --control 7979   # macOS
Ard.exe --control 7979                                   # Windows
./ard --control 7979                                     # Linux
```

</div>

منفذ التحكم محلي فقط (127.0.0.1)، فلا يصل إليه أحد من الشبكة. أغلق عرض أو شغّله دون `--control` حين لا تحتاجه.

## ما أضافه ناصر القحطاني إلى DeckCraft

- الواجهة العربية افتراضيًا، مع ترجمة عربية للواجهة.
- الاتجاه من اليمين إلى اليسار في الواجهة كلها، وفي الشرائح الجديدة.
- خط Noto Sans Arabic بحجم مريح للقراءة.
- خادم MCP ضمن التنزيلات لتتحكم تطبيقات الذكاء الاصطناعي بعرض، ونافذة «ربط مع الذكاء الاصطناعي» تربطه بتطبيقات الذكاء الاصطناعي بنقرة، وإضافة Claude ‏(`.mcpb`).
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

## Use it from AI apps (MCP)

Ard ships an [MCP](https://modelcontextprotocol.io) server, so Claude and any other MCP app
(Codex, Cursor, VS Code, Gemini CLI, LM Studio…) can add slides, text, shapes and pictures, format, read and save presentations. It is installed and updated with
the app: `/Applications/عرض.app/Contents/MacOS/ard-cli` on macOS, `ard-cli.exe` beside `Ard.exe` on Windows, `ard-cli` beside `ard`
on Linux. File ▸ About ▸ Connect to AI Apps adds it with one click to Claude, Claude Code, Codex, Cursor,
VS Code, Windsurf, Gemini CLI and LM Studio, and on macOS the `ard-claude-extension-macos.mcpb`
release download installs it into Claude. By hand, add it to your client as that command with the argument `mcp` (for example
`claude mcp add ard -- "/Applications/عرض.app/Contents/MacOS/ard-cli" mcp`). To drive the open window instead, start Ard with
`--control 7979` and use `mcp --connect 7979`.

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
