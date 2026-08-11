# BitPet

BitPet は Rust で実装する小さな CLI 育成ゲームです。

現在の実装は、プロジェクト基盤の初期段階です。`bitpet` コマンドとしてビルド・起動でき、デフォルトのペット状態を表示し、予定されている CLI コマンドをパースできます。ゲームアクション、ファイル永続化、時間経過、進化、お出かけ、レポート、リリース自動化、Wasm 出力はまだ未実装です。

## コンセプト

BitPet は、仕事や作業の合間にターミナルから短時間だけ様子を見るデジタルペットです。

想定している基本ループは、CLI を開いてペットを見る、少し世話をする、作業へ戻る、時間をおいてまた様子を見る、というものです。

長時間プレイや常駐プロセスは前提にしません。将来的には、コマンド実行時に保存済み時刻から経過時間を再計算する設計です。

## 現在の主な機能

- `bitpet` という Rust CLI バイナリ
- `bitpet` と `bitpet status` によるデフォルトペット状態の表示
- 小さな ASCII Art 表示
- 予定されているコマンド群の基本パーサ
- CLI、application、domain、infrastructure、ASCII rendering の初期レイヤ分離
- `GameState`、`Pet`、ステータス値の最小ドメインモデル
- 永続化やテスト容易性に向けた repository、clock、random 境界の土台

## 今後の予定

- 新規ゲーム作成と `save.json` への永続化
- 2 回目以降の起動で同じペット状態を復元
- アプリを起動していない間の時間経過
- 1 日の行動回数制限
- `feed` と `play`
- 経験値、レベル、成長段階、進化
- お出かけと帰還結果
- Daily Report と連続ログイン
- Native CLI のリリース workflow
- 将来的な Wasm / Web UI 対応

## CLI の利用イメージ

リポジトリ内で実行する場合:

```bash
cargo run -- status
```

ビルド後に実行する場合:

```bash
cargo build
./target/debug/bitpet
./target/debug/bitpet status
./target/debug/bitpet --version
```

現在の出力は最小のステータス表示です。

```text
  /\_/\
 ( o.o )
  > ^ <

Mochi
Lv. 1

Mood     : 72%
Hunger   : 72%
Energy   : 72%
```

以下のコマンドはパースされますが、ゲーム処理はまだ未実装です。

```bash
bitpet feed
bitpet play
bitpet go
bitpet report
bitpet streak
```

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

ファイル永続化は予定されていますが、まだ実装されていません。

設計上は DB を使わず、ローカルファイルへ保存します。macOS / Linux では `~/.bitpet/save.json`、Windows では OS 標準のアプリケーションデータ領域を使う予定です。保存形式には将来のマイグレーション用に `version` を持たせます。

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
