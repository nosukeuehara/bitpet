# BitPet Agent Guide

このファイルは、BitPetを開発するAI Agent向けの最上位ガイドである。

ゲームそのものの詳細仕様をすべてここへ記載しない。

このファイルでは主に以下を定義する。

- プロジェクト全体の開発方針
- アーキテクチャ上の制約
- ディレクトリ構成
- ドキュメント構成
- 実装時に参照すべき仕様書
- テスト方針
- CI/CD運用
- Git / Release運用
- AI Agentの作業ルール

ゲームルールやUI仕様の詳細は `.codex/docs/` 以下を参照すること。

---

# 1. Project Overview

BitPetはRustで実装するCLI育成ゲームである。

仕事や作業の合間にターミナルから少しだけ触れるデジタルペットを提供する。

BitPetは長時間プレイを要求しない。

基本的なゲーム体験は、

```text
BitPetを起動
↓
ペットを見る
↓
少し世話をする
↓
仕事へ戻る
↓
時間経過
↓
再びBitPetを起動
```

である。

詳細なゲームコンセプトおよびゲームルールは、

```text
.codex/docs/GAME_DESIGN.md
```

を参照すること。

---

# 2. Technology

主要技術:

```text
Language:
Rust

Primary Target:
CLI

Platforms:
macOS
Linux
Windows

Future Target:
WebAssembly
Web UI
```

Rust stableを使用する。

Rust toolchainについては、

```text
rust-toolchain.toml
```

をsource of truthとする。

---

# 3. Architecture Principles

BitPetではゲームロジックと入出力を分離する。

基本構造:

```text
CLI / Web
    ↓
Application
    ↓
Domain
    ↓
Infrastructure
```

各レイヤーの責務を混在させないこと。

---

## 3.1 Domain

Domainはゲームそのもののルールを扱う。

例:

```text
Pet
Status
Action
Evolution
Expedition
Daily Report
Time progression
```

Domainは以下へ直接依存しない。

```text
terminal
filesystem
OS command
HTTP
browser API
```

可能な限りPure Rustとして実装する。

ゲームルールの詳細:

```text
.codex/docs/GAME_DESIGN.md
```

---

## 3.2 Application

Applicationはユースケースを組み立てる。

例:

```rust
GameService::status()
GameService::feed()
GameService::play()
GameService::start_expedition()
GameService::report()
```

Applicationは、

```text
Domain
Storage
Clock
Random
```

などを組み合わせて処理を実行する。

CLI固有の表示処理はここへ含めない。

---

## 3.3 CLI

CLIはユーザー入力と表示を担当する。

例:

```bash
bitpet
bitpet status
bitpet feed
bitpet play
bitpet go
bitpet report
bitpet streak
```

CLIはゲームルールを直接実装しない。

CLI仕様および表示仕様:

```text
.codex/docs/CLI_UX.md
```

---

## 3.4 Infrastructure

Infrastructureは外部環境へのアクセスを担当する。

例:

```text
Filesystem
Clock
Random
Configuration
```

ゲームロジックから直接Filesystemへアクセスしない。

永続化仕様:

```text
.codex/docs/PERSISTENCE.md
```

---

# 4. Repository Structure

基本ディレクトリ構成:

```text
bitpet/
├── README.md
├── Cargo.toml
├── rust-toolchain.toml
│
├── .codex/
│   ├── AGENTS.md
│   └── docs/
│       ├── ARCHITECTURE.md
│       ├── CLI_UX.md
│       ├── DEVELOPMENT.md
│       ├── GAME_DESIGN.md
│       ├── MONSTER.md
│       └── PERSISTENCE.md
│
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
│   │   └── report.rs
│   │
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   ├── storage.rs
│   │   ├── filesystem.rs
│   │   ├── clock.rs
│   │   └── random.rs
│   │
│   └── ascii/
│       ├── mod.rs
│       ├── pets.rs
│       └── monsters/
│
├── tests/
│   ├── lifecycle.rs
│   ├── evolution.rs
│   └── persistence.rs
│
└── .github/
    └── workflows/
        ├── ci.yml
        ├── release.yml
        └── wasm.yml
```

MVP段階では小さなmoduleを統合してもよい。

ただし最低限、

