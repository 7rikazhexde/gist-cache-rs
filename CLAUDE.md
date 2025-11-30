# CLAUDE.md

このファイルは、Claude Code (claude.ai/code) がこのリポジトリで作業する際のガイダンスを提供します。

## プロジェクト概要

**gist-cache-rs** は、GitHub Gistを効率的にキャッシュ・検索・実行するためのRust製CLIツールです。高速な差分更新、複数言語のスクリプト実行サポート、コンテンツキャッシュ機能を提供します。

**対応プラットフォーム**: Linux、macOS、Windows 10以降

**サポート対象インタープリタ**: bash, sh, zsh, python3, ruby, node, php, perl, pwsh (PowerShell Core), TypeScript (ts-node, deno, bun), uv

## 開発コマンド

### ビルドとテスト

```bash
# 開発ビルド
cargo build

# リリースビルド（最適化済み）
cargo build --release

# ローカルインストール
cargo install --path .

# テスト実行
cargo test

# 詳細出力付きテスト実行
cargo test -- --nocapture
```

### コード品質チェック（justfile経由）

```bash
# 全チェックを実行（フォーマット、lint、テスト）
just check

# フォーマットチェックのみ
just fmt-check

# clippyでlint
just lint

# テストを静かに実行
just test

# コードを自動フォーマット
just fmt

# CI用チェック（警告をエラーとして扱う）
just ci-check
```

### アプリケーションの実行

```bash
# キャッシュ更新
cargo run -- update
cargo run -- update --force
cargo run -- update --verbose

# Gist実行
cargo run -- run <query> [interpreter] [args...]
cargo run -- run --preview <query>
cargo run -- run --interactive <query>
cargo run -- run --force <query>  # 実行前にキャッシュ更新
cargo run -- run --download <query>  # ダウンロードフォルダに保存

# キャッシュ管理
cargo run -- cache list
cargo run -- cache size
cargo run -- cache clear

# アプリケーション自体の更新
cargo run -- self update --check  # 更新確認のみ
cargo run -- self update          # 最新版に更新
cargo run -- self update --verbose
```

## アーキテクチャ概要

### ファイル構造

```bash
src/
├── cache/              # キャッシュ管理層
│   ├── content.rs      # コンテンツキャッシュ (541行)
│   ├── types.rs        # データ型定義 (246行)
│   ├── update.rs       # 差分更新ロジック (849行)
│   └── mod.rs
├── github/             # GitHub API統合
│   ├── api.rs          # GitHub CLI wrapper (212行)
│   ├── client.rs       # トレイト定義 (104行)
│   └── mod.rs
├── execution/          # スクリプト実行
│   ├── runner.rs       # マルチインタープリタ実行 (758行)
│   └── mod.rs
├── search/             # 検索機能
│   ├── query.rs        # 検索クエリ処理 (420行)
│   └── mod.rs
├── self_update/        # Self-update機能
│   ├── updater.rs      # アプリ更新ロジック
│   └── mod.rs
├── cli.rs              # CLI引数処理 (967行)
├── config.rs           # 設定管理 (163行)
├── error.rs            # エラー型定義 (160行)
├── lib.rs              # ライブラリルート
└── main.rs             # エントリーポイント

合計: 18ファイル, 約4,600行
```

### モジュール構造

コードベースは明確な関心の分離を持つモジュラーアーキテクチャに従っています：

**`cache/`** - キャッシュ管理層（2層キャッシング構造）
- `types.rs`: コアデータ構造（`GistCache`, `GistInfo`, `GistFile`, `CacheMetadata`）
- `update.rs`: `CacheUpdater`はGitHub APIの`since`パラメータを使用した差分メタデータキャッシュ更新を処理。Gist更新検出時に対応するコンテンツキャッシュを自動削除
- `content.rs`: `ContentCache`は`~/.cache/gist-cache/contents/{gist_id}/{filename}`に個別のGistコンテンツファイルを管理。初回実行時に作成され、2回目以降の実行を高速化（約20倍）

**`github/`** - GitHub API統合
- `api.rs`: `GitHubApi`は認証、レート制限チェック、gist取得のためにGitHub CLI（`gh`）をラップ
- 全てのGitHub操作は直接REST APIコールではなく`gh` CLIを使用

**`search/`** - 検索機能
- `query.rs`: 複数モード（Auto、Id、Filename、Description）を持つ`SearchQuery`を実装
- 番号付きプロンプトを使用したインタラクティブ選択UI

