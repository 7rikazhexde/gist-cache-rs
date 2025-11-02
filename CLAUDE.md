# CLAUDE.md

このファイルは、Claude Code (claude.ai/code) がこのリポジトリで作業する際のガイダンスを提供します。

## プロジェクト概要

**gist-cache-rs** は、GitHub Gistを効率的にキャッシュ・検索・実行するためのRust製CLIツールです。高速な差分更新、複数言語のスクリプト実行サポート、コンテンツキャッシュ機能を提供します。

**対応プラットフォーム**: Linux と macOS のみ（Windows対応は将来予定）

**サポート対象インタープリタ**: bash, sh, zsh, python3, ruby, node, php, perl, pwsh (PowerShell Core), uv

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

# キャッシュ管理
cargo run -- cache list
cargo run -- cache size
cargo run -- cache clear
```

## アーキテクチャ概要

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
- `runner.rs`: `ScriptRunner`は複数インタープリタ実行（bash、python、ruby、node、php、perl、uv）を処理
- stdin ベースとファイルベースの両方の実行モードをサポート
- `uv`インタープリタはPEP 723メタデータサポートのためにファイルベース実行を使用
- `pwsh`（PowerShell Core）と`powershell`（Windows PowerShell）はスクリプト実行ポリシーとの互換性のためファイルベース実行を使用
- `read`などを使用するスクリプト用のインタラクティブモード

**`config.rs`** - 設定管理
- キャッシュパスを管理：`~/.cache/gist-cache/cache.json` と `~/.cache/gist-cache/contents/`

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
   - その他: 標準的なstdinベース実行

5. **検索モード**: 柔軟な検索をサポート:
   - `Auto`: クエリがGist ID（32文字の16進数）か検出し、またはファイル名/説明文を検索
   - `Id`: 直接ID検索
   - `Filename`: ファイル名のみを検索
   - `Description`: 説明文のみを検索

6. **--forceオプション**: `run`コマンドで`--force`を指定すると、実行前に自動的に`update`コマンドを実行（差分更新）し、最新のGist情報を取得してから実行。更新されたGistは自動的に最新版が取得される

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
