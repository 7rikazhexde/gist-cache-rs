# テストインベントリ - 全テストの分類と概要

**作成日**: 2025-11-06
**総テスト数**: 153個
**カバレッジ**: 68.95% (533/773行)

---

## 📋 テストの分類

### テストピラミッド構成

| テスト種別 | 数 | 配置 |
|-----------|-----|------|
| **ユニットテスト** | 120個 | `src/` 内の `#[cfg(test)]` |
| **統合テスト** | 33個 | `tests/` ディレクトリ |
| **E2Eテスト** | 26ケース | `docs/tests/` (手動) |
| **合計** | **153個** | - |

---

## 🧪 1. ユニットテスト (120個)

### 1.1 cache/types.rs (11個)

**場所**: `src/cache/types.rs` の `#[cfg(test)]` モジュール

**目的**: データ構造のシリアライズ/デシリアライズ、型変換

| テスト名 | 目的 | 重要度 |
|---------|------|--------|
| `test_gist_cache_serialization` | GistCacheのJSON変換 | ⭐⭐⭐ |
| `test_datetime_serialization` | 日時のシリアライズ（サブ秒なし） | ⭐⭐⭐ |
| `test_datetime_deserialization` | 日時のデシリアライズ | ⭐⭐⭐ |
| `test_gist_info_without_description` | 説明なしGistの処理 | ⭐⭐ |
| `test_github_gist_to_gist_info` | GitHub API → 内部型変換 | ⭐⭐⭐ |
| `test_gist_file_clone` | GistFileのクローン | ⭐ |
| `test_cache_metadata` | メタデータの構造 | ⭐⭐ |
| その他 | 基本的な型操作 | ⭐ |

**カバレッジ**: 100% (15/15行)

**Windows対応**: ✅ 完全対応（ファイルI/O依存なし）

---

### 1.2 config.rs (5個)

**場所**: `src/config.rs` の `#[cfg(test)]` モジュール

**目的**: 設定管理、プラットフォーム別パス生成

| テスト名 | 目的 | 重要度 |
|---------|------|--------|
| `test_config_new` | Configの初期化 | ⭐⭐⭐ |
| `test_config_default` | デフォルト設定 | ⭐⭐⭐ |
| `test_ensure_cache_dir` | キャッシュディレクトリ作成 | ⭐⭐⭐ |
| `test_ensure_download_dir` | ダウンロードディレクトリ作成 | ⭐⭐ |
| `test_cache_exists` | キャッシュファイル存在確認 | ⭐⭐ |

**カバレッジ**: 96.15% (25/26行)

**Windows対応**: ✅ 完全対応（プラットフォーム別パス生成をテスト）

---

### 1.3 cache/content.rs (18個)

**場所**: `src/cache/content.rs` の `#[cfg(test)]` モジュール

**目的**: コンテンツキャッシュの読み書き、ディレクトリ管理

| テスト名 | 目的 | 重要度 |
|---------|------|--------|
| `test_write_and_read` | 基本的な読み書き | ⭐⭐⭐ |
| `test_read_nonexistent_file` | 存在しないファイルのエラー処理 | ⭐⭐⭐ |
| `test_write_creates_gist_directory` | ディレクトリ自動作成 | ⭐⭐⭐ |
| `test_overwrite_existing_file` | ファイル上書き | ⭐⭐ |
| `test_multiple_files_in_same_gist` | 複数ファイル管理 | ⭐⭐⭐ |
| `test_delete_gist` | Gistキャッシュ削除 | ⭐⭐ |
| `test_delete_gist_already_deleted` | 既削除Gistの処理 | ⭐ |
| `test_list_cached_gists` | キャッシュ一覧取得 | ⭐⭐ |
| `test_list_cached_gists_when_no_cache_dir` | ディレクトリなしの処理 | ⭐⭐ |
| `test_list_cached_gists_with_file_in_contents_dir` | 予期しないファイルの処理 | ⭐⭐ |
| `test_total_size` | キャッシュサイズ計算 | ⭐⭐ |
| `test_total_size_when_no_cache_dir` | ディレクトリなしのサイズ | ⭐ |
| `test_clear_all` | 全キャッシュ削除 | ⭐⭐ |
| `test_clear_all_when_empty` | 空ディレクトリの削除 | ⭐ |
| `test_cache_path_generation` | パス生成の正確性 | ⭐⭐ |
| `test_self_healing_unexpected_files` | 異常ファイルの自動修復 | ⭐⭐ |
| 他2個 | エッジケース | ⭐ |

