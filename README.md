# gist-cache-rs

GitHubのGistを効率的にキャッシュ・検索・実行するためのCLIツール（Rust実装版）

## 特徴

- ⚡ **高速性**: Rustによる実装で、キャッシュ操作と検索が高速
- 🔄 **差分更新**: 効率的な差分キャッシュ更新をサポート
- 💾 **2層キャッシング**: メタデータとコンテンツの両方をキャッシュし、実行を高速化
- 🔍 **多様な検索**: ID、ファイル名、説明文による検索
- ▶️ **実行サポート**: 複数のインタープリタ（bash, python, ruby, node, php, perl, pwsh, TypeScript）に対応
- 💬 **対話モード**: `read`コマンドなどを使用するスクリプトの対話的実行
- 📦 **uv対応**: PEP 723メタデータに対応した実行
- 📥 **ダウンロード機能**: Gistファイルをダウンロードフォルダに保存
- 🗂️ **キャッシュ管理**: 強力なキャッシュ管理コマンドで効率的に運用

本プロジェクトはLinux、macOS、Windows（Windows 10以降）をサポートします。

## 📋 前提条件

- Rust toolchain (1.85以降)
- GitHub CLI (`gh`) - 認証済み

## 🔧 インストール

**セットアップスクリプト（推奨）:**

```bash
# Linux / macOS
git clone https://github.com/7rikazhexde/gist-cache-rs.git
cd gist-cache-rs
./script/setup.sh install

# Windows
git clone https://github.com/7rikazhexde/gist-cache-rs.git
cd gist-cache-rs
.\script\setup.ps1 install
```

対話的に前提条件チェック、ビルド、インストール、初回キャッシュ作成を実行します。

**その他のインストール方法:** [INSTALL.md](docs/INSTALL.md) を参照してください。

## 🚀 [クイックスタート](docs/QUICKSTART.md)

実際の使用例については [EXAMPLES.md](docs/EXAMPLES.md) にもまとめています。

```bash
# 初回キャッシュ作成
gist-cache-rs update

# Gist検索と実行
gist-cache-rs run backup bash

# Python スクリプトを実行
gist-cache-rs run data_analysis.py python3 input.csv
```

## 🔄 キャッシュの更新

```bash
# 差分更新（デフォルト）
gist-cache-rs update

# 詳細表示付き
gist-cache-rs update --verbose

# 強制全件更新
gist-cache-rs update --force
```

## 🔃 アプリケーションの更新

gist-cache-rs自体を最新版に更新できます：

```bash
# 最新版に更新
gist-cache-rs self update

# 更新確認のみ
gist-cache-rs self update --check

# ソースからビルドして更新
gist-cache-rs self update --from-source
```

詳細は [SELF-UPDATE.md](docs/SELF-UPDATE.md) を参照してください。

## 💾 キャッシュの仕組み

gist-cache-rsは2層のキャッシュ構造を持ちます：

### メタデータキャッシュ

- **内容**: Gist ID、ファイル名、説明文、更新日時などのメタ情報
- **更新**: `update`コマンドで差分または全件更新

### コンテンツキャッシュ

- **内容**: 実際のスクリプト本文
- **更新**: 初回実行時に自動作成、Gist更新検出時に自動削除
- **利点**: 2回目以降の実行が約20倍高速化（ネットワークアクセス不要）

### キャッシュの保存場所

**Linux / macOS:**

```text
~/.cache/gist-cache/
├── cache.json                    # メタデータキャッシュ
└── contents/                     # コンテンツキャッシュ
    ├── {gist_id_1}/
    │   └── {filename_1}
    ├── {gist_id_2}/
    │   └── {filename_2}
    └── ...
```

**Windows:**

```text
%LOCALAPPDATA%\gist-cache\
├── cache.json                    # メタデータキャッシュ
└── contents\                     # コンテンツキャッシュ
    ├── {gist_id_1}\
    │   └── {filename_1}
    ├── {gist_id_2}\
    │   └── {filename_2}
    └── ...
```

## 🔍 Gistの検索と実行

### 検索方法

```bash
# キーワード検索（ファイル名または説明文）
gist-cache-rs run backup

# ID直接指定
gist-cache-rs run abc123def456789

# ファイル名で検索
gist-cache-rs run --filename setup.sh

# 説明文で検索
gist-cache-rs run --description "data processor"
```

### インタープリタ指定