```text
Domain
CLI
Persistence
```

の責務は分離すること。

---

# 5. Documentation Structure

BitPetの仕様は責務ごとに分割する。

AI Agentは作業内容に応じて必要なファイルだけ参照すること。

---

## 5.1 AGENTS.md

対象:

```text
AI Agent
Developer
```

内容:

```text
開発ルール
アーキテクチャ原則
ディレクトリ構成
ドキュメント構成
テストルール
CI/CDルール
Git運用
Agent作業ルール
```

ゲームバランスや進化条件などの詳細仕様は記載しない。

---

## 5.2 .codex/docs/GAME_DESIGN.md

ゲームそのものの仕様を管理する。

内容:

```text
ゲームコンセプト
コアループ
ペットライフサイクル
卵
ステータス
時間経過
日付処理
連続ログイン
行動回数
経験値
レベル
進化
進化ツリー
お出かけ
帰還
ランダムイベント
Daily Report
イベントログ
ゲームバランス
UX原則
MVPゲーム仕様
```

以下の変更を行う場合は必ず参照する。

```text
Pet
Feed
Play
Evolution
Expedition
Time progression
Daily Report
Streak
```

ゲーム仕様を変更した場合はこのファイルも更新すること。

---

## 5.3 .codex/docs/MONSTER.md

Monster関連仕様を管理する。

内容:

```text
Monster classification
MonsterFamily
Species
Monster catalog
Monster evolution tree
Monster naming
ASCII asset mapping
Legacy evolution migration concept
```

以下の変更時に参照する。

```text
Monster
SpeciesId
MonsterFamily
Evolution tree
ASCII asset mapping
```

Monsterの詳細なFamily / Species /進化ツリーはこのファイルをsource of truthとする。

GAME_DESIGN.mdには進化のゲームルールを置き、28体の一覧はコピーしない。

---

## 5.4 .codex/docs/CLI_UX.md

CLIとユーザー体験を管理する。

内容:

```text
CLI commands
command arguments
status表示
feed表示
play表示
go表示
report表示
streak表示
エラーメッセージ
ASCII Art
キャラクターデザイン方針
表示サイズ
ユーザー向け文言
```

以下の変更時に参照する。

```text
src/cli/
src/ascii/
```

Domainから完成済みUI文言を返さない。

Domainは状態を返し、CLI rendererが表示へ変換する。

---

## 5.5 .codex/docs/ARCHITECTURE.md

内部設計を管理する。

内容:

```text
Layer architecture
Domain model
Application layer
GameService
Clock abstraction
Random abstraction
Repository abstraction
Wasm architecture
Dependency direction
Error architecture
Future workspace structure
```

以下のような技術判断はこちらへ記録する。

```text
trait Clock
trait RandomSource
trait GameRepository
bitpet-core
bitpet-cli
bitpet-wasm
```

ゲームルール自体はこちらへ記載しない。

---

## 5.6 .codex/docs/PERSISTENCE.md

保存データに関する仕様を管理する。

内容:

```text
save directory
save.json
config.json
schema
save version
migration
atomic write
load sequence
save sequence
corrupted data handling
Wasm storage
localStorage
IndexedDB
```

save formatを変更する場合は必ずこのファイルを更新する。

保存フォーマットにはversionを持たせる。

---

## 5.7 .codex/docs/DEVELOPMENT.md

開発・ビルド・配布に関する詳細を管理する。

内容:

```text
local development
cargo commands
testing strategy
GitHub Actions
CI
CD
release workflow
Wasm workflow
build targets
GitHub Releases
checksum
Semantic Versioning
Dependabot
Secrets
release notes
package manager distribution
self update
```

CI/CDの詳細はこちらをsource of truthとする。

AGENTS.mdにはCI/CDの原則のみ記載し、workflowの詳細はこのファイルへ置く。

---

# 6. Source of Truth Rules

同じ仕様を複数ファイルへコピーしない。

仕様ごとのsource of truthは以下とする。