**`execution/`** - スクリプト実行
- `runner.rs`: `ScriptRunner`は複数インタープリタ実行（bash、python、ruby、node、php、perl、pwsh、TypeScript、uv）を処理
- stdin ベースとファイルベースの両方の実行モードをサポート
- `uv`インタープリタはPEP 723メタデータサポートのためにファイルベース実行を使用
- `pwsh`（PowerShell Core）と`powershell`（Windows PowerShell）はスクリプト実行ポリシーとの互換性のためファイルベース実行を使用
- TypeScriptインタープリタ（`ts-node`、`deno`、`bun`）はモジュール解決のためファイルベース実行を使用
- `read`などを使用するスクリプト用のインタラクティブモード

**`self_update/`** - アプリケーション自己更新機能
- `updater.rs`: `Updater`はGitHub Releasesからの自動更新を処理（`self_update` crateを使用）
- 更新確認（`--check`）、強制更新（`--force`）、バージョン指定更新をサポート
- Phase 1: GitHub Releasesからのバイナリダウンロード（実装済み）
- Phase 2: ソースからのビルド更新（`--from-source`、未実装）

**`config.rs`** - 設定管理
- キャッシュパスを管理（プラットフォーム別）：
  - 環境変数`GIST_CACHE_DIR`でオーバーライド可能（テスト用）
  - Unix: `~/.cache/gist-cache/cache.json` と `~/.cache/gist-cache/contents/`
  - Windows: `%LOCALAPPDATA%\gist-cache\cache.json` と `%LOCALAPPDATA%\gist-cache\contents\`
- ダウンロードパスを管理：`dirs::download_dir()`を使用してOSの標準に従う
- テスト環境での分離：`GIST_CACHE_DIR`を設定することで、実際のユーザーキャッシュに影響を与えずにテスト可能

**`error.rs`** - `thiserror`を使用した集中エラー処理

### 主要な設計パターン

1. **差分更新**: メタデータキャッシュ更新はGitHub APIの`since`パラメータを使用して変更されたgistのみを取得。タイムスタンプは`cache.json`の`last_updated`に保存

2. **2層キャッシング（オンデマンド方式）**:
   - **メタデータキャッシュ**: `cache.json`にgistメタデータ（id、description、files、updated_at）を格納。`update`コマンドで更新
   - **コンテンツキャッシュ**: `contents/{gist_id}/{filename}`に実際のスクリプト本文を保存。実行時にオンデマンドで作成し、Gist更新時に自動削除
   - **キャッシュ鮮度管理**: `update`コマンドが新旧メタデータの`updated_at`を比較し、更新されたGistのコンテンツキャッシュディレクトリを削除

3. **GitHub CLI統合**: 認証とAPI アクセスに直接REST APIコールではなく`gh`コマンドを使用

4. **複数インタープリタサポート**: 実行層は異なるインタープリタを抽象化し、特別な処理を実装:
   - シェルスクリプト（bash/sh/zsh）: 直接実行
   - `uv`: PEP 723サポートのため`uv run`コマンドを使用したファイルベース
   - `php`: 信頼性の高い引数処理のための強制ファイルベース実行
   - `pwsh`/`powershell`: スクリプト実行ポリシー互換性のための強制ファイルベース実行
   - TypeScript（`ts-node`、`deno`、`bun`）: モジュール解決とランタイム要件のための強制ファイルベース実行
     - `ts-node`: Node.js上でTypeScriptを実行
     - `deno`: Denoランタイムで`deno run`コマンドを使用
     - `bun`: Bunランタイムで実行
   - その他: 標準的なstdinベース実行

5. **検索モード**: 柔軟な検索をサポート:
   - `Auto`: クエリがGist ID（32文字の16進数）か検出し、またはファイル名/説明文を検索
   - `Id`: 直接ID検索
   - `Filename`: ファイル名のみを検索
   - `Description`: 説明文のみを検索

6. **--forceオプション**: `run`コマンドで`--force`を指定すると、実行前に自動的に`update`コマンドを実行（差分更新）し、最新のGist情報を取得してから実行。更新されたGistは自動的に最新版が取得される

7. **--downloadオプション**: `run`コマンドで`--download`を指定すると、Gistファイルをダウンロードフォルダ（`~/Downloads`）に保存。実行可能なスクリプトキャッシュとは別に、個別に保存したい場合に便利。ダウンロード時にコンテンツキャッシュも自動作成され、2回目以降の実行が高速化。他のオプション（`--preview`, `--force`, `--interactive`など）と併用可能

## 重要な実装詳細

### 日時処理

全てのタイムスタンプは、元のbash実装との互換性を維持するため、サブ秒なしのISO 8601形式（`%Y-%m-%dT%H:%M:%SZ`）を使用。`cache/types.rs`にカスタムシリアライザ/デシリアライザあり。

### キャッシュファイル形式

`cache.json`の構造：

```json
{
  "metadata": {
    "last_updated": "2024-01-01T12:00:00Z",
    "total_count": 42,
    "github_user": "username"
  },
  "gists": [...]
}
```

### 実行モード

- **Stdinモード**（デフォルト）: スクリプトコンテンツをインタープリタに直接パイプ
- **ファイルモード**（uv、php、interactive）: 実行用に一時ファイルを作成
- **インタラクティブモード**（`-i`）: `read`コマンドをサポートするためstdioに`inherit()`を使用
- **プレビューモード**（`-p`/`--preview`）: スクリプトを実行せず、Description、Files、Gist内容のみを表示。検索モード（Auto、ID、Filename、Description）と組み合わせ可能

### プラットフォーム固有の実装

**Windows対応**:
- **パーミッション設定**: 条件付きコンパイル（`#[cfg(unix)]`）を使用し、Unix環境のみで`chmod`を実行。Windowsではファイル拡張子で実行可能性が決定されるため、パーミッション設定は不要
- **パス設定**: `src/config.rs`でプラットフォーム別のキャッシュディレクトリを使用
  - Unix: `~/.cache/gist-cache`
  - Windows: `%LOCALAPPDATA%\gist-cache`（`dirs::cache_dir()`を使用）
  - ダウンロードディレクトリは全プラットフォームで`dirs::download_dir()`を使用