```bash
# Bashスクリプトとして実行（デフォルト）
gist-cache-rs run backup bash

# Python3で実行
gist-cache-rs run data-analysis python3

# uvで実行（PEP 723対応）
gist-cache-rs run ml-script uv

# その他のインタープリタ
gist-cache-rs run script ruby
gist-cache-rs run script node
gist-cache-rs run script.ts deno     # TypeScript (Deno)
# ... ruby, perl, php, pwsh, ts-node, bun も対応
```

### 引数の渡し方

```bash
# スクリプトに引数を渡す
gist-cache-rs run backup bash /src /dst

# Python スクリプトに引数
gist-cache-rs run data_analysis.py python3 input.csv --output result.json

# uvで実行時に引数を渡す
gist-cache-rs run ml-training uv --epochs 100 --batch-size 32
```

### 対話モード

```bash
# 対話的なスクリプトを実行（readコマンドなど使用時）
gist-cache-rs run --interactive create-folders

# 短縮形
gist-cache-rs run -i config-tool bash
```

### プレビュー

スクリプトを実行せずに内容を確認できます：

```bash
# 実行せずに内容を表示
gist-cache-rs run --preview backup

# 短縮形
gist-cache-rs run -p data-analysis

# ID直接指定と組み合わせ
gist-cache-rs run -p --id abc123def456

# ファイル名検索と組み合わせ
gist-cache-rs run -p --filename setup.sh
```

**プレビュー表示内容**:

- 説明（Description）
- ファイル名（Files）
- スクリプト全文（構文ハイライトなし）

**用途**:

- スクリプトの内容を確認してから実行
- 引数や設定を確認
- 間違ったスクリプトの実行を防止

### ファイルのダウンロード

Gistファイルをダウンロードフォルダ（`~/Downloads`）に保存できます：

```bash
# 実行後にダウンロード
gist-cache-rs run --download backup bash

# プレビュー後にダウンロード
gist-cache-rs run --preview --download script.py

# ID指定でダウンロード
gist-cache-rs run --download --id abc123def456
```

**特徴**:

- ダウンロードフォルダ（`~/Downloads`）に保存
- 実行可能なスクリプトとは別に、個別に保存したい場合に便利
- ダウンロード時にキャッシュも自動作成され、2回目以降の実行が高速化
- 他のオプション（`--preview`, `--force`, `--interactive`など）と併用可能

**ダウンロードの動作順序**:

1. `--preview --download`: プレビュー表示 → ダウンロード
2. `--force --download`: キャッシュ更新 → 実行 → ダウンロード
3. `--download` のみ: 実行 → ダウンロード

### 強制更新オプション

```bash
# 実行前に最新のGist情報を取得してから実行
# コンテンツキャッシュが更新されている場合は自動的に再取得
gist-cache-rs run --force backup bash

# 説明文検索と組み合わせ
gist-cache-rs run --force --description "data processor" python3
```

## ⌨️ エイリアス設定

より便利に使用するため、お好みのエイリアスを設定できます：

### 自動設定（setup.sh使用時）

インストール時に対話的に設定：

- 推奨エイリアス（`gcrsu`, `gcrsr`）
- カスタムエイリアス名

### 手動設定

```bash
# ~/.bashrc または ~/.zshrc に追加
alias gcrsu='gist-cache-rs update'
alias gcrsr='gist-cache-rs run'

# 反映
source ~/.bashrc
```

使用例：

```bash
gcrsu  # キャッシュ更新
gcrsr backup bash /src /dst  # Gist実行
gcrsr -p script  # プレビュー
gcrsr -i interactive-script  # 対話モード
gcrsr --download backup  # ダウンロード
gcrsr -p --download script  # プレビュー後ダウンロード
```

## 🗑️ アンインストール

### Linux / macOS

```bash
# 自動アンインストール
./script/setup.sh uninstall

# 手動アンインストール
cargo uninstall gist-cache-rs
rm -rf ~/.cache/gist-cache/
```

### Windows

```powershell
# 自動アンインストール
.\script\setup.ps1 uninstall

# 手動アンインストール
cargo uninstall gist-cache-rs
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\gist-cache"
```

## ❓ ヘルプ

```bash
# 全体のヘルプ
gist-cache-rs --help

# サブコマンドのヘルプ
gist-cache-rs update --help
gist-cache-rs run --help
```

## 🔍 トラブルシューティング

### エラー: Cache file not found. Please run 'gist-cache-rs update' first

