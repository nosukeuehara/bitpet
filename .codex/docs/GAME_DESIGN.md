# BitPet — Game Design

**## 1. プロジェクト概要**

**### 1.1 プロジェクト名**

**\*\*BitPet\*\***

仮称。
「画面の隅にいる小さなペット」「仕事中にたまに様子を見るデジタルペット」というコンセプトを表す。

**### 1.2 コンセプト**

BitPetは、仕事や作業の合間にCLIから少しだけ触れる育成ゲームである。

長時間プレイするゲームではなく、

\- 朝、仕事を始めるときに状態を見る
\- 昼休みにご飯をあげる
\- 仕事中に外出させる
\- 数時間後に帰ってきた結果を見る
\- 退勤前に1日のレポートを見る

といった、現実時間と連動したゆるいゲーム体験を提供する。

ゲームを起動していない間も時間は進行する。

基本的に常駐プロセスは使用しない。

**---**

**# 5. ペットライフサイクル**

ペットは以下の順番で成長する。

\`\`\`text
Egg
 ↓
Baby
 ↓
Stage 1
 ↓
Stage 2
 ↓
Final
\`\`\`

**---**

**# 6. 卵**

ゲーム初回起動時に生成する。

内部的にはランダムな遺伝情報を保持してよい。

例:

\`\`\`rust
struct Genetics {
    nature: Nature,
    tendency: Tendency,
    seed: u64,
}
\`\`\`

ただし孵化直後の見た目は全ユーザー共通。

つまり、

\`\`\`text
Egg
 ↓
Baby（共通）
 ↓
最初の進化で分岐
\`\`\`

とする。

ユーザーが「最初から当たり・外れ」を意識しすぎない設計にする。

**---**

**# 7. ステータス**

最低限以下を持つ。

\`\`\`rust
struct Pet {
    id: String,
    name: String,

    stage: GrowthStage,

    level: u32,
    experience: u32,

    hunger: u8,
    mood: u8,
    energy: u8,

    created\_at: Timestamp,
    last\_updated\_at: Timestamp,

    genetics: Genetics,

    evolution: EvolutionState,

    expedition: Option\<Expedition>,
}
\`\`\`

各ステータスは原則:

\`\`\`text
0..=100
\`\`\`

**---**

**# 8. 時間経過**

BitPetの重要な設計原則。

バックグラウンドプロセスを常駐させない。

代わりに、

\`\`\`text
最後に保存した時刻
↓
現在時刻
↓
経過時間
↓
状態を再計算
\`\`\`

する。

例えば:

\`\`\`rust
fn apply\_elapsed\_time(
    pet: &mut Pet,
    last\_updated: Timestamp,
    now: Timestamp,
)
\`\`\`

を実装する。

**---**

**## 8.1 基本的な時間変化**

例:

1時間につき

\`\`\`text
hunger -3
energy +2
\`\`\`

など。

実際の値は定数として管理する。

\`\`\`rust
const HUNGER\_DECAY\_PER\_HOUR: f32 = 3.0;
\`\`\`

ゲームバランス値をロジック中に直接埋め込まない。

**---**

**# 9. 日付処理**

「日付」と「経過時間」を別概念として扱う。

日付変更時には以下を更新する。

\`\`\`text
daily action count reset
daily report initialization
login streak calculation
\`\`\`

**---**

**# 10. 連続ログイン**

ユーザーがBitPetを起動した日をログイン日とする。

例えば:

\`\`\`text
8/1 login
8/2 login
8/3 login
\`\`\`

なら

\`\`\`text
streak = 3
\`\`\`

8/4に起動せず8/5に起動した場合:

\`\`\`text
streak = 1
\`\`\`

ただしユーザーを過度に罰するシステムにしない。

連続ログインは主に記録・演出用途とする。

強力なゲーム報酬とは結びつけすぎない。

**---**

**# 11. 行動回数**

1日に何度もコマンドを叩くゲームにしない。

目安:

\`\`\`text
feed: 3回
play: 3回
\`\`\`

程度。

上限到達時:

\`\`\`text
Mochi looks full.

Maybe try again tomorrow.
\`\`\`

のようなメッセージを表示する。

**---**

**# 12. 進化システム**

進化は単純なレベルのみでは決定しない。

以下を参考に進化先を決定する。

\`\`\`text
feed回数
play回数
adventure回数
mood
activity tendency
genetics
\`\`\`

例えば:

\`\`\`rust
struct EvolutionScore {
    active: i32,
    gentle: i32,
    brave: i32,
    lazy: i32,
}
\`\`\`

ユーザーの育て方によってポイントが蓄積する。

例:

\`\`\`text
Play
active +2

Feed
gentle +1

Battle
brave +3

放置
lazy +1
\`\`\`

一定条件を満たしたとき進化判定を行う。

**---**

**# 13. 進化ツリー**

MVPでは分岐数を少なくする。

例:

\`\`\`text
                  Baby
             ┌────┼────┐
             │    │    │
           Fluffy Sharp Weird
             │    │    │
            ...  ...  ...
\`\`\`

最初の進化では3〜5種類程度。

後から追加できるデータ構造とする。

**---**

**# 14. お出かけ**

一定成長後に解禁。

例:

\`\`\`text
Stage 1
\`\`\`

以降。

**---**

**## 14.1 お出かけ種類**

MVP:

\`\`\`text
Explore
\`\`\`

のみでもよい。

将来的に:

\`\`\`text
Explore
Battle
Meet Friend
Treasure Hunt
\`\`\`

などを追加可能にする。

**---**

**# 15. 非同期風ゲームシステム**

お出かけ開始時:

\`\`\`rust
struct Expedition {
    expedition\_type: ExpeditionType,

    started\_at: Timestamp,
    returns\_at: Timestamp,

    seed: u64,
}
\`\`\`

結果は開始時に確定してもよい。

ただしユーザーには帰還時まで表示しない。

**---**

**## 15.1 帰還判定**

BitPet起動時:

\`\`\`text
if now >= returns\_at
\`\`\`

の場合、

\`\`\`text
expedition complete
\`\`\`

として結果を反映する。

バックグラウンド処理は不要。

**---**

**# 16. お出かけ中**

例:

\`\`\`text
   z
  z
 (\\\_/)
 ( -.-)
 / >👜

Mochi is exploring.

Returns in:
1h 24m
\`\`\`

この間は、

\`\`\`text
feed
play
\`\`\`

などを禁止してもよい。

**---**

**# 17. 日次レポート**

その日の行動を記録する。

\`\`\`rust
struct DailyReport {
    date: Date,

    feed\_count: u32,
    play\_count: u32,
    expedition\_count: u32,

    experience\_gained: u32,

    events: Vec\<ReportEvent>,
}
\`\`\`

**---**

**# 18. イベントログ**

ユーザーの行動だけでなく小さな出来事を記録する。

例:

\`\`\`text
09:12 Woke up
09:15 Ate breakfast
12:34 Played with you
14:00 Went exploring
16:12 Came home
\`\`\`

これによってDaily Reportを生成する。

**---**

**# 19. ランダムイベント**

起動時や時間経過時に低確率で発生する。

例:

\`\`\`text
Mochi found a leaf.

Mochi seems interested in something outside.

Mochi had a strange dream.
\`\`\`

ゲームへの影響は小さくする。

BitPetの「生きている感じ」を出す目的。

**---**

**# 36. MVP範囲**

最初のリリースでは以下だけ実装する。

**### 必須**

\`\`\`text
初回起動
卵生成
孵化
status
feed
play
時間経過
経験値
レベル
簡単な進化
連続ログイン
Daily Report
save/load
\`\`\`

**### MVP後**

\`\`\`text
お出かけ
討伐
アイテム
複雑な進化
大量のキャラクター
Wasm
Web UI
オンライン要素
\`\`\`

お出かけについては、プロジェクトの魅力を確認するためMVPへ前倒ししてもよい。

**---**

**# 39. 非目標**

現段階では以下を実装しない。

\`\`\`text
アカウント
ログイン
クラウド保存
サーバー
PvP
リアルタイム通信
課金
広告
AI会話
\`\`\`

BitPet単体で完結することを優先する。

**---**

**# 40. UX原則**

このゲームで最も重要なのは、

\`\`\`text
何度も開きたくなる
\`\`\`

ことではなく、

\`\`\`text
たまに開いたときに嬉しい
\`\`\`

ことである。

ユーザーに張り付きを要求しない。

通知やログインボーナスによってプレッシャーを与えない。

数時間放置したことによる大きな罰を作らない。

仕事中に安心して放置できるゲームにする。

**---**

**# 41. ゲームのコアループ**

\`\`\`text
BitPetを開く

      ↓

ペットの様子を見る

      ↓

少し世話をする
  ↙       ↘
Feed      Play

      ↓

仕事へ戻る

      ↓

時間経過

      ↓

再びBitPetを開く
\`\`\`

成長後:

\`\`\`text
BitPetを開く
      ↓
お出かけさせる
      ↓
仕事へ戻る
      ↓
数時間経過
      ↓
帰還結果を見る
\`\`\`

このループを壊さないこと。

**---**

**# 42. 完成条件**

CLI版v0.1の完成条件は、

\`\`\`bash
bitpet
\`\`\`

を初めて実行するとペットが誕生し、

別の時間に再度実行した際に時間経過が反映され、

\`\`\`bash
bitpet feed
bitpet play
bitpet report
\`\`\`

によって世話と記録ができ、

数日プレイすることでペットが成長することである。

さらに、

\`\`\`text
アプリを終了しても状態が保存される
常駐プロセスが不要
ネットワーク接続が不要
\`\`\`

こと。

**---**