**カバレッジ**: 83.54% (66/79行)

**Windows対応**: ✅ 完全対応（tempfileでクロスプラットフォーム）

---

### 1.4 cli.rs (21個)

**場所**: `src/cli.rs` の `#[cfg(test)]` モジュール

**目的**: CLI引数処理、インタープリタ解析、キャッシュコマンド

| テスト名 | 目的 | 重要度 |
|---------|------|--------|
| `test_parse_interpreter_bash` | bashインタープリタ解析 | ⭐⭐⭐ |
| `test_parse_interpreter_python` | pythonインタープリタ解析 | ⭐⭐⭐ |
| `test_parse_interpreter_node` | nodeインタープリタ解析 | ⭐⭐ |
| `test_parse_interpreter_ruby` | rubyインタープリタ解析 | ⭐⭐ |
| `test_parse_interpreter_php` | phpインタープリタ解析 | ⭐⭐ |
| `test_parse_interpreter_perl` | perlインタープリタ解析 | ⭐⭐ |
| `test_parse_interpreter_pwsh` | pwshインタープリタ解析 | ⭐⭐ |
| `test_parse_interpreter_powershell` | powershellインタープリタ解析 | ⭐⭐ |
| `test_parse_interpreter_ts_node` | ts-nodeインタープリタ解析 | ⭐⭐ |
| `test_parse_interpreter_deno` | denoインタープリタ解析 | ⭐⭐ |
| `test_parse_interpreter_bun` | bunインタープリタ解析 | ⭐⭐ |
| `test_parse_interpreter_uv` | uvインタープリタ解析 | ⭐⭐ |
| `test_parse_interpreter_poetry` | poetryインタープリタ解析 | ⭐⭐ |
| `test_parse_interpreter_sh` | shインタープリタ解析 | ⭐⭐ |
| `test_parse_interpreter_zsh` | zshインタープリタ解析 | ⭐⭐ |
| `test_parse_interpreter_python_alias` | python3エイリアス | ⭐⭐ |
| `test_parse_interpreter_none` | インタープリタなし | ⭐⭐ |
| `test_parse_interpreter_custom_invalid` | 無効なカスタムインタープリタ | ⭐⭐⭐ |
| `test_format_bytes` | バイトフォーマット | ⭐⭐ |
| `test_format_bytes_edge_cases` | バイト境界値 | ⭐⭐ |
| `test_format_bytes_multiple_gb` | GB単位表示 | ⭐ |
| `test_handle_cache_command_list_empty` | 空キャッシュ一覧 | ⭐⭐ |
| `test_handle_cache_command_list_with_cache` | キャッシュ一覧表示 | ⭐⭐⭐ |
| `test_handle_cache_command_list_no_metadata` | メタデータなし一覧 | ⭐⭐ |
| `test_handle_cache_command_size` | キャッシュサイズ表示 | ⭐⭐ |
| `test_handle_cache_command_clean` | キャッシュクリーン（未実装） | ⭐ |
| `test_run_gist_cache_not_found` | キャッシュ未検出エラー | ⭐⭐⭐ |
| `test_run_gist_no_results` | 検索結果なしエラー | ⭐⭐⭐ |
| `test_run_gist_with_filename_search` | ファイル名検索モード | ⭐⭐⭐ |
| `test_run_gist_with_description_search` | 説明文検索モード | ⭐⭐⭐ |
| `test_print_run_help` | ヘルプ表示 | ⭐ |

**カバレッジ**: 78.16% (161/206行)

**Windows対応**: ✅ 完全対応

---

### 1.5 search/query.rs (26個)

**場所**: `src/search/query.rs` の `#[cfg(test)]` モジュール

**目的**: 検索機能、ID/ファイル名/説明文検索

