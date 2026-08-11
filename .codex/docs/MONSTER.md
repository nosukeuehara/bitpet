# BitPet — Monster Catalog & Evolution Design

## 1. Purpose

この文書は、BitPet のモンスター分類、進化系統、ASCII Art の配置、および実装時の管理境界を定義する。

現在の `GAME_DESIGN.md` が定義しているライフサイクル:

```text
Egg
 ↓
Baby
 ↓
Stage 1
 ↓
Stage 2
 ↓
Final
```

を維持し、今回追加する28体を7 Familyへ分類する。

孵化直後の `Baby` は全ユーザー共通とし、今回の28体は Stage 1 以降の候補として扱う。

---

## 2. Design Principles

1. **Family と Species を分離する**
   - Family: 系統・生態・見た目の大分類
   - Species: `Mofflet` など具体的な1種

2. **内部IDと表示名を分離する**
   - internal ID: `mofflet`
   - display name: `Mofflet`
   - セーブデータには将来的に安定した internal ID を保存する。

3. **ASCII Art は表示層に置く**
   - 配置先: `src/ascii/monsters/`
   - Domain 層は ASCII 文字列へ依存しない。

4. **進化判定は Domain 層に置く**
   - `src/ascii/monsters/catalog.json` は設計・ツール用メタデータ。
   - ランタイムの進化ルールの唯一の正本にはしない。
   - 実装時は `src/domain/evolution.rs` 等で型安全に管理する。

5. **ASCII Art はセーブしない**
   - セーブデータでは monster/species ID を保持し、表示時にアセットを解決する。

---

## 3. Family Overview

| Family | Theme | Current placeholder affinity |
|---|---|---|
| Fuzz | 毛・丸み・王道ペット | Fluffy |
| Wing | 翼・飛行・探索 | Sharp |
| Drift | 浮遊・雲・漂遊 | Weird |
| Spike | 角・突起・嘴 | Sharp |
| Colony | 分裂・双子・多頭 | Weird |
| Flora | 芽・植物・根 | Fluffy |
| Oddling | 分類不能な謎生物 | Weird |

現在実装されている `Fluffy / Sharp / Weird` は Stage 1 の仮分類として扱い、
複雑な進化ツリー導入時に上記 Family へ段階的に移行する。

---

## 4. Full Evolution Tree

```text
Baby
├─ Fuzz
│  Mofflet -> Fuzzard -> Brumruff
│                    └-> Woolram
├─ Wing
│  Flitter -> Fuzzwing -> Grandwing
│                     └-> Mantara
├─ Drift
│  Bloblet -> Floatle -> Cloudruff
│                  └-> Driftle
├─ Spike
│  Spindle -> Pricklet -> Starwing
│                    └-> Beakruff
├─ Colony
│  Buddle -> Twindle -> Tribble
│                  └-> Cerbloop
├─ Flora
│  Spriglet -> Dewbud -> Bloomuff
│                   └-> Rainroot
└─ Oddling
   Wormlet -> Whiskerp -> Crownruff
                     └-> Manelet
```

---

## 5. Families

## Fuzz Family

毛量と丸みが増えていく、BitPetの王道もふもふ系。

```text
Mofflet -> Fuzzard -> Brumruff / Woolram
```

| ID | Name | Stage | Evolves to |
|---|---|---|---|
| `mofflet` | Mofflet | stage1 | Fuzzard |
| `fuzzard` | Fuzzard | stage2 | Brumruff / Woolram |
| `brumruff` | Brumruff | final | - |
| `woolram` | Woolram | final | - |

**Final branch guideline:** Care寄りでBrumruff、Activity寄りでWoolram。

**Current placeholder affinity:** `Fluffy`

## Wing Family

翼や大きな横幅へ成長する、外出・探索に似合う飛行系。

```text
Flitter -> Fuzzwing -> Grandwing / Mantara
```

| ID | Name | Stage | Evolves to |
|---|---|---|---|
| `flitter` | Flitter | stage1 | Fuzzwing |
| `fuzzwing` | Fuzzwing | stage2 | Grandwing / Mantara |
| `grandwing` | Grandwing | final | - |
| `mantara` | Mantara | final | - |

