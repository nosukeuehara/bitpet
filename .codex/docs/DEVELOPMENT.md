# BitPet — Development

**# 30. テスト戦略**

細かな実装詳細をすべてテストする必要はない。

ゲームルールを中心にテストする。

優先度高:

\`\`\`text
時間経過
日付変更
連続ログイン
行動回数制限
経験値
進化判定
お出かけ帰還
セーブ/ロード
\`\`\`

**---**

**# 31. テスト例**

\`\`\`rust
\#[test]
fn hunger\_decreases\_after\_time\_passes() {
    // Given
    // hunger = 100
    // last update = 10:00

    // When
    // current time = 12:00

    // Then
    // hunger < 100
}
\`\`\`

時間そのものをmock可能にすること。

**---**

**# 37. 開発フェーズ**

AI Agentは以下の順番で実装すること。

**## Phase 1**

プロジェクト基盤

\`\`\`text
Cargo project
CLI parser
save directory
GameState
Pet
\`\`\`

**---**

**## Phase 2**

基本ループ

\`\`\`text
new game
load game
status
save
\`\`\`

ここで

\`\`\`bash
bitpet
\`\`\`

を実行してペットが表示される状態にする。

**---**

**## Phase 3**

時間システム

\`\`\`text
Clock
elapsed time
status decay
date rollover
\`\`\`

**---**

**## Phase 4**

アクション

\`\`\`text
feed
play
daily action count
experience
\`\`\`

**---**

**## Phase 5**

成長

\`\`\`text
level
growth stage
evolution
ASCII switching
\`\`\`

**---**

**## Phase 6**

レポート

\`\`\`text
event log
daily report
login streak
\`\`\`

**---**

**## Phase 7**

お出かけ

\`\`\`text
start expedition
return time
away state
result
reward
\`\`\`

**---**

**## Phase 8**

品質

\`\`\`text
error handling
save migration
atomic write
tests
README
\`\`\`

**---**

**## Phase 9**

完成前整合性

\`\`\`text
Monster Domain final check
legacy save fixtures
Egg lifecycle
hatching boundary
local calendar day handling
Native / Wasm checks
\`\`\`

**---**

**# 38. AI Agent実装ルール**

AI Agentは以下を守る。

**### 1.**

一度に巨大な機能を実装しない。

小さな単位で、

\`\`\`text
実装
↓
test
↓
次
\`\`\`

と進める。

**### 2.**

ゲームバランス値をハードコードして散らばらせない。

定数または設定へ集約する。

**### 3.**

CLIの都合をDomainへ持ち込まない。

**### 4.**

不要な依存ライブラリを増やさない。

BitPetは小さい実行ファイルを目指す。

**### 5.**

高度な抽象化を先回りして作らない。

将来必要になる可能性だけを理由に巨大なtrait構造を作らない。

**### 6.**

ただし、

\`\`\`text
Time
Random
Storage
\`\`\`

はテスト・Wasm対応のため境界を意識する。

**### 7.**

\`.unwrap()\` は、失敗しないことが論理的に保証できる場所以外では避ける。

**### 8.**

ゲームデータ破損につながる変更を行う場合はマイグレーションを検討する。

**---**

**# 43. AI Agentへの最終指示**

最初から全仕様を完成させようとしないこと。

まず以下の最小vertical sliceを完成させる。

\`\`\`text
bitpet起動
↓
初回ならペット生成
↓
ASCII表示
↓
状態表示
↓
保存
↓
次回起動時に読み込み
↓
経過時間反映
\`\`\`

この状態で実際にCLIとして使用できることを確認する。

その後、

\`\`\`text
feed
play
report
evolution
expedition
\`\`\`

の順番で追加する。

各Phase終了時に、

\`\`\`bash
cargo fmt --check
cargo clippy
cargo test
\`\`\`

が成功する状態を維持すること。

ゲームロジックを変更するときは、対応するゲームルールのテストを追加または修正すること。

設計よりも複雑な実装を作らないこと。

BitPetの価値はシステムの複雑さではなく、

**\*\*「ターミナルを開いたとき、小さな生き物がそこにいる感覚」\*\***

にある。

**# 44. CI/CD設計**

BitPetはGitHub Actionsを利用してCI/CDを構築する。

目的は以下。

\`\`\`text
Pull Request
    ↓
自動品質チェック
    ↓
mainへマージ
    ↓
リリースタグ作成
    ↓
各OS向けバイナリをビルド
    ↓
Wasmをビルド
    ↓
GitHub Releases / Webへ配布
\`\`\`

CLI版とWasm版は同じDomainロジックを使用するが、成果物としては分離して配布する。

**---**

**# 45. CIの目的**

CIではコードを配布しない。

以下の品質チェックを自動化する。

\`\`\`text
format
lint
test
build
\`\`\`

Pull Requestおよびmainへのpush時に実行する。

最低限以下を実行する。

\`\`\`bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
\`\`\`

CIが失敗している状態ではmainへマージしない運用を推奨する。

**---**

**# 46. CI workflow**

推奨ファイル:

\`\`\`text
.github/
└── workflows/
    ├── ci.yml
    ├── release.yml
    └── wasm.yml
\`\`\`

MVP段階では、

\`\`\`text
ci.yml
release.yml
\`\`\`

の2つから開始してもよい。

**---**

**# 47. ci.yml**

実行条件:

\`\`\`yaml
on:
  pull\_request:
  push:
    branches:
      \- main
\`\`\`

CIではLinux環境を基本とする。

Rustの通常のユニットテストやDomainテストについては、毎回Windows/macOSで実行する必要はない。

基本:

\`\`\`text
ubuntu
 ↓
fmt
 ↓
clippy
 ↓
test
 ↓
build
\`\`\`

とする。

OS依存コードが増えた場合のみmatrixテストを検討する。

**---**

**# 48. CIの推奨処理順**

\`\`\`text
checkout
↓
Rust toolchain setup
↓
dependency cache
↓
cargo fmt
↓
cargo clippy
↓
cargo test
↓
cargo build
\`\`\`

Rustのtoolchainはstableを基本とする。

プロジェクトでRustバージョンを固定する場合は、

\`\`\`text
rust-toolchain.toml
\`\`\`

をリポジトリへ追加する。

例:

\`\`\`toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
\`\`\`

**---**

**# 49. Cargoキャッシュ**

GitHub ActionsではCargoのキャッシュを使用する。

対象:

\`\`\`text
\~/.cargo
target/
\`\`\`

ただし独自の複雑なキャッシュ処理を最初から作らない。

一般的なRust向けGitHub ActionまたはGitHub Actions標準のcache機能を利用する。

CI高速化が目的であり、キャッシュ処理自体をプロジェクトの複雑性にしない。

**---**

**# 50. CDの基本方針**

BitPetはタグベースでリリースする。

例:

\`\`\`text
v0.1.0
v0.2.0
v1.0.0
\`\`\`

以下の形式のtagがpushされた場合にrelease workflowを実行する。

\`\`\`yaml
on:
  push:
    tags:
      \- "v\*"
\`\`\`

通常のmainへのpushだけではユーザー向けバイナリをリリースしない。

**---**

**# 51. バージョニング**

Semantic Versioningを基本とする。

\`\`\`text
MAJOR.MINOR.PATCH
\`\`\`

例:

\`\`\`text
0.1.0
0.2.0
0.2.1
1.0.0
\`\`\`

MVP開発中は、

\`\`\`text
0.x.x
\`\`\`

を使用する。

Cargo.tomlのversionとGit tagは一致させる。

例:

\`\`\`toml
[package]
version = "0.2.0"
\`\`\`

に対して、

\`\`\`text
v0.2.0
\`\`\`

をリリースする。

AI Agentはリリース処理を変更する際、この整合性を壊さないこと。

**---**

**# 52. CLIリリース**

タグが作成された場合、GitHub Actionsで各OS向けにビルドする。

最低対応ターゲット:

\`\`\`text
macOS Apple Silicon
macOS Intel
Linux x86\_64
Windows x86\_64
\`\`\`

推奨Rust target:

\`\`\`text
aarch64-apple-darwin
x86\_64-apple-darwin
x86\_64-unknown-linux-gnu
x86\_64-pc-windows-msvc
\`\`\`

必要に応じて将来的に、

\`\`\`text
aarch64-unknown-linux-gnu
\`\`\`

等を追加する。

**---**

**# 53. ビルドmatrix**

GitHub Actionsではmatrix strategyを利用する。

概念:

\`\`\`text
                ┌─ macOS ARM
                ├─ macOS Intel
tag v0.2.0 ─────┼─ Linux
                └─ Windows
\`\`\`

各runnerで、

\`\`\`bash
cargo build --release
\`\`\`

を実行する。

クロスコンパイルを無理に1台のLinux runnerから行う必要はない。

基本はターゲットOSのGitHub-hosted runnerを使用する。

**---**

**# 54. 配布ファイル**

CLI版の成果物は圧縮して配布する。

Unix系:

\`\`\`text
bitpet-v0.2.0-aarch64-apple-darwin.tar.gz

bitpet-v0.2.0-x86\_64-apple-darwin.tar.gz

bitpet-v0.2.0-x86\_64-unknown-linux-gnu.tar.gz
\`\`\`

Windows:

\`\`\`text
bitpet-v0.2.0-x86\_64-pc-windows-msvc.zip
\`\`\`

中身:

\`\`\`text
bitpet
\`\`\`

またはWindowsの場合:

\`\`\`text
bitpet.exe
\`\`\`

必要に応じて、

\`\`\`text
README
LICENSE
\`\`\`

を同梱してもよい。

**---**

**# 55. GitHub Releases**

CLIバイナリはGitHub Releasesへアップロードする。

GitHub Release:

\`\`\`text
BitPet v0.2.0

Assets

bitpet-v0.2.0-aarch64-apple-darwin.tar.gz
bitpet-v0.2.0-x86\_64-apple-darwin.tar.gz
bitpet-v0.2.0-x86\_64-unknown-linux-gnu.tar.gz
bitpet-v0.2.0-x86\_64-pc-windows-msvc.zip
\`\`\`

ユーザーは自分のOS向け成果物をダウンロードして使用できる。

**---**

**# 56. checksum**

リリースファイルには可能であればchecksumを付与する。

例:

\`\`\`text
SHA256SUMS
\`\`\`

内容例:

\`\`\`text
xxxx  bitpet-v0.2.0-aarch64-apple-darwin.tar.gz
yyyy  bitpet-v0.2.0-x86\_64-unknown-linux-gnu.tar.gz
\`\`\`

将来的にインストールスクリプトやpackage managerからダウンロードするときの検証にも利用できる。

**---**

**# 57. Wasm CI/CD**

Wasm版はCLIとは別の成果物として扱う。

想定構成:

\`\`\`text
bitpet-core
     │
     ├── bitpet-cli
     │
     └── bitpet-wasm
\`\`\`

Wasm側は、

\`\`\`text
bitpet-core
\`\`\`

に依存する。

**---**

**# 58. Wasm build**

Wasmターゲット:

\`\`\`text
wasm32-unknown-unknown
\`\`\`

を基本とする。

利用するWebバインディング方式に応じて、

\`\`\`text
wasm-bindgen
wasm-pack
\`\`\`

等を使用してもよい。

Wasm成果物例:

\`\`\`text
bitpet\_bg.wasm
bitpet.js
bitpet.d.ts
\`\`\`

実際の構成は採用するWasmツールチェーンに合わせる。

**---**

**# 59. Wasm配信方法**

Wasmは静的ファイルとして配信する。

ブラウザアクセス時:

\`\`\`text
Web App
   ↓
bitpet.js
   ↓
bitpet\_bg.wasm
\`\`\`

必要なタイミングでブラウザがWasmファイルをHTTP経由で取得する。

Wasmプロセスをサーバー側で常駐させる構成にはしない。

**---**

**# 60. Webへのデプロイ**

Wasm版はGitHub Actionsから静的ホスティング先へデプロイする。

候補:

\`\`\`text
GitHub Pages
Cloudflare Pages
Vercel
Netlify
S3 + CDN
\`\`\`

BitPet自体にサーバー処理が不要であれば静的ホスティングを優先する。

WasmとJavaScript、HTML、CSSだけを配信できる構成とする。

**---**

**# 61. CLIとWasmのCD分離**

CLIリリースとWebデプロイは別workflowとして扱うことを推奨する。

例:

\`\`\`text
release.yml
    ↓
Native Binary
    ↓
GitHub Releases


wasm.yml
    ↓
Wasm build
    ↓
Static Hosting
\`\`\`

理由:

\`\`\`text
成果物が異なる
実行環境が異なる
失敗原因が異なる
デプロイ頻度を独立させられる
\`\`\`

ため。

**---**

**# 62. Wasmデプロイ条件**

2つの方式が考えられる。

**## 方法A**

main更新時に自動デプロイ。

\`\`\`text
main
 ↓
wasm build
 ↓
Web deploy
\`\`\`

開発中はこちらを推奨する。

Web版を常に最新版として確認できる。

**## 方法B**

Git tag作成時だけデプロイ。

\`\`\`text
v0.2.0
 ↓
wasm build
 ↓
Web deploy
\`\`\`

CLI版とWeb版のバージョンを完全に同期したい場合はこちら。

BitPetでは最初は、

\`\`\`text
main → preview/latest Web
tag → CLI Release
\`\`\`

でもよい。

**---**

**# 63. 推奨CI/CD全体像**

\`\`\`text
Developer
    │
    │ push
    ▼
Feature Branch
    │
    │ Pull Request
    ▼
GitHub Actions CI
    │
    ├─ cargo fmt
    ├─ cargo clippy
    ├─ cargo test
    └─ cargo build
    │
    ▼
Merge
    │
    ▼
main
    │
    ├─────────────────────┐
    │                     │
    ▼                     ▼
Wasm Workflow          Development
    │
    ▼
Wasm Build
    │
    ▼
Static Hosting


Release時:

git tag v0.x.x
    │
    ▼
Release Workflow
    │
    ├─ macOS ARM
    ├─ macOS Intel
    ├─ Linux
    └─ Windows
    │
    ▼
Package
    │
    ▼
Checksum
    │
    ▼
GitHub Release
\`\`\`

**---**

**# 64. Release前検証**

release workflow内でも最低限、

\`\`\`bash
cargo test
\`\`\`

を実行する。

CIを通過済みであっても、ReleaseがCIに完全依存しない構成にする。

推奨:

\`\`\`text
test
 ↓
build release binaries
 ↓
package
 ↓
publish
\`\`\`

**---**

**# 65. GitHub Actions権限**

Workflowには必要最低限のpermissionsだけを付与する。

通常CI:

\`\`\`yaml
permissions:
  contents: read
\`\`\`

GitHub Release作成時:

\`\`\`yaml
permissions:
  contents: write
\`\`\`

不要な、

\`\`\`text
issues: write
pull-requests: write
packages: write
\`\`\`

などは付与しない。

**---**

**# 66. Secrets**

秘密情報をリポジトリへ保存しない。

外部ホスティングへのデプロイでtokenが必要な場合は、

\`\`\`text
GitHub Actions Secrets
\`\`\`

またはホスティングサービスが提供するGitHub連携を使用する。

Rustコード、workflow、設定ファイルへtokenをハードコードしない。

**---**

**# 67. Dependabot**

可能であればGitHub Dependabotを有効にする。

対象:

\`\`\`text
Cargo dependencies
GitHub Actions
\`\`\`

ただし更新PRを自動マージする必要はない。

CIが成功していることと、変更内容を確認した上で更新する。

**---**

**# 68. Release Notes**

GitHub Releasesには変更内容を記載する。

最低限、

\`\`\`text
Features
Fixes
Breaking Changes
\`\`\`

が分かるようにする。

GitHubの自動生成Release Notesを使用してもよい。

MVP段階では手動で詳細なCHANGELOGを管理しなくてもよい。

**---**

**# 69. CHANGELOG**

プロジェクトが成長した場合、

\`\`\`text
CHANGELOG.md
\`\`\`

を導入する。

ただしv0.1段階では必須ではない。

GitHub Release自体を変更履歴として利用してもよい。

**---**

**# 70. 配布手段の拡張**

GitHub Releasesを最初の正式な配布元とする。

将来的には以下を追加できる。

macOS:

\`\`\`text
Homebrew
\`\`\`

Windows:

\`\`\`text
winget
scoop
\`\`\`

Linux:

\`\`\`text
Homebrew
AUR
deb/rpm
\`\`\`

Rust:

\`\`\`text
cargo install bitpet
\`\`\`

ただし最初から全package managerへ対応しない。

優先順位:

\`\`\`text
GitHub Releases
↓
Homebrew
↓
その他package manager
\`\`\`

程度でよい。

**---**

**# 71. Self Update**

将来的に、

\`\`\`bash
bitpet update
\`\`\`

のような自己更新機能を提供してもよい。

その場合、

\`\`\`text
GitHub Releases API
↓
latest version確認
↓
現在version比較
↓
OS/CPUに対応するartifact取得
↓
checksum検証
↓
binary更新
\`\`\`

という構成を検討する。

ただしMVPには含めない。

package manager経由で導入されたBitPetについては、原則としてpackage manager側のupdate機構を利用する。

自己更新とpackage manager更新を混在させる場合は、ユーザー環境を壊さないよう十分に設計する。

**---**

**# 72. AI Agent向けCI/CD実装指示**

AI AgentはCI/CD実装時に以下を守る。

1\. まずCIを完成させ、その後Release automationを追加する。

2\. CIでは最低限以下を保証する。

\`\`\`bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
\`\`\`

3\. GitHub ActionsのActionは可能な限り広く利用されているものを使用する。

4\. Actionのversionを明示する。

5\. Workflowへ不要なpermissionsを与えない。

6\. Secretsをソースコードやworkflowへ直接記載しない。

7\. Native releaseとWasm deploymentを1つの巨大workflowへまとめない。

8\. Release成果物のファイル名には以下を含める。

\`\`\`text
project name
version
target
\`\`\`

9\. Release前にテストを実行する。

10\. CI/CD実装後はREADMEへ、

\`\`\`text
Install
Release
Development
\`\`\`

の最低限の説明を追加する。

**---**

**# 73. 初期CI/CD完成条件**

以下が動作すれば最初のCI/CDは完成とする。

Pull Request:

\`\`\`text
PR作成
↓
GitHub Actions
↓
fmt
clippy
test
build
↓
成功
\`\`\`

Release:

\`\`\`text
git tag v0.1.0
git push origin v0.1.0
↓
GitHub Actions
↓
macOS / Linux / Windows build
↓
archive作成
↓
GitHub Release作成
↓
binary添付
\`\`\`

Wasm:

\`\`\`text
main更新
↓
GitHub Actions
↓
Wasm build
↓
静的ホスティングへdeploy
\`\`\`

この3本が独立して動作する状態を目標とする。