| テスト名 | 目的 | 重要度 |
|---------|------|--------|
| `test_search_by_id` | ID検索 | ⭐⭐⭐ |
| `test_search_id_no_match` | ID検索マッチなし | ⭐⭐ |
| `test_search_by_filename` | ファイル名検索 | ⭐⭐⭐ |
| `test_search_by_filename_case_insensitive` | 大小文字非依存 | ⭐⭐⭐ |
| `test_search_filename_multiple_files` | 複数ファイルGist検索 | ⭐⭐⭐ |
| `test_search_by_description` | 説明文検索 | ⭐⭐⭐ |
| `test_search_by_description_case_insensitive` | 説明大小文字非依存 | ⭐⭐⭐ |
| `test_search_by_description_no_description` | 説明なしGist | ⭐⭐ |
| `test_search_both` | ファイル名+説明文検索 | ⭐⭐⭐ |
| `test_search_both_filename_only_match` | ファイル名のみマッチ | ⭐⭐ |
| `test_search_both_description_only_match` | 説明のみマッチ | ⭐⭐ |
| `test_search_both_multiple_matches` | 複数マッチ | ⭐⭐ |
| `test_search_both_no_description` | 説明なし時のBoth検索 | ⭐⭐ |
| `test_search_mode_auto_detects_id` | Auto: ID検出 | ⭐⭐⭐ |
| `test_search_mode_auto_detects_both` | Auto: キーワード検出 | ⭐⭐⭐ |
| `test_search_mode_auto_with_short_hex` | Auto: 31文字16進数 | ⭐⭐⭐ |
| `test_search_mode_auto_with_non_hex` | Auto: 非16進数 | ⭐⭐⭐ |
| `test_search_with_auto_mode_id` | Autoモード: ID | ⭐⭐ |
| `test_search_with_auto_mode_keyword` | Autoモード: キーワード | ⭐⭐ |
| `test_search_no_results` | 検索結果なし | ⭐⭐⭐ |
| `test_search_empty_gist_list` | 空Gistリスト | ⭐⭐ |
| `test_select_from_single_result` | 単一結果の選択 | ⭐⭐ |
| `test_select_from_empty_results` | 空結果の選択 | ⭐⭐ |
| 他3個 | エッジケース | ⭐ |

**カバレッジ**: 70.59% (48/68行)

**Windows対応**: ✅ 完全対応

---

### 1.6 cache/update.rs (16個)

**場所**: `src/cache/update.rs` の `#[cfg(test)]` モジュール

**目的**: キャッシュ更新ロジック、差分更新、MockGitHubClient

| テスト名 | 目的 | 重要度 |
|---------|------|--------|
| `test_updater_new` | CacheUpdaterの初期化 | ⭐⭐ |
| `test_save_and_load_cache` | キャッシュ保存・読込 | ⭐⭐⭐ |
| `test_load_cache_missing_file` | 欠損ファイルの処理 | ⭐⭐⭐ |
| `test_save_cache_invalid_json` | 無効JSON検出 | ⭐⭐ |
| `test_update_force_with_mock` | 強制更新（Mock） | ⭐⭐⭐ |
| `test_update_differential_with_mock` | 差分更新（Mock） | ⭐⭐⭐ |
| `test_update_with_no_changes` | 更新なし時の処理 | ⭐⭐ |
| `test_update_with_rate_limit_warning` | レート制限警告 | ⭐⭐⭐ |
| `test_update_auth_failure` | 認証失敗エラー | ⭐⭐⭐ |
| `test_update_with_gist_modification_deletes_cache` | Gist更新時のキャッシュ削除 | ⭐⭐⭐ |
| `test_cache_updater_with_verbose` | verboseモード | ⭐⭐ |
| `test_update_verbose_mode` | verbose詳細ログ | ⭐⭐ |
| `test_update_differential_with_existing_cache_verbose` | verbose差分更新 | ⭐⭐ |
| `test_update_with_low_rate_limit_verbose` | verbose低レート制限 | ⭐⭐ |
| `test_update_gist_modification_verbose` | verbose Gist更新 | ⭐⭐ |
| `test_update_force_verbose_without_existing_cache` | verbose新規キャッシュ | ⭐⭐ |

**カバレッジ**: 62.24% (89/143行)

**Windows対応**: ✅ 完全対応

---

### 1.7 execution/runner.rs (10個)

**場所**: `src/execution/runner.rs` の `#[cfg(test)]` モジュール

**目的**: ScriptRunnerの基本機能（ファイル選択、オプション設定）

