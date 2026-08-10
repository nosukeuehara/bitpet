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
│       └── pets.rs
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

理想:

\`\`\`rust
trait Clock {
    fn now(&self) -> Timestamp;
}
\`\`\`

本番:

\`\`\`text
SystemClock
\`\`\`

テスト:

\`\`\`text
FixedClock
\`\`\`

これにより時間経過テストを安定して実装できる。

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

**---**
