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
  "version": 4,
  "last_updated_at": 1760000000,
  "daily_actions": {
    "day": 20370,
    "feed_count": 1,
    "play_count": 0
  },
  "care_stats": {
    "feed_total": 4,
    "play_total": 2
  },
  "pet": {
    "name": "Mochi",
    "stage": "Stage 1",
    "evolution": "Fluffy",
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

`last_updated_at` はUnix epoch秒で保存する。

Phase 3では、`version: 1` の既存saveに `last_updated_at` が存在しない場合、読み込み時に現在時刻を設定して `version: 2` として保存し直す。

`daily_actions.day` はUnix epoch日数で保存する。

Phase 4では、`version: 1` または `version: 2` の既存saveに `daily_actions` が存在しない場合、読み込み時に現在日で初期化して `version: 3` として保存し直す。

`care_stats` は進化判定に使う累計行動数を保存する。

Phase 5では、`version: 1` から `version: 3` の既存saveに `care_stats` や `pet.stage` / `pet.evolution` が存在しない場合、読み込み時に補完して `version: 4` として保存し直す。

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