| テスト名 | 目的 | 重要度 |
|---------|------|--------|
| `test_runner_new` | ScriptRunnerの初期化 | ⭐⭐ |
| `test_run_options` | RunOptionsの設定 | ⭐⭐ |
| `test_run_options_combinations` | オプション組み合わせ | ⭐⭐ |
| `test_run_options_preview_mode` | プレビューモード | ⭐⭐ |
| `test_run_options_download_mode` | ダウンロードモード | ⭐⭐ |
| `test_select_main_file_single_file` | 単一ファイル選択 | ⭐⭐⭐ |
| `test_select_main_file_multiple_files` | 複数ファイル選択 | ⭐⭐⭐ |
| `test_select_main_file_by_interpreter` | インタープリタ別選択 | ⭐⭐⭐ |
| `test_select_main_file_with_explicit_filename` | 明示的ファイル名指定 | ⭐⭐⭐ |
| `test_runner_with_different_interpreters` | 各インタープリタの動作 | ⭐⭐ |
| `test_display_info` | 情報表示 | ⭐ |

**カバレッジ**: 19.88% → 69.01%（統合テストで向上）

**Windows対応**: ✅ 完全対応（実行部分を除く）

---

### 1.8 github/api.rs (1個)

**場所**: `src/github/api.rs` の `#[cfg(test)]` モジュール

**目的**: GitHubApi構造体の初期化（実行テストは#[ignore]）