- **インストールスクリプト**: PowerShell版（`script/setup.ps1`）を提供

**クロスプラットフォーム設計**:
- 条件付きコンパイル（`cfg`属性）で明示的に分岐
- プラットフォーム非依存のコードを優先
- 既存のLinux/macOS環境に影響を与えないデグレード防止

### レート制限

Updaterはレート制限をチェックし、残りリクエストが50未満の場合に警告。`update --force`による強制全件更新は全gistを取得するため、大量のレート制限を消費する可能性あり。

### コンテンツキャッシュの動作フロー

1. **初回実行**: GitHub APIから本文を取得し、実行後に`contents/{gist_id}/{filename}`にキャッシュを作成
2. **2回目以降**: キャッシュから読み込んで実行（ネットワークアクセス不要、約20倍高速）
3. **Gist更新時**: `update`コマンドがメタデータの`updated_at`変更を検出し、該当するコンテンツキャッシュディレクトリ（`contents/{gist_id}/`）を自動削除
4. **更新後の初回実行**: 最新版をAPIから取得し、新しいキャッシュを作成

### --forceオプションの動作

`run --force`を指定すると：
1. 実行前に自動的に`update`コマンドを実行（差分更新、`update --force`ではない）
2. Gistが更新されていればコンテンツキャッシュが削除される
3. 最新版を取得して実行
4. 新しいキャッシュを作成

これにより、開発中のGistを頻繁に更新している場合でも、常に最新版を実行できる。

## テスト

テストは非同期関数用に`tokio::test`を使用し、`#[cfg(test)]`を使ってモジュールとインラインで配置。現在`src/cache/content.rs`に最小限のテストカバレッジ。

開発用依存関係：
- `assert_cmd`: CLI統合テスト用
- `tempfile`: 一時テストフィクスチャ用

## キャッシュ管理コマンド

`cache`サブコマンドで実装されたコンテンツキャッシュ管理機能（main.rs:287-412）：

- `cache list`: キャッシュされたGistの一覧表示（ID、説明、ファイル名、更新日時）
- `cache size`: キャッシュディレクトリの合計サイズを表示
- `cache clean`: 孤立キャッシュの削除（未実装、将来予定）
- `cache clear`: 全コンテンツキャッシュを削除（確認プロンプト付き）

`ContentCache`構造体（src/cache/content.rs）が提供するメソッド：
- `list_cached_gists()`: キャッシュ済みGist IDの一覧取得
- `total_size()`: キャッシュディレクトリの合計サイズ計算
- `clear_all()`: 全キャッシュ削除
- `read()`, `write()`, `exists()`: 個別キャッシュの読み書き

## 依存関係

主要なランタイム依存関係：
- `tokio`: 非同期ランタイム
- `reqwest`: HTTPクライアント（未使用、直接API実装時代の名残）
- `serde`/`serde_json`: シリアライゼーション
- `clap`: CLI引数パース
- `chrono`: 日時処理
- `anyhow`/`thiserror`: エラー処理
- `dirs`: プラットフォーム固有のディレクトリ検出
- `colored`: ターミナル出力の色付け
- `self_update`: GitHub Releasesからの自動更新

開発用依存関係：
- `mockall`: モックライブラリ（外部依存のテスト用）
- `tempfile`: 一時ファイル/ディレクトリ（テスト用）
- `assert_cmd`: CLIテスト用（将来の統合テスト向け）

## テストとカバレッジ

### 現在の状況（2025-11-07）

