#!/usr/bin/env bash
# Claude desktop extension (.mcpb) for عرض on macOS: double-clicking it opens Claude's own
# «Install» window, which adds عرض's MCP server to Claude without editing any file.
#
# The extension holds only its description and the عرض logo: it runs the MCP server inside the
# installed app (/Applications/عرض.app/Contents/MacOS/ard-cli), so the server updates with the
# app and the file stays a few kilobytes. On other systems, and for other AI apps, عرض's own
# File › About › Connect to AI Apps window sets it up.
#
# usage: packaging/mcpb/make-mcpb.sh OUT_DIR VERSION
set -euo pipefail
OUT="${1:?out dir}"
VERSION="${2:?version}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
# Claude recommends a 512 px icon: scaled down with macOS's sips where it exists.
cp "$ROOT/assets/app-icon/ard-macos-1024.png" "$WORK/icon.png"
if command -v sips >/dev/null; then sips -Z 512 "$WORK/icon.png" >/dev/null; fi
cat > "$WORK/manifest.json" <<JSON
{
  "manifest_version": "0.3",
  "name": "ard",
  "display_name": "عرض (Ard)",
  "version": "$VERSION",
  "description": "أنشئ عروضًا تقديمية ونسّقها بكلامك في عرض، برنامج العروض العربي.",
  "long_description": "عرض برنامج عروض تقديمية عربي مبني على DeckCraft. بعد تثبيت هذه الإضافة ينشئ Claude الشرائح ويكتب النصوص ويضيف الأشكال والصور والتنسيق ويحفظ العرض بصيغة PowerPoint أو PDF عندما تطلب منه ذلك. يحتاج عرض مثبتًا في مجلد التطبيقات.\n\nArd is an Arabic presentation app based on DeckCraft. With this extension Claude can build slides, write text, add shapes and pictures, and save as PowerPoint or PDF. Needs Ard installed in Applications.",
  "author": { "name": "Nasser Alqahtani", "url": "https://github.com/nasserfq/ard" },
  "homepage": "https://github.com/nasserfq/ard",
  "repository": { "type": "git", "url": "https://github.com/nasserfq/ard.git" },
  "license": "MIT OR Apache-2.0",
  "icon": "icon.png",
  "server": {
    "type": "binary",
    "entry_point": "/Applications/عرض.app/Contents/MacOS/ard-cli",
    "mcp_config": {
      "command": "/Applications/عرض.app/Contents/MacOS/ard-cli",
      "args": ["mcp"]
    }
  },
  "keywords": ["presentation", "slides", "pptx", "arabic", "عرض"],
  "compatibility": { "platforms": ["darwin"] }
}
JSON
python3 -m json.tool "$WORK/manifest.json" >/dev/null
mkdir -p "$OUT"
rm -f "$OUT/ard-claude-extension-macos.mcpb"
(cd "$WORK" && zip -q -X "ard-claude-extension-macos.mcpb" manifest.json icon.png)
mv "$WORK/ard-claude-extension-macos.mcpb" "$OUT/"
echo "$OUT/ard-claude-extension-macos.mcpb"
