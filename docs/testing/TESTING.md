# テスト戦略と実行ガイド

## 概要

gist-cache-rsは、ユニットテスト、統合テスト、E2Eテストの3層構造でテスト戦略を構成しています。

**現在のカバレッジ**: 68.95% (533/773 lines)
**自動テスト数**: 153個 (ユニット 120 + 統合 33)
**手動E2Eテスト**: 26ケース

---

## テストピラミッド構成

| テスト種別  | 数    | 配置                      | 実行方法               |
| :---------- | :---- | :------------------------ | :--------------------- |
| **ユニットテスト** | 120個 | `src/` 内の `#[cfg(test)]` | `cargo test` (自動) |
| **統合テスト** | 33個 | `tests/` ディレクトリ | `cargo test` (自動) |
| **E2Eテスト** | 26ケース | `docs/tests/` | 手動実行 |
| **合計** | **153個** | - | - |

**テストピラミッドの原則**:

- ユニットテストが最も多い（78%）- 高速、外部依存なし
- 統合テストは中間（22%）- 実際のプロセス実行を検証
- E2Eテストは最小（手動）- ユーザー視点の包括的検証

---

## テスト実行

### 基本的なテスト実行

```bash
# 全テスト実行
cargo test

# 詳細出力付き
cargo test -- --nocapture

# 特定のテストのみ
cargo test test_cache_content
```

### ignore属性のテスト実行

```bash
# ignore属性のテストを含めて実行
cargo test -- --include-ignored

# ignore属性のテストのみ実行
cargo test -- --ignored
```

### カバレッジ測定

```bash
# 標準出力にカバレッジを表示
cargo tarpaulin --out Stdout

# HTMLレポート生成
cargo tarpaulin --out Html --output-dir coverage

# 詳細は docs/testing/COVERAGE.md を参照
```

---

## テスト構成

### 1. ユニットテスト (120個)

**場所**: `src/` 内の `#[cfg(test)]` モジュール

**カバレッジ対象**:

- データ構造とシリアライゼーション (`cache/types.rs`)
- キャッシュ管理ロジック (`cache/content.rs`, `cache/update.rs`)
- 検索ロジック (`search/query.rs`)
- CLI引数処理 (`cli.rs`)
- 設定管理 (`config.rs`)
- エラーハンドリング (`error.rs`)
- 実行ランナーの基本機能 (`execution/runner.rs`)
- GitHub API モック (`github/client.rs`)

**特徴**:

- 高速実行（外部依存なし）
- MockGitHubClientでGitHub API依存を排除
- 自動CI/CD実行可能

### 2. 統合テスト (33個)

**場所**: `tests/` ディレクトリ

#### 2.1 CLIテスト (`tests/cli_tests.rs`) - 15個

- コマンドライン引数処理の検証
- サブコマンド動作検証 (`update`, `run`, `cache`)
- エラーケース検証（認証エラー、キャッシュなしなど）
- フラグ組み合わせ検証 (`--preview`, `--force`, `--filename` など)

#### 2.2 インタープリタ統合テスト (`tests/integration_test.rs`) - 12個

- Bash, Python, Node.js実行テスト
- TypeScript (ts-node, deno, bun) 実行テスト
- Ruby, Perl, PHP 実行テスト
- 引数渡し、エラーハンドリング
- プレビューモードの動作確認

#### 2.3 ランナーテスト (`tests/runner_test.rs`) - 6個

- スクリプト実行ロジックの詳細検証
- キャッシュ作成動作の確認
- ダウンロードモードの動作検証
- force_file_based実行の確認
- 複数ファイルGistの選択ロジック

**特徴**:

- 実際のプロセス実行を検証
- Unix環境のみ実行 (`#[cfg_attr]`で制御)
- インタープリタ未インストール時は自動スキップ

### 3. E2Eテスト (26ケース、手動)

**場所**: `docs/tests/`

**テストセット**:

1. キャッシング機能 (`test_set_01_caching.md`) - 8ケース
2. 検索機能 (`test_set_02_search.md`) - 6ケース
3. インタープリタ (`test_set_03_interpreter.md`) - 7ケース
4. プレビュー機能 (`test_set_04_preview.md`) - 5ケース

**特徴**:

- 実際のGistを使用した包括的検証
- ユーザー視点のワークフロー確認
- 再実行可能な詳細手順付き

---

## テスト方針

### 何をユニットテストでカバーするか

✅ **対象**:

- ビジネスロジック
- データ変換・シリアライゼーション
- エラーハンドリング
- モック可能な外部依存

❌ **対象外**:

- 外部プロセス実行（bash, python等） → 統合テストで検証
- GitHub CLI (`gh`コマンド) → MockGitHubClientで代替、または #[ignore] テスト
- ユーザー入力処理 → E2Eテストで検証

### テストの品質指標

**目標カバレッジ**: 60-70% (CLIツールの標準)
**現在のカバレッジ**: 68.95% ✅ 目標達成

**達成理由**:

- コアロジックは高カバレッジ (types 100%, config 96%, content 83%, cli 78%)
- 外部プロセス依存コード (runner.rs 20%, api.rs 8%) は統合テストで検証
- thin wrapperは低カバレッジで妥当

---

## トラブルシューティング

### テストが filtered out される

**原因**: Unix環境以外で実行、またはインタープリタが未インストール

**解決策**:

- 統合テストはUnix環境を推奨
- インタープリタ (bash, python, node等) をインストール
- または、自動スキップされるのは正常動作

### カバレッジが測定できない

**原因**: tarpaulinが未インストール

**解決策**:

```bash
cargo install cargo-tarpaulin
```

### 統合テストが失敗する

**原因**: インタープリタ (bash, python, node等) が未インストール

**解決策**:

- 必要なインタープリタをインストール
- または、スキップされるのは正常動作

---

## 詳細ドキュメント

- **カバレッジ測定**: [COVERAGE.md](./COVERAGE.md) - 測定方法とモジュール別詳細
- **テストインベントリ**: [TEST_INVENTORY.md](./TEST_INVENTORY.md) - 全テストの分類と概要
- **GitHub CLI関連テスト評価**: [GH_TESTING_EVALUATION.md](./GH_TESTING_EVALUATION.md) - gh コマンドテストの評価

---

## 参考資料

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
- [mockall](https://docs.rs/mockall/latest/mockall/)

---

**最終更新**: 2025-11-06
**現在のカバレッジ**: 68.95%
**自動テスト数**: 153個
**カバー行数**: 533/773 lines