**Final branch guideline:** Adventure寄りでGrandwing、Balanced寄りでMantara。

**Current placeholder affinity:** `Sharp`

## Drift Family

身体が軽くなり、雲や浮遊生物のようになる漂遊系。

```text
Bloblet -> Floatle -> Cloudruff / Driftle
```

| ID | Name | Stage | Evolves to |
|---|---|---|---|
| `bloblet` | Bloblet | stage1 | Floatle |
| `floatle` | Floatle | stage2 | Cloudruff / Driftle |
| `cloudruff` | Cloudruff | final | - |
| `driftle` | Driftle | final | - |

**Final branch guideline:** Gentle/Rest寄りでCloudruff、Adventure寄りでDriftle。

**Current placeholder affinity:** `Weird`

## Spike Family

角・突起・嘴など輪郭が鋭く発達していく系統。

```text
Spindle -> Pricklet -> Starwing / Beakruff
```

| ID | Name | Stage | Evolves to |
|---|---|---|---|
| `spindle` | Spindle | stage1 | Pricklet |
| `pricklet` | Pricklet | stage2 | Starwing / Beakruff |
| `starwing` | Starwing | final | - |
| `beakruff` | Beakruff | final | - |

**Final branch guideline:** Brave寄りでStarwing、Care寄りでBeakruff。

**Current placeholder affinity:** `Sharp`

## Colony Family

分裂・双子化・多頭化によって個体数が増えていく系統。

```text
Buddle -> Twindle -> Tribble / Cerbloop
```

| ID | Name | Stage | Evolves to |
|---|---|---|---|
| `buddle` | Buddle | stage1 | Twindle |
| `twindle` | Twindle | stage2 | Tribble / Cerbloop |
| `tribble` | Tribble | final | - |
| `cerbloop` | Cerbloop | final | - |

**Final branch guideline:** Play/Social寄りでTribble、Battle/Brave寄りでCerbloop。

**Current placeholder affinity:** `Weird`

## Flora Family

芽・根・植物のようなパーツが育つ穏やかな系統。

```text
Spriglet -> Dewbud -> Bloomuff / Rainroot
```

| ID | Name | Stage | Evolves to |
|---|---|---|---|
| `spriglet` | Spriglet | stage1 | Dewbud |
| `dewbud` | Dewbud | stage2 | Bloomuff / Rainroot |
| `bloomuff` | Bloomuff | final | - |
| `rainroot` | Rainroot | final | - |

**Final branch guideline:** Care寄りでBloomuff、Rest/Low-activity寄りでRainroot。

**Current placeholder affinity:** `Fluffy`

## Oddling Family

既存の生物分類に収まりにくい、BitPetらしい謎生物系。

```text
Wormlet -> Whiskerp -> Crownruff / Manelet
```

| ID | Name | Stage | Evolves to |
|---|---|---|---|
| `wormlet` | Wormlet | stage1 | Whiskerp |
| `whiskerp` | Whiskerp | stage2 | Crownruff / Manelet |
| `crownruff` | Crownruff | final | - |
| `manelet` | Manelet | final | - |

**Final branch guideline:** 特殊条件・同点・低頻度条件などの受け皿にする。

**Current placeholder affinity:** `Weird`


---

## 6. Evolution Condition Policy

進化先は単純なレベルだけで決めず、既存 `GAME_DESIGN.md` の方針どおり、
育て方の累積値を使う。

候補:

- `feed_total`
- `play_total`
- `expedition_count`
- mood / energy の傾向
- active / gentle / brave / lazy 等の evolution score
- genetics / seed

Stage 1 への進化時に Family を決定し、その後は Family 内で成長する方式を基本案とする。

```text
Baby
  ↓ family selection
Stage 1 species
  ↓ growth
Stage 2 species
  ↓ play style / score branch
Final species
```

最終分岐条件は本書の `Final branch guideline` を方向性として採用するが、
具体的な閾値はゲームバランス調整時に決定する。

---

## 7. Compatibility with Current Source

現行コードは:

```text
GrowthStage:
  Egg
  Baby
  Stage1
  Stage2
  Final

MonsterFamily:
  Fuzz
  Wing
  Drift
  Spike
  Colony
  Flora
  Oddling

SpeciesId:
  baby
  28 Stage 1以降のSpecies ID
```

である。

save compatibility のため、`SpeciesId` のserde表現は安定したsnake_case ID (`"mofflet"`) とする。

### Legacy evolution migration

```text
Baby   -> baby
Fluffy -> mofflet
Sharp  -> spindle
Weird  -> wormlet
```

EggはSpeciesそのものではない。Egg中のsaveでは `pet.stage = "Egg"` と `pet.species_id = "baby"` を保持し、孵化後に共通Babyへ移る。

---

## 8. ASCII Asset Management

```text
src/ascii/
├── mod.rs
├── pets.rs                 # current Baby/Fluffy/Sharp/Weird rendering
├── README.md
└── monsters/
    ├── mod.rs
    ├── catalog.csv
    ├── catalog.json
    ├── EVOLUTION_TREE.txt
    ├── fuzz/
    ├── wing/
    ├── drift/
    ├── spike/
    ├── colony/
    ├── flora/
    └── oddling/
```

`src/ascii/monsters/mod.rs` は `include_str!` によってASCIIをバイナリへ埋め込む。
Native / Wasm のどちらでも filesystem 読み込みを要求しない。

---

## 9. Source Index

`source_index` は今回受領したテキストファイルで上から数えた1始まりの番号。
これは制作・レビュー用であり、ゲームの永続IDには使用しない。

| Source | ID | Name | Family | Stage |
|---:|---|---|---|---|
| 1 | `mofflet` | Mofflet | fuzz | stage1 |
| 2 | `bloblet` | Bloblet | drift | stage1 |
| 3 | `brumruff` | Brumruff | fuzz | final |
| 4 | `spriglet` | Spriglet | flora | stage1 |
| 5 | `spindle` | Spindle | spike | stage1 |
| 6 | `manelet` | Manelet | oddling | final |
| 7 | `bloomuff` | Bloomuff | flora | final |
| 8 | `woolram` | Woolram | fuzz | final |
| 9 | `cloudruff` | Cloudruff | drift | final |
| 10 | `grandwing` | Grandwing | wing | final |
| 11 | `crownruff` | Crownruff | oddling | final |
| 12 | `flitter` | Flitter | wing | stage1 |
| 13 | `rainroot` | Rainroot | flora | final |
| 14 | `buddle` | Buddle | colony | stage1 |
| 15 | `cerbloop` | Cerbloop | colony | final |
| 16 | `fuzzwing` | Fuzzwing | wing | stage2 |
| 17 | `twindle` | Twindle | colony | stage2 |
| 18 | `dewbud` | Dewbud | flora | stage2 |
| 19 | `floatle` | Floatle | drift | stage2 |
| 20 | `whiskerp` | Whiskerp | oddling | stage2 |
| 21 | `mantara` | Mantara | wing | final |
| 22 | `pricklet` | Pricklet | spike | stage2 |
| 23 | `fuzzard` | Fuzzard | fuzz | stage2 |
| 24 | `wormlet` | Wormlet | oddling | stage1 |
| 25 | `tribble` | Tribble | colony | final |
| 26 | `starwing` | Starwing | spike | final |
| 27 | `driftle` | Driftle | drift | final |
| 28 | `beakruff` | Beakruff | spike | final |

---

## 10. Next Implementation Step

このZIPを配置した時点では、既存の `pet_art()` の挙動は変更しない。
そのため既存ユーザーの `Baby / Fluffy / Sharp / Weird` 表示は維持される。

次の開発Phaseで:

1. Domainへ `MonsterFamily` / `SpeciesId` を追加
2. save schema migration を設計
3. Stage 2 / Final を `GrowthStage` に追加
4. Family 選択ルールを実装
5. Final 分岐ルールを実装
6. CLI renderer を `species_id -> art_by_id()` に切り替える
7. 進化・save migration・Wasmの回帰テストを追加

の順で導入する。