```text
ゲームルール
→ .codex/docs/GAME_DESIGN.md

Monster classification / MonsterFamily / Species / Monster catalog / Monster evolution tree / Monster naming / ASCII asset mapping
→ .codex/docs/MONSTER.md

CLI / 表示
→ .codex/docs/CLI_UX.md

内部アーキテクチャ
→ .codex/docs/ARCHITECTURE.md

保存形式
→ .codex/docs/PERSISTENCE.md

CI/CD / 開発運用
→ .codex/docs/DEVELOPMENT.md

Agent共通ルール
→ AGENTS.md
```

複数ドキュメントに関係する場合は、

```text
詳細を書く
+
他ファイルからリンクする
```

方式を使用する。

同じ仕様をコピーして同期させない。

---

# 7. Documentation Lookup Rules

AI Agentは作業開始前にAGENTS.mdを読む。

その後、作業内容に応じて必要なドキュメントを読む。

例:

Monster / Evolution:

```text
AGENTS.md
↓
.codex/docs/MONSTER.md
↓
.codex/docs/GAME_DESIGN.md
↓
.codex/docs/ARCHITECTURE.md
```

進化のゲームルールのみを扱う場合:

```text
AGENTS.md
↓
.codex/docs/GAME_DESIGN.md
↓
.codex/docs/ARCHITECTURE.md
```

CLI表示:

```text
AGENTS.md
↓
.codex/docs/CLI_UX.md
```

セーブ処理:

```text
AGENTS.md
↓
.codex/docs/PERSISTENCE.md
↓
.codex/docs/ARCHITECTURE.md
```

GitHub Actions:

```text
AGENTS.md
↓
.codex/docs/DEVELOPMENT.md
```

Wasm対応:

```text
AGENTS.md
↓
.codex/docs/ARCHITECTURE.md
↓
.codex/docs/PERSISTENCE.md
↓
.codex/docs/DEVELOPMENT.md
```

関係のないドキュメントを毎回すべて読み込む必要はない。

---

# 8. Implementation Rules

AI Agentは以下を守る。

## 8.1 Small Changes

一度に巨大な変更を行わない。

基本:

```text
small implementation
↓
test
↓
next implementation
```

vertical sliceを優先する。

---

## 8.2 Avoid Overengineering

将来必要になる可能性だけを理由に巨大な抽象化を作らない。

現在必要な責務のみ実装する。

ただし、

```text
Time
Random
Storage
```

についてはテスト可能性およびWasm対応のため境界を明確にする。

---

## 8.3 Dependency Policy

不要なcrateを追加しない。

BitPetは小さなCLIバイナリを目標とする。

新しいdependencyを追加する場合、

```text
標準ライブラリでは難しいか
既存dependencyで実現できないか
継続的に保守されているか
binary sizeへの影響
```

を考慮する。

---

## 8.4 Error Handling

通常のユーザー操作エラーにpanicを使用しない。

`.unwrap()` は失敗しないことが論理的に保証できる場合のみ使用する。

通常のエラーは、

```rust
Result<T, GameError>
```

等で扱う。

内部エラーをそのままユーザーへ表示しない。

---

## 8.5 Game Balance Values

ゲームバランス値をロジック中へ散在させない。

例:

```rust
const HUNGER_DECAY_PER_HOUR: f32 = 3.0;
```

または設定構造へ集約する。

ゲームバランス変更時は、

```text
.codex/docs/GAME_DESIGN.md
```

も確認する。

---

# 9. Testing Rules

テストでは実装詳細よりゲームルールを優先する。

最低限テスト対象:

```text
時間経過
日付変更
連続ログイン
行動回数制限
経験値
進化
お出かけ帰還
save/load
migration
```

時間依存テストでSystem Clockを直接利用しない。

固定時刻を注入できる設計にする。

例:

```text
SystemClock
FixedClock
```

ランダム処理も可能な限り決定的にテストできるようにする。

変更後は最低限以下を実行する。

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
```

---

# 10. CI

GitHub Actionsを使用する。

CIは、

```text
Pull Request
main push
```

で実行する。

最低限:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
```

を実行する。

CIが失敗している状態をmainへマージしない。

詳細:

```text
.codex/docs/DEVELOPMENT.md
```

---

# 11. CD

Native CLIとWasmは別workflowとして管理する。

```text
.github/workflows/ci.yml
.github/workflows/release.yml
.github/workflows/wasm.yml
```

役割:

```text
ci.yml
→ Pull Request / main quality checks

release.yml
→ Native binary release

wasm.yml
→ Wasm build / Web deploy
```

