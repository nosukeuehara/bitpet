# Monster ASCII Assets

このディレクトリには BitPet のモンスター ASCII Art を配置します。

## 役割

- `monsters/<family>/<monster_id>.txt`: 1体につき1ファイルのASCII Art
- `monsters/mod.rs`: ASCII Art のコンパイル時埋め込みと ID 検索
- `monsters/catalog.csv`: 人間が一覧レビューしやすい表
- `monsters/catalog.json`: 将来の移行・生成ツール向けメタデータ
- `monsters/EVOLUTION_TREE.txt`: 進化ツリーの簡易一覧

ゲームルールや進化判定は `src/domain/` が責務を持ちます。
このディレクトリに進化条件の判定ロジックを実装しないでください。

ASCII Art は元テキストの内容を変更せず、個体単位に切り出しています。
