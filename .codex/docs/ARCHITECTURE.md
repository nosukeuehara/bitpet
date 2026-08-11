# BitPet — Architecture

**# 2. 技術方針**

**## 2.1 使用言語**

Rust

**## 2.2 最初のターゲット**

CLIアプリケーション

対応対象:

\- macOS
\- Linux
\- Windows

将来的に以下にも展開可能な設計とする。

\- WebAssembly
\- Web UI
\- GitHub Releases
\- package manager経由の配布

**## 2.3 基本原則**

ゲームロジックとUIを分離する。

CLI固有処理をゲームロジックへ混入させない。

以下の構造を基本とする。

\`\`\`text
CLI
 ↓
Application
 ↓
Domain
 ↓
Storage / Time / Random
\`\`\`

Domain層は可能な限りPure Rustで実装する。

将来的なWasm版でもDomain層を再利用できる状態を維持する。

**---**

**# 3. ディレクトリ構成**

推奨構成:

\`\`\`text
bitpet/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   │
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── commands.rs
│   │   └── renderer.rs
│   │
│   ├── application/
│   │   ├── mod.rs
│   │   ├── service.rs
│   │   └── result.rs
│   │
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── pet.rs
│   │   ├── monster.rs
│   │   ├── status.rs
│   │   ├── action.rs
│   │   ├── evolution.rs
│   │   ├── expedition.rs
│   │   ├── report.rs
│   │   └── clock.rs
│   │
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   ├── storage.rs
│   │   ├── filesystem.rs
│   │   └── random.rs
│   │
│   └── ascii/
│       ├── mod.rs
│       ├── pets.rs
│       └── monsters/
│
└── tests/
    ├── lifecycle.rs
    └── evolution.rs
\`\`\`

MVP段階では多少統合してもよい。

ただし、

\`\`\`text
domain
CLI
永続化
\`\`\`

の3つは分離すること。

**---**

**# 3.1 Monster Domain**

v0.2.0ではMonster DomainをDomain層へ追加する。

現在の責務:

\`\`\`text
src/domain/monster.rs
  SpeciesId
  MonsterFamily
  MonsterDefinition
  Monster catalog
  Monster evolution tree
  Legacy evolution mapping

src/domain/evolution.rs
  GrowthStage (Egg / Baby / Stage1 / Stage2 / Final)
  EvolutionEvent
  PendingEvolution

src/domain/hatching.rs
  HatchingState
  deterministic hatch boundary

src/domain/pet.rs
  Pet state
  level recalculation
  growth stage transition candidate / application

src/ascii/monsters/
  SpeciesIdに対応するASCII Art asset
\`\`\`

Domain層は `SpeciesId` を扱うが、ASCII Art文字列やterminal表示へ依存しない。

CLI renderer / ASCII層が `SpeciesId` から表示用assetを解決する。

行動中の進化はDomain/Applicationで制御する。外出報酬などで条件を満たした場合は `PendingEvolution` として保存し、`pet.stage` / `pet.species_id` は進化前の姿を保持する。

Application層はpet-facing commandでpendingを解決し、`EvolutionEvent` を返す。CLI層はこのイベントを受けて進化演出を描画するが、Domain層はANSIやsleepへ依存しない。

Monster分類、Species一覧、進化ツリー、ASCII asset mappingの詳細は以下をsource of truthとする。

\`\`\`text
.codex/docs/MONSTER.md
\`\`\`

**---**

**# 26. エラー設計**

panicを通常のユーザー操作エラーに使用しない。

例えば、

\`\`\`text
Save file unavailable
Invalid action
Pet is away
Daily action limit exceeded
\`\`\`

はResultで扱う。

例:

\`\`\`rust
enum GameError {
    PetAway,
    ActionLimitReached,
    StorageError,
    InvalidSaveData,
}
\`\`\`

**---**

**# 28. Random設計**

ランダム値取得をdomain内で直接行いすぎない。

テスト可能性のため、

\`\`\`rust
trait RandomSource
\`\`\`

のような抽象化を検討する。

最低でもランダム依存処理は関数境界を明確にする。

**---**

**# 29. Clock設計**

現在時刻取得もdomain内部から直接呼ばない。

現在の構成:

\`\`\`rust
trait Clock {
    fn now(&self) -> Timestamp;
}
\`\`\`

`Timestamp` はUnix epoch秒として扱う。

絶対時刻はDomain / PersistenceでUTC Unix timestampとして扱う。

daily action reset / daily report / login streak の日付判定は、Application層が `Clock` から取得したlocal calendar dayをDomainへ渡す。

CLI表示ではlocal timeへ変換するが、save dataへlocal datetime文字列は保存しない。

本番:

\`\`\`text
SystemClock
\`\`\`

テスト:

\`\`\`text
FixedClock
\`\`\`

これにより時間経過テストを安定して実装できる。

Application層が `Clock` を使って現在時刻を取得し、Domain層には `Timestamp` を渡す。

Domain層はシステム時計を直接参照しない。

例:

\`\`\`rust
fn apply_elapsed_time(state: &mut GameState, now: Timestamp)
\`\`\`

**---**

**# 32. CLI表示とゲームロジックを分離する**

Domainから、

\`\`\`text
"Mochi looks hungry!"
\`\`\`

のような完成済みUI文言を大量に返さない。

Domainは状態を返す。

例:

\`\`\`rust
PetCondition::Hungry
\`\`\`

CLI rendererが、

\`\`\`text
Mochi looks hungry!
\`\`\`

へ変換する。

将来的にWeb版を作りやすくするため。

**---**

**# 33. Application Layer**

ユースケース単位で処理する。

例:

\`\`\`rust
GameService::status()
GameService::feed()
GameService::play()
GameService::start\_expedition()
GameService::report()
\`\`\`

CLIは原則GameServiceを呼ぶだけとする。

Phase 4では、`feed` / `play` の状態変更と日次回数制限はDomain層で扱う。

Application層は、

\`\`\`text
load
↓
apply elapsed time
↓
reset daily action count when needed
↓
execute action
↓
save
\`\`\`

を組み立てる。

Phase 5では、level計算、growth stage更新、evolution判定もDomain層で扱う。

進化イベントの責務:

\`\`\`text
Domain:
  evolution candidate
  pending evolution
  evolution application

Application:
  load/update/save
  expedition completion
  pending resolution timing
  EvolutionEvent return

CLI:
  door status while away
  ANSI-based evolution effect
  final status rendering
\`\`\`

CLIはDomainの状態を表示に変換し、ASCII Artの選択はCLI表示側で行う。

**---**

**# 34. Wasm対応方針**

将来的に、

\`\`\`text
bitpet-core
\`\`\`

としてDomainをcrate分離できる構造が理想。

例:

\`\`\`text
workspace
├── bitpet-core
├── bitpet-cli
└── bitpet-wasm
\`\`\`

最初からworkspace化する必要はない。

ただしDomainが、

\`\`\`text
filesystem
terminal
OS command
\`\`\`

へ依存しないようにする。

Phase 10では、crate分割は行わず `src/wasm.rs` にWasm adapterを追加する。

Wasm adapterはDomain/Applicationを再利用し、ブラウザ側へsave JSONを返す。

ブラウザでの永続化先は `localStorage` や IndexedDB など呼び出し側の責務とする。

**---**
