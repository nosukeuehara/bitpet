# BitPet

BitPet は Rust で実装する小さな CLI 育成ゲームです。

現在の実装は、基本的なゲーム開始・復元ループ、起動していない間の時間経過反映、`feed` / `play` による世話、レベルアップ、Monster Domain による Stage 1 / Stage 2 / Final 進化、日次レポート、連続ログイン日数、お出かけまで対応しています。`bitpet` コマンドとしてビルド・起動でき、初回起動時に新しいペットを作成して `save.json` に保存し、2 回目以降は保存済みのペット状態を読み込んで表示します。Native CLI のリリース workflow と Wasm build adapter も用意しています。

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
- 7 Family / 28 Species の Monster catalog
- Baby から Stage 1 / Stage 2 / Final への進化
- Species ID に応じた ASCII Art 切り替え
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

- Explore 以外のお出かけ種類
- Web UI

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

Family   : -
Stage    : Baby
Mood     : 72%
Hunger   : 72%
Energy   : 72%
```

`feed` は hunger と mood を回復します。`play` は mood と experience を増やし、energy を消費します。どちらも 1 日 3 回まで実行できます。

experience が一定値に達するとレベルが上がり、Baby から Stage 1、Stage 2、Final へ進化します。Stage 1 で Family / Species が決まり、以降は基本的にその Family 内で成長します。進化先は `feed` / `play` の累計傾向から決定されます。

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

GitHub Release が公開されている場合は、自分の OS / CPU に合う archive をダウンロードして `bitpet` を実行します。

想定される配布ファイル:

- `bitpet-vX.Y.Z-aarch64-apple-darwin.tar.gz`
- `bitpet-vX.Y.Z-x86_64-apple-darwin.tar.gz`
- `bitpet-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz`
- `bitpet-vX.Y.Z-x86_64-pc-windows-msvc.zip`

v0.1.0 の場合は `vX.Y.Z` を `v0.1.0` に読み替えてください。

対応 OS / target:

- macOS Apple Silicon: `aarch64-apple-darwin`
- macOS Intel: `x86_64-apple-darwin`
- Linux x86_64: `x86_64-unknown-linux-gnu`
- Windows x86_64: `x86_64-pc-windows-msvc`

macOS / Linux:

```bash
tar -xzf bitpet-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz
./bitpet-vX.Y.Z-x86_64-unknown-linux-gnu/bitpet
```

Windows:

```powershell
Expand-Archive bitpet-vX.Y.Z-x86_64-pc-windows-msvc.zip
.\bitpet-vX.Y.Z-x86_64-pc-windows-msvc\bitpet.exe
```

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

macOS / Linux では `~/.bitpet/save.json`、Windows では `%APPDATA%\BitPet\save.json` を使用します。保存形式には将来のマイグレーション用に `version` を持たせ、時間経過計算用に `last_updated_at`、行動回数制限用に `daily_actions`、進化判定用に `care_stats`、日次記録用に `daily_report` と `login`、お出かけ状態用に `expedition`、Monster の安定IDとして `pet.species_id` を保存しています。

古い save version は起動時に現在の形式へ移行します。壊れた `save.json` は panic せず、読み込みエラーとして表示します。

## CI/CD とリリース方針

品質確認の基本コマンドは以下です。

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
```

GitHub Actions は Pull Request と `main` push で CI を実行します。Native CLI のリリースは `vX.Y.Z` 形式の Git tag を push したときに、macOS Apple Silicon、macOS Intel、Linux x86_64、Windows x86_64 向け archive と checksum を作成し、GitHub Release へアップロードする構成です。

## Wasm 対応について

BitPet は `wasm` feature で WebAssembly 向け adapter をビルドできます。Web UI はまだ未実装ですが、Web アプリケーション側からゲームロジックを呼び出し、save JSON を `localStorage` や IndexedDB へ保存するための入口を用意しています。

Wasm target の確認:

```bash
rustup target add wasm32-unknown-unknown
cargo check --target wasm32-unknown-unknown --no-default-features --features wasm
cargo build --release --target wasm32-unknown-unknown --no-default-features --features wasm
```

Web 向け binding を生成する場合:

```bash
cargo install wasm-bindgen-cli --version 0.2.127 --locked
wasm-bindgen target/wasm32-unknown-unknown/release/bitpet.wasm \
  --out-dir dist/wasm \
  --target web
```

Web 側の利用イメージ:

```js
import init, { BitPetWasm } from "./bitpet.js";

await init();

const now = Math.floor(Date.now() / 1000);
const saved = localStorage.getItem("bitpet.save");
const bitpet = saved
  ? BitPetWasm.from_save_json(saved, now)
  : BitPetWasm.new_game(now);

localStorage.setItem("bitpet.save", bitpet.status(now));
```

Native 版は filesystem 上の `save.json` を直接読み書きします。Wasm 版は filesystem を使わず、呼び出し側が `save_json()` や各コマンドの戻り値をブラウザ storage へ保存します。

## ライセンス

BitPet は MIT License で配布します。詳細は [LICENSE](LICENSE) を参照してください。

配布 archive には Rust 依存 crate のライセンス概要として [THIRD_PARTY_LICENSES.txt](THIRD_PARTY_LICENSES.txt) も同梱します。

## 詳細設計書

詳細な設計は `.codex/docs/` 配下にあります。

- [Agent guide](.codex/AGENTS.md)
- [Game design](.codex/docs/GAME_DESIGN.md)
- [Monster catalog / evolution](.codex/docs/MONSTER.md)
- [CLI / UX](.codex/docs/CLI_UX.md)
- [Architecture](.codex/docs/ARCHITECTURE.md)
- [Persistence](.codex/docs/PERSISTENCE.md)
- [Development](.codex/docs/DEVELOPMENT.md)
