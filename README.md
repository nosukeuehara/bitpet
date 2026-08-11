# BitPet

BitPet は Rust で実装する小さな CLI 育成ゲームです。

現在の実装は、基本的なゲーム開始・復元ループ、起動していない間の時間経過反映、`feed` / `play` による世話、レベルアップと最初の進化、日次レポート、連続ログイン日数、お出かけまで対応しています。`bitpet` コマンドとしてビルド・起動でき、初回起動時に新しいペットを作成して `save.json` に保存し、2 回目以降は保存済みのペット状態を読み込んで表示します。リリース自動化、Wasm 出力はまだ未実装です。

## コンセプト

BitPet は、仕事や作業の合間にターミナルから短時間だけ様子を見るデジタルペットです。

想定している基本ループは、CLI を開いてペットを見る、少し世話をする、作業へ戻る、時間をおいてまた様子を見る、というものです。

長時間プレイや常駐プロセスは前提にしません。BitPet はコマンド実行時に保存済み時刻から経過時間を計算し、ペットの状態へ反映します。

## 現在の主な機能

- `bitpet` という Rust CLI バイナリ
- `bitpet` と `bitpet status` によるペット状態の表示
- 初回起動時の新規ゲーム作成
- `save.json` への保存
- 2 回目以降の起動での保存済みゲーム復元
- 前回起動時刻からの時間経過反映
- 常駐プロセスを使わない状態更新
- `bitpet feed` で食事を与える
- `bitpet play` で遊ぶ
- 1 日あたり `feed` / `play` 各 3 回までの行動回数制限
- `play` による経験値獲得
- experience によるレベルアップ
- Baby から Stage 1 への最初の進化
- 進化先に応じた ASCII Art 切り替え
- `bitpet report` による当日の行動レポート表示
- `bitpet streak` による連続ログイン日数表示
- 起動日をログイン日として記録
- `bitpet go` による Explore お出かけ
- お出かけ中の状態保存と帰還時の結果反映
- 小さな ASCII Art 表示
- CLI、application、domain、infrastructure、ASCII rendering の初期レイヤ分離
- `GameState`、`Pet`、ステータス値の最小ドメインモデル
- ファイル永続化と、テスト容易性に向けた repository、clock、random 境界の土台

## 今後の予定

- Stage 2 以降の成長
- 複雑な進化ツリー
- Explore 以外のお出かけ種類
- Native CLI のリリース workflow
- 将来的な Wasm / Web UI 対応

## CLI の利用イメージ

リポジトリ内で実行する場合:

```bash
cargo run
```

初回起動では保存データがない場合に新しいペットを作成し、状態を表示して保存します。2 回目以降は保存済みの `save.json` を読み込み、前回保存時刻からの経過時間を反映して表示・保存します。

ビルド後に実行する場合:

```bash
cargo build
./target/debug/bitpet
./target/debug/bitpet status
./target/debug/bitpet feed
./target/debug/bitpet play
./target/debug/bitpet go
./target/debug/bitpet report
./target/debug/bitpet streak
./target/debug/bitpet --version
```

現在の出力は最小のステータス表示です。

```text
  /\_/\
 ( o.o )
  > ^ <

Mochi
Baby
Lv. 1

Stage    : Baby
Mood     : 72%
Hunger   : 72%
Energy   : 72%
```

`feed` は hunger と mood を回復します。`play` は mood と experience を増やし、energy を消費します。どちらも 1 日 3 回まで実行できます。

experience が一定値に達するとレベルが上がり、Baby から Stage 1 へ進化します。最初の進化先は、これまでの `feed` / `play` の傾向によって `Fluffy` / `Sharp` / `Weird` のいずれかになります。

`go` は Stage 1 以降のペットを Explore に出かけさせます。BitPet は常駐せず、帰還予定時刻を `save.json` に保存します。帰還時刻を過ぎたあとに次のコマンドを実行すると、結果が反映されます。

```text
Mochi went exploring.

Expected return:
14:32
```

外出中は `feed` / `play` / 再度の `go` は実行できません。

`report` は当日の `feed` / `play` 回数、獲得 experience、mood 変化、イベントログを表示します。

```text
BitPet Daily Report

Feed        1
Play        1
Adventure   0

EXP gained  5
Mood        +15

Events
09:00 Checked in
09:01 Fed Mochi
09:02 Played with Mochi
```

`streak` は BitPet を連続して起動した日数を表示します。BitPet は常駐せず、コマンド起動時にその日をログイン日として記録します。

```text
Login streak

3 day(s)
```

現時点では未実装コマンドはありません。ただし、Explore 以外のお出かけ種類や Web UI などは今後の予定です。

## インストール方法

現時点では公開済みのリリースパッケージはありません。

ローカルでビルドする場合:

```bash
cargo build
```

現在のローカルバイナリを Cargo でインストールする場合:

```bash
cargo install --path .
```

## 開発環境のセットアップ

必要なもの:

- Rust stable
- Cargo

Rust の toolchain は `rust-toolchain.toml` を参照してください。

リポジトリを取得したら、まずビルドします。

```bash
cargo build
```

CLI を実行します。

```bash
cargo run
cargo run -- status
```

## 基本的な開発コマンド

変更前後の確認には以下を使用します。

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
```

## プロジェクト構成

```text
bitpet/
├── Cargo.toml
├── rust-toolchain.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli/
│   ├── application/
│   ├── domain/
│   ├── infrastructure/
│   └── ascii/
├── tests/
├── .codex/
│   ├── AGENTS.md
│   └── docs/
└── .github/
```

各レイヤの責務:

- `src/domain/`: ゲーム状態とゲームルール。terminal や filesystem へ直接依存しない
- `src/application/`: status 取得などのユースケース
- `src/cli/`: コマンドパースとターミナル表示
- `src/infrastructure/`: filesystem、storage、clock、random など外部環境との境界
- `src/ascii/`: CLI で使う小さなキャラクター表示

## 永続化

BitPet は DB を使わず、ローカルファイルへ保存します。

macOS / Linux では `~/.bitpet/save.json`、Windows では `%APPDATA%\BitPet\save.json` を使用します。保存形式には将来のマイグレーション用に `version` を持たせ、時間経過計算用に `last_updated_at`、行動回数制限用に `daily_actions`、進化判定用に `care_stats`、日次記録用に `daily_report` と `login`、お出かけ状態用に `expedition` を保存しています。

古い save version は起動時に現在の形式へ移行します。壊れた `save.json` は panic せず、読み込みエラーとして表示します。

## CI/CD とリリース方針

品質確認の基本コマンドは以下です。

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
```

GitHub Actions は Pull Request と `main` push で実行する予定です。Native CLI のリリースは Git tag による Semantic Versioning ベースの運用を予定しています。現在の workflow ファイルはプレースホルダーです。

## Wasm 対応について

Wasm と Web UI は将来ターゲットです。現在の crate はまだ Wasm package ではありません。

ただし、将来的に domain logic を再利用できるよう、terminal や filesystem に依存する処理を domain へ混ぜない構成を目指しています。

## 詳細設計書

詳細な設計は `.codex/docs/` 配下にあります。

- [Agent guide](.codex/AGENTS.md)
- [Game design](.codex/docs/GAME_DESIGN.md)
- [CLI / UX](.codex/docs/CLI_UX.md)
- [Architecture](.codex/docs/ARCHITECTURE.md)
- [Persistence](.codex/docs/PERSISTENCE.md)
- [Development](.codex/docs/DEVELOPMENT.md)