**全体カバレッジ**: 68.95% (533/773 lines)
**テスト数**: 163個（ユニット: 120, 統合: 43、プラットフォーム別にスキップ: 18）
**テスト実行時間**: 約15秒（Windows）、約10秒（Unix）

### モジュール別カバレッジ

| モジュール | カバレッジ | 状態 | 備考 |
|-----------|-----------|------|------|
| `cache/types.rs` | 100.00% | ✅ | コアデータ型 |
| `config.rs` | 96.15% | ✅ | 設定管理 |
| `cache/content.rs` | 83.54% | ✅ | コンテンツキャッシュ |
| `cli.rs` | 78.16% | ✅ | CLI処理 |
| `search/query.rs` | 70.59% | 🟡 | 検索ロジック |
| `cache/update.rs` | 62.24% | 🟡 | 差分更新 |
| `execution/runner.rs` | 19.88% | 🔴 | 外部プロセス依存 |
| `github/api.rs` | 8.33% | 🔴 | gh コマンド依存 |
| `error.rs` | 0.00% | - | 単純な型定義 |
| `main.rs` | 0.00% | - | エントリーポイント |

### テスト方針

**ユニットテストでカバー**:
- ビジネスロジック（検索、キャッシュ管理、データ変換）
- エラーハンドリング
- モック可能な外部依存（MockGitHubClient使用）

**統合テスト/手動テストで検証**:
- GitHub CLI (`gh`)コマンド実行 → `#[ignore]`テストで手動検証可能
- 実際のスクリプト実行（bash, python等） → 統合テストで検証
- ユーザー入力を伴う処理 → E2Eテストで検証

### カバレッジ測定

```bash
# 基本測定
cargo tarpaulin --out Stdout

# HTMLレポート生成
cargo tarpaulin --out Html --output-dir coverage
open coverage/index.html

# CI用（詳細出力）
cargo tarpaulin --out Stdout --output-dir coverage 2>&1 | tail -100
```

詳細は [TESTING.md](docs/testing/TESTING.md)、[COVERAGE.md](docs/testing/COVERAGE.md)、[TEST_INVENTORY.md](docs/testing/TEST_INVENTORY.md) を参照。

### テスト構成

**ユニットテスト (125個)**:
- `src/` 内の `#[cfg(test)]` モジュール
- MockGitHubClient を使用した外部依存の排除
- 高速実行、CI/CD対応

**統合テスト (43個、プラットフォーム依存18個)**:
- `tests/cli_tests.rs`: CLI動作テスト (15個)
- `tests/integration_test.rs`: インタープリタテスト (16個)
  - **Unix専用 (12個)**: Bash, Python, Node.js, Ruby, Perl, PHP, TypeScript (ts-node, deno, bun)
  - **Windows専用 (4個)**: PowerShell Core (pwsh)
- `tests/runner_test.rs`: ランナーテスト (12個)
  - **Unix専用 (6個)**: Bashを使用したテスト
  - **Windows専用 (6個)**: PowerShellを使用したテスト

**E2Eテスト (手動)**:
- `docs/tests/`: 機能検証テスト設計書 (26ケース)
- 実際のGistを使用した包括的検証

### プラットフォーム別テスト戦略

**Unix環境（Linux/macOS）**:
- bashを使用した統合テスト（18個）が実行される
- PowerShellテスト（10個）はコンパイル時に除外される（`#[cfg(windows)]`）
- 合計: 120 + 15 + 18 = **153個のテストが実行**

**Windows環境**:
- PowerShellを使用した統合テスト（10個）が実行される
- bashテスト（18個）は実行時にスキップされる（`#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]`）
- 合計: 120 + 15 + 10 = **145個のテストが実行**

**テスト対等性**:
| テスト種別 | bash (Unix) | PowerShell (Windows) |
|-----------|-------------|---------------------|
| 統合テスト | 12個 | 4個 |
| ランナーテスト | 6個 | 6個 |
| **合計** | **18個** | **10個** |

bashとPowerShellでテストカバレッジが対等になるよう設計されており、両プラットフォームで同等の品質保証を実現しています。

### 設計判断：68.95%カバレッジの内訳

以下のモジュールは外部依存が多く、統合テスト/E2Eテストで品質担保：

1. **execution/runner.rs (758行, 19.88%)**
   - 実際のプロセス実行（bash/python/node等）に依存
   - 統合テストで12言語の実行を検証

2. **github/api.rs (212行, 8.33%)**
   - GitHub CLI (`gh`コマンド)に依存
   - MockGitHubClientでビジネスロジックをカバー

3. **main.rs / error.rs**
   - エントリーポイント、単純な型定義
   - E2Eテストで検証

**結論**: コアビジネスロジックは高カバレッジ（types 100%, config 96%, content 83%, cli 78%）を達成。CLIツールの標準的な60-70%目標を達成し、適切なテスト戦略を実現。