**解決方法:** `gist-cache-rs update` を実行してキャッシュを作成してください。

### エラー: GitHub CLI (gh) is not authenticated

**解決方法:** `gh auth login` を実行して認証してください。

### 警告: レートリミット残量が50と低いです

**解決方法:** しばらく待ってから再試行するか、強制更新を避けてください。

### command not found: gist-cache-rs

**解決方法:**

- `~/.cargo/bin` がPATHに含まれているか確認
- または `/usr/local/bin` にバイナリをコピー

詳細は [INSTALL.md](docs/INSTALL.md) を参照してください。

## 📁 プロジェクト構成

```bash
gist-cache-rs/
├── Cargo.toml           # プロジェクト設定
├── src/
│   ├── main.rs          # CLIエントリーポイント
│   ├── lib.rs           # ライブラリルート
│   ├── error.rs         # エラー型定義
│   ├── config.rs        # 設定管理
│   ├── cache/           # キャッシュモジュール
│   ├── github/          # GitHub APIモジュール
│   ├── search/          # 検索モジュール
│   └── execution/       # 実行モジュール
├── script/
│   ├── setup.sh         # セットアップスクリプト (Linux/macOS)
│   └── setup.ps1        # セットアップスクリプト (Windows)
└── README.md
```

## 🗂️ キャッシュ管理

実行したGistのコンテンツキャッシュを効率的に管理できます：

```bash
# キャッシュ一覧を表示
gist-cache-rs cache list

# キャッシュサイズを確認
gist-cache-rs cache size

# 孤立キャッシュを削除（未実装）
gist-cache-rs cache clean

# 全キャッシュを削除
gist-cache-rs cache clear
```

### キャッシュの動作

1. **初回実行**: GitHub APIから本文を取得し、実行後にキャッシュを作成
2. **2回目以降**: キャッシュから高速に読み込んで実行（約20倍高速）
3. **Gist更新時**: `update`コマンドが更新を検出し、自動的にキャッシュを削除
4. **更新後の初回実行**: 最新版をAPIから取得し、新しいキャッシュを作成

## 🛠️ 開発環境のセットアップ

このプロジェクトに貢献する場合、以下のツールを使用すると開発体験が向上します。

### pre-commit フックの設定

コードの品質を保つため、[prek](https://github.com/j178/prek)（Rust製の高速pre-commitツール）の使用を推奨します。

**prekのインストール:**

```bash
# cargoでインストール
cargo install --locked prek

# または、システムのパッケージマネージャーで
# macOS: brew install prek
# その他のインストール方法は https://github.com/j178/prek を参照
```

**フックの有効化:**

```bash
# リポジトリルートで実行
prek install
```

**フックの実行内容:**

- `cargo fmt` - コードフォーマット
- `cargo check` - コンパイルチェック
- `cargo clippy` - lint チェック
- `markdownlint` - Markdown linting
- TOML/YAML validation

**手動実行:**

```bash
# 全てのファイルに対して実行
prek run --all-files

# 特定のフックのみ実行
prek run fmt
prek run clippy
```

**従来のpre-commitとの互換性:**

`prek`は従来の`pre-commit`と完全互換なので、既存の`.pre-commit-config.yaml`をそのまま使用できます。

### 開発コマンド

```bash
# 全チェック実行（fmt, lint, test）
just check

# テスト実行
cargo test

# リリースビルド
cargo build --release
```

## 📚 ドキュメント

### ユーザー向け

- [README.md](README.md) - プロジェクト概要と基本機能
- [INSTALL.md](docs/INSTALL.md) - インストール方法
- [QUICKSTART.md](docs/QUICKSTART.md) - 5分で始めるガイド
- [EXAMPLES.md](docs/EXAMPLES.md) - 実例集

### 開発者向け

- [CLAUDE.md](CLAUDE.md) - プロジェクト構造とアーキテクチャ
- [TESTING.md](docs/testing/TESTING.md) - テスト戦略と実行ガイド
- [TEST_INVENTORY.md](docs/testing/TEST_INVENTORY.md) - テストインベントリ（全テスト一覧）
- [COVERAGE.md](docs/testing/COVERAGE.md) - カバレッジ測定ガイド
- [GH_TESTING_EVALUATION.md](docs/testing/GH_TESTING_EVALUATION.md) - GitHub CLI関連テスト評価
- [docs/tests/](docs/tests/) - 機能検証テスト設計書（E2Eテスト）

## 📄 ライセンス

MIT License
