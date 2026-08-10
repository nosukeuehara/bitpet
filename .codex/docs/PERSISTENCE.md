# BitPet — Persistence

**# 22. 永続化**

ローカルファイルに保存する。

DBは使用しない。

例:

macOS / Linux:

\`\`\`text
\~/.bitpet/
\`\`\`

Windows:

OS標準のAppData等を使用。

Rustでは可能ならOS標準のconfig/data directory取得ライブラリを利用する。

**---**

**# 23. 保存ファイル**

例:

\`\`\`text
\~/.bitpet/
├── save.json
└── config.json
\`\`\`

**---**

**# 24. save.json**

例:

\`\`\`json
{
  "version": 1,
  "pet": {
    "name": "Mochi",
    "level": 3,
    "experience": 24,
    "hunger": 72,
    "mood": 81,
    "energy": 64
  }
}
\`\`\`

フォーマットには必ず

\`\`\`json
"version"
\`\`\`

を持たせる。

将来のマイグレーション用。

**---**

**# 25. セーブ処理**

以下の順番を基本とする。

\`\`\`text
load
↓
migrate
↓
apply elapsed time
↓
execute action
↓
save
\`\`\`

途中でゲーム状態が破損しないようにする。

可能なら、

\`\`\`text
temporary file
↓
write
↓
rename
\`\`\`

によるatomic writeを使用する。

**---**

**# 35. Wasm版の保存**

WebではFilesystemの代わりに、

\`\`\`text
localStorage
IndexedDB
\`\`\`

などを使用できる。

したがって、

\`\`\`rust
trait GameRepository
\`\`\`

として保存処理を抽象化できる設計が望ましい。

**---**