1つの巨大workflowへ統合しない。

---

# 12. Release Policy

Native CLIはGit tagによってリリースする。

例:

```text
v0.1.0
v0.2.0
v1.0.0
```

Semantic Versioningを基本とする。

```text
Cargo.toml
version = 0.2.0

Git tag
v0.2.0
```

のようにversionを一致させる。

通常のmain pushではNative Releaseを作成しない。

---

# 13. Native Release Targets

最低対応:

```text
aarch64-apple-darwin
x86_64-apple-darwin
x86_64-unknown-linux-gnu
x86_64-pc-windows-msvc
```

成果物名には、

```text
project
version
target
```

を含める。

例:

```text
bitpet-v0.2.0-aarch64-apple-darwin.tar.gz
bitpet-v0.2.0-x86_64-unknown-linux-gnu.tar.gz
bitpet-v0.2.0-x86_64-pc-windows-msvc.zip
```

GitHub Releasesを最初の正式配布元とする。

---

# 14. Wasm

Wasm版はNative CLIとは別成果物とする。

将来的な構成:

```text
workspace/
├── bitpet-core
├── bitpet-cli
└── bitpet-wasm
```

DomainはWasmから再利用可能にする。

Wasm側からFilesystemやTerminalへの依存を要求しない。

Wasm成果物は静的ホスティング可能な構成とする。

詳細:

```text
.codex/docs/ARCHITECTURE.md
.codex/docs/PERSISTENCE.md
.codex/docs/DEVELOPMENT.md
```

---

# 15. GitHub Actions Security

workflowは最小権限とする。

通常CI:

```yaml
permissions:
  contents: read
```

Release:

```yaml
permissions:
  contents: write
```

必要のない権限を追加しない。

秘密情報は、

```text
GitHub Actions Secrets
```

等を利用する。

tokenやcredentialをコードへ直接保存しない。

---

# 16. AI Agent Workflow

AI Agentは機能実装時、以下の手順を基本とする。

```text
1. AGENTS.mdを読む

2. 関係するdocsを読む

3. 現在の実装を確認する

4. 小さい変更単位を決定する

5. 実装する

6. テストを追加・修正する

7. cargo fmt

8. cargo clippy

9. cargo test

10. 必要ならdocsを更新する
```

仕様と実装が食い違う場合、勝手に仕様を変更しない。

意図的に仕様変更を行う場合は対応するdocsも更新する。

---

# 17. Documentation Update Rules

コード変更によって仕様が変わる場合、同じ変更でdocsも更新する。

例:

進化条件変更:

```text
code
+
.codex/docs/GAME_DESIGN.md
```

save schema変更:

```text
code
+
.codex/docs/PERSISTENCE.md
```

CLI command追加:

```text
code
+
.codex/docs/CLI_UX.md
```

CI変更:

```text
workflow
+
.codex/docs/DEVELOPMENT.md
```

アーキテクチャ変更:

```text
code
+
.codex/docs/ARCHITECTURE.md
```

AGENTS.mdは頻繁に変更する必要はない。

プロジェクト全体の開発ルールまたはドキュメント構成が変わった場合のみ更新する。

---

# 18. MVP Development Order

基本的な実装順:

```text
Phase 1
Project foundation

Phase 2
New game / load / status / save

Phase 3
Time progression

Phase 4
Feed / Play

Phase 5
Level / Evolution

Phase 6
Daily Report / Streak

Phase 7
Expedition

Phase 8
Error handling / Migration / Quality

Phase 9
Native Release

Phase 10
Wasm
```

ただし現在実装済みのPhaseを再実装しない。

既存コードを確認してから作業を開始する。

---

# 19. Non Goals

現段階では以下を前提に設計しない。

```text
Account
Login
Cloud Save
Server
PvP
Realtime Communication
Payment
Advertisement
AI Chat
```

不要なサーバー依存を導入しない。

---

# 20. Core Development Principle

BitPetの価値はシステムの複雑さではない。

最も重要なのは、

> ターミナルを開いたとき、小さな生き物がそこにいる感覚

である。

技術的に高度であることより、

```text
simple
small
predictable
testable
maintainable
```

な実装を優先する。