| テスト名 | 目的 | 重要度 |
|---------|------|--------|
| `test_api_structure` | 構造体の基本機能 | ⭐ |
| `test_check_auth_when_authenticated` (#[ignore]) | gh認証確認 | ⭐⭐⭐ |
| `test_get_user` (#[ignore]) | ユーザー名取得 | ⭐⭐ |
| `test_check_rate_limit` (#[ignore]) | レート制限確認 | ⭐⭐ |
| `test_fetch_gists_without_since` (#[ignore]) | 全Gist取得 | ⭐⭐⭐ |
| `test_fetch_gists_with_since` (#[ignore]) | 差分Gist取得 | ⭐⭐⭐ |

**カバレッジ**: 8.33% (5/60行)

**Windows対応**: ⚠️ 部分対応（gh CLI必要）

---

### 1.9 github/client.rs (5個)

**場所**: `src/github/client.rs` の `#[cfg(test)]` モジュール

**目的**: MockGitHubClientの動作確認

| テスト名 | 目的 | 重要度 |
|---------|------|--------|
| `test_mock_check_auth_success` | Mock認証成功 | ⭐⭐ |
| `test_mock_get_user` | Mockユーザー取得 | ⭐⭐ |
| `test_mock_check_rate_limit` | Mockレート制限 | ⭐⭐ |
| `test_mock_fetch_gists` | Mock Gist取得 | ⭐⭐ |
| `test_mock_fetch_gist_content` | Mockコンテンツ取得 | ⭐⭐ |

**カバレッジ**: 100% (MockGitHubClientの実装)

**Windows対応**: ✅ 完全対応

---

### 1.10 error.rs (4個)

**場所**: `src/error.rs` の `#[cfg(test)]` モジュール

**目的**: エラー型の変換、表示

| テスト名 | 目的 | 重要度 |
|---------|------|--------|
| `test_error_display` | エラー表示 | ⭐⭐ |
| `test_error_from_io` | IO エラー変換 | ⭐⭐ |
| `test_error_from_json` | JSONエラー変換 | ⭐⭐ |
| `test_error_from_reqwest` | HTTPエラー変換 | ⭐ |
| `test_result_type_alias` | Result型エイリアス | ⭐ |

**カバレッジ**: 0% (エラー型は実行時にテスト)

**Windows対応**: ✅ 完全対応

---

## 🔗 2. 統合テスト (12個)

### 2.1 tests/integration_test.rs (6個)

**場所**: `tests/integration_test.rs`

**目的**: 実際のインタープリタ実行の検証

**依存**: bash, python3, node (Unix環境推奨)

| テスト名 | 目的 | fixture | 重要度 |
|---------|------|---------|--------|
| `test_execute_bash_script` | Bash実行 | hello.sh | ⭐⭐⭐ |
| `test_execute_python_script` | Python実行 | hello.py | ⭐⭐⭐ |
| `test_execute_node_script` | Node.js実行 | hello.js | ⭐⭐ |
| `test_execute_with_arguments` | 引数渡し | args_echo.sh | ⭐⭐⭐ |
| `test_execute_failing_script` | エラー検出 | error_exit.sh | ⭐⭐⭐ |
| `test_preview_mode_does_not_execute` | プレビューモード | hello.sh | ⭐⭐⭐ |

**カバレッジへの貢献**: execution/runner.rs +30-40%

**Windows対応**: ⚠️ 条件付き
- bash: Git Bash または WSL必要
- python3: Windows版Python必要
- node: Windows版Node.js必要

---

### 2.2 tests/runner_test.rs (6個)

**場所**: `tests/runner_test.rs`

**目的**: ScriptRunnerの詳細な動作検証

**依存**: bash (Unix環境推奨)

| テスト名 | 目的 | fixture | 重要度 |
|---------|------|---------|--------|
| `test_download_mode_creates_file` | ダウンロード機能 | hello.sh | ⭐⭐⭐ |
| `test_preview_with_download_mode` | プレビュー+DL | hello.sh | ⭐⭐ |
| `test_cache_creation_after_execution` | キャッシュ作成 | hello.sh | ⭐⭐⭐ |
| `test_multiple_files_gist` | 複数ファイル選択 | hello.sh | ⭐⭐⭐ |
| `test_force_file_based_execution` | ファイルベース実行 | hello.sh | ⭐⭐ |
| `test_script_with_empty_arguments` | 空引数処理 | hello.sh | ⭐⭐ |

**カバレッジへの貢献**: execution/runner.rs +10-15%

**Windows対応**: ⚠️ 条件付き（bash必要）

| `args_echo.sh` | 引数渡しテスト | `echo "Arguments: $@"` |
| `error_exit.sh` | エラーテスト | `exit 1` |

**Windows対応**: ⚠️ 改行コード要注意（LF推奨）

---

## 📝 3. E2Eテスト (26ケース、手動)

### 3.1 E2Eテスト概要

**E2Eテストとは**: コマンドラインから実際にバイナリを実行し、完全なユーザーワークフローを検証

**まだ実装していない理由**: 優先度が統合テストより低い

**実装予定**:

```bash
tests/e2e/
├── test_full_workflow.sh        # update → run の完全なフロー
├── test_cache_management.sh     # cache list/size/clear
└── test_option_combinations.sh  # --force, --preview, --download
```

**期待効果**:
- ユーザー体験の完全な保証
- モジュール間の統合確認
- 手動テストの自動化

---

## 📊 テスト分類サマリー

| 分類 | テスト数 | カバレッジ貢献 | Windows対応 | 状態 |
|------|---------|--------------|------------|------|
| **ユニットテスト** | 120個 | 57-60% | ✅ 完全 | ✅ 完了 |
| **統合テスト** | 12個 | +10-11% | ⚠️ 条件付き | ✅ 完了 |
| **E2Eテスト** | 26ケース | +2-3% | ⚠️ 条件付き | 📝 完了 |
| **合計** | 132個 | 68.18% | - | - |

---

## 🎯 テストの重要度基準

| マーク | 意味 | 説明 |
|-------|------|------|
| ⭐⭐⭐ | 必須 | データ損失・クリティカルバグに直結 |
| ⭐⭐ | 推奨 | ユーザー体験・主要機能に影響 |
| ⭐ | オプション | エッジケース・稀な状況 |

---

## 🔍 カバレッジ貢献度

### 高貢献（10%以上）

- **統合テスト（tests/integration_test.rs + runner_test.rs）**: +10.87%
  - execution/runner.rs の実行部分をカバー

### 中貢献（5-10%）

- **cli.rs ユニットテスト**: 全体に約5-6%貢献
- **search/query.rs ユニットテスト**: 全体に約4-5%貢献

### 低貢献（1-5%）

- 各モジュールの個別ユニットテスト

---

## 📝 テスト実装の履歴

### Phase 1-5 (ユニットテスト): 120個

- cache/types.rs: 11個
- config.rs: 5個
- cache/content.rs: 18個
- cli.rs: 21個
- search/query.rs: 26個
- cache/update.rs: 16個
- execution/runner.rs: 10個
- github/client.rs: 5個
- error.rs: 4個
- その他: 4個

**達成カバレッジ**: 57.31%

### Phase 6 (統合テスト): 12個

- tests/integration_test.rs: 6個
- tests/runner_test.rs: 6個
- tests/fixtures/: 5ファイル

**達成カバレッジ**: 68.18% (+10.87%)

### Phase 7 (E2E): 26ケース (手動実行)

**期待カバレッジ**: 70-71% (+2-3%)

---

**最終更新**: 2025-11-06
**カバレッジ**: 68.95% (533/773行)
