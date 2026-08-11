# BitPet — CLI / UX Specification

**# 4. CLI仕様**

基本コマンド:

\`\`\`bash
bitpet
\`\`\`

引数なしの場合は現在のペット状態を表示する。

例:

\`\`\`text
   /\\\_/\\
  ( o.o )
   \> ^ <

Mochi
Fuzzard
Lv. 3

Family   : Fuzz
Stage    : Stage 2
Mood     : Happy
Hunger   : 72%
Energy   : 61%

Last seen: 2h ago

What will you do?

1\. Feed
2\. Play
3\. Go out
4\. Report
\`\`\`

**---**

**## 4.1 コマンド一覧**

**### status**

\`\`\`bash
bitpet status
\`\`\`

ペットの現在状態を表示する。

\`bitpet\` 単体も同じ動作としてよい。

Egg状態の例:

\`\`\`text
   __
 /    \
 \____/

Egg

Hatching in 1h 0m
\`\`\`

**---**

**### feed**

\`\`\`bash
bitpet feed
\`\`\`

食事を与える。

効果:

\`\`\`text
hunger +
mood +
\`\`\`

1日の利用回数には上限を設定する。

Egg中は実行できない。

**---**

**### play**

\`\`\`bash
bitpet play
\`\`\`

遊ぶ。

効果:

\`\`\`text
mood +
energy -
experience +
\`\`\`

Egg中は実行できない。

**---**

**### go**

\`\`\`bash
bitpet go
\`\`\`

お出かけする。

成長後にアンロックされる。

外出中は一部アクションを使用できない。

Egg中は実行できない。

例:

\`\`\`text
Mochi went exploring.

Expected return:
14:32
\`\`\`

**---**

**### report**

\`\`\`bash
bitpet report
\`\`\`

その日の行動履歴を表示する。

例:

\`\`\`text
BitPet Daily Report

Feed        2
Play        1
Adventure   1

EXP gained  18
Mood        +12

Mochi found:
Small Acorn

Today Mochi seemed happy.
\`\`\`

**---**

**### streak**

\`\`\`bash
bitpet streak
\`\`\`

連続ログイン日数を表示する。

**---**

**### version**

\`\`\`bash
bitpet --version
\`\`\`

**---**

**# 20. キャラクター表示**

ASCII Artを使用する。

v0.2.0以降は、Baby以外の表示を `SpeciesId` から解決する。

Monster ASCII Artの配置とID対応は以下をsource of truthとする。

\`\`\`text
.codex/docs/MONSTER.md
\`\`\`

CLI statusでは必要に応じて以下を表示する。

\`\`\`text
Species name
Family
Growth Stage
Level
\`\`\`

Egg状態ではSpecies / Family / Level / status値を前面に出さず、Egg表示と孵化までの残り時間を簡潔に表示する。

帰還予定やDaily Report event時刻はユーザーのlocal timeで表示する。

save dataにはlocal time文字列を保存しない。

表示サイズは小さめを基本とする。

目安:

\`\`\`text
横幅 5〜15文字
高さ 3〜8行
\`\`\`

CLIを圧迫する巨大AAは避ける。

**---**

**# 21. キャラクターデザイン方針**

BitPetのキャラクターは、

\`\`\`text
動物そのもの
\`\`\`

である必要はない。

例えば:

\`\`\`text
うさぎ
フクロウ
きのこ
雲
ブロッコリー
妖精
コウモリ
ドラゴン
双子
顔だけの謎生物
\`\`\`

など。

「何の生き物なのか少し分からない」ものも積極的に採用する。

特に、

\- 小さい
\- 顔が主体
\- 丸みがある
\- 記号数が少ない
\- 一目で識別できる

デザインを優先する。

**---**

**# 27. ユーザー向けエラー**

内部エラーをそのまま表示しない。

悪い例:

\`\`\`text
thread 'main' panicked at unwrap()
\`\`\`

良い例:

\`\`\`text
BitPet couldn't read your save data.

Save file:
\~/.bitpet/save.json
\`\`\`

必要であればdebugモードのみ詳細を出す。

**---**
