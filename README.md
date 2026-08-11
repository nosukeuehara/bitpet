# BitPet

BitPet は Rust で実装する小さな CLI 育成ゲームです。

v1.0.0 は、Egg から始まるゲーム開始・復元ループ、起動していない間の時間経過反映、孵化、`feed` / `play` による世話、レベルアップ、Monster Domain による Stage 1 / Stage 2 / Final 進化、日次レポート、連続ログイン日数、お出かけまで対応しています。`bitpet` コマンドとしてビルド・起動でき、初回起動時に新しい Egg を作成して `save.json` に保存し、2 回目以降は保存済みの状態を読み込んで表示します。Native CLI のリリース workflow と Wasm build adapter も用意しています。

## コンセプト

BitPet は、仕事や作業の合間にターミナルから短時間だけ様子を見るデジタルペットです。

想定している基本ループは、CLI を開いてペットを見る、少し世話をする、作業へ戻る、時間をおいてまた様子を見る、というものです。

長時間プレイや常駐プロセスは前提にしません。BitPet はコマンド実行時に保存済み時刻から経過時間を計算し、ペットの状態へ反映します。

## 現在の主な機能

- `bitpet` という Rust CLI バイナリ
- `bitpet` と `bitpet status` によるペット状態の表示
- 初回起動時の Egg 作成
- Egg から Baby への決定的な孵化
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
- feed/play/帰還時の進化イベント
- Species ID に応じた ASCII Art 切り替え
- `bitpet report` による当日の行動レポート表示
- `bitpet streak` による連続ログイン日数表示
- 起動日をログイン日として記録
- `bitpet go` による Explore お出かけ
- お出かけ中の状態保存と帰還時の結果反映
- お出かけ中の扉表示と、帰還後の進化演出
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

初回起動では保存データがない場合に新しい Egg を作成し、状態を表示して保存します。2 回目以降は保存済みの `save.json` を読み込み、前回保存時刻からの経過時間を反映して表示・保存します。

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
./target/debug/bitpet --help
./target/debug/bitpet --version
```

利用可能なコマンドを忘れた場合は `bitpet --help` または `bitpet -h` で確認できます。各コマンドの簡単な説明は `bitpet status --help` のように表示できます。Version は `bitpet --version` または `bitpet -V` で確認できます。

新規ゲーム直後は Egg と孵化までの残り時間を表示します。

```text
   __
 /    \
 \____/

Egg

Hatching in 1h 0m
```

孵化後の出力は最小のステータス表示です。

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

Egg は作成から 1 時間後に Baby へ孵化します。孵化判定は保存済みの `egg_created_at` / `hatches_at` に基づく決定的な処理です。Egg 中は `feed` / `play` / `go` を実行できません。

`feed` は hunger と mood を回復します。`play` は mood と experience を増やし、energy を消費します。どちらも local calendar day ごとに 3 回まで実行できます。

experience が一定値に達するとレベルが上がり、Baby から Stage 1、Stage 2、Final へ進化します。Stage 1 で Family / Species が決まり、以降は基本的にその Family 内で成長します。進化先は `feed` / `play` の累計傾向から決定されます。

進化時は短いCLI演出を表示してから新しい姿を表示します。外出中に進化条件を満たした場合は、その場では進化後の姿を見せず、帰還後にペットと向き合うタイミングで進化します。

`go` は Stage 1 以降のペットを Explore に出かけさせます。BitPet は常駐せず、帰還予定時刻を `save.json` に保存します。帰還時刻を過ぎたあとに次のコマンドを実行すると、結果が反映されます。

```text
Mochi went exploring.

Expected return:
14:32
```

外出中は `feed` / `play` / 再度の `go` は実行できません。

外出中の `status` はMonsterの姿ではなく扉を表示します。

```text
+------+
|  __  |
| |  | |
| |__| |
|  __  |
| |  | |
+------+

Out now...
Back at 18:40

Returns in:
42m
```

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

v1.0.0 の配布ファイル:

| Platform | Download |
|---|---|
| macOS Apple Silicon (M1 / M2 / M3 / M4 and newer) | `bitpet-v1.0.0-aarch64-apple-darwin.tar.gz` |
| macOS Intel | `bitpet-v1.0.0-x86_64-apple-darwin.tar.gz` |
| Windows 64-bit | `bitpet-v1.0.0-x86_64-pc-windows-msvc.zip` |
| Linux 64-bit Intel / AMD | `bitpet-v1.0.0-x86_64-unknown-linux-gnu.tar.gz` |

対応 OS / target:

- macOS Apple Silicon: `aarch64-apple-darwin`
- macOS Intel: `x86_64-apple-darwin`
- Linux x86_64: `x86_64-unknown-linux-gnu`
- Windows x86_64: `x86_64-pc-windows-msvc`

Mac の種類が分からない場合は Apple menu -> About This Mac を確認してください。Apple M1 / M2 / M3 / M4 と表示される場合は Apple Silicon 版、Intel と表示される場合は Intel Mac 版を使用します。Apple Silicon 版と Intel Mac 版は別のAssetです。

`.sha256` で終わるファイルはダウンロード検証用のchecksumです。BitPetをインストールして遊ぶだけなら必須ではありません。GitHubが自動生成する `Source code (zip)` / `Source code (tar.gz)` は開発者向けで、通常のユーザー向けバイナリではありません。

macOS / Linux:

```bash
tar -xzf <downloaded-file>.tar.gz
cd <extracted-directory>
./bitpet
```

Windows:

```powershell
Expand-Archive <downloaded-file>.zip
cd <extracted-directory>
.\bitpet.exe
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

macOS / Linux では `~/.bitpet/save.json`、Windows では `%APPDATA%\BitPet\save.json` を使用します。保存形式には将来のマイグレーション用に `version` を持たせ、時間経過計算用に `last_updated_at`、行動回数制限用に `daily_actions`、進化判定用に `care_stats`、日次記録用に `daily_report` と `login`、お出かけ状態用に `expedition`、Egg の孵化状態用に `hatching`、Monster の安定IDとして `pet.species_id` を保存しています。

v0.1.x を含む古い save version は起動時に現在の形式へ移行します。既存のペットを新規Eggへ戻したり、保存データを置き換えたりしません。壊れた `save.json` は panic せず、読み込みエラーとして表示します。

時間経過、孵化、お出かけ帰還は Unix timestamp による絶対時刻で扱います。日次レポート、連続ログイン、行動回数リセットはユーザーの local calendar day で判定します。

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
const localOffsetSeconds = -new Date().getTimezoneOffset() * 60;
const saved = localStorage.getItem("bitpet.save");
const bitpet = saved
  ? BitPetWasm.from_save_json_with_local_day_offset(saved, now, localOffsetSeconds)
  : BitPetWasm.new_game_with_local_day_offset(now, localOffsetSeconds);

localStorage.setItem(
  "bitpet.save",
  bitpet.status_with_local_day_offset(now, localOffsetSeconds),
);
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
