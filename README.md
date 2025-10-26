# gist-cache-rs

GitHubのGistを効率的にキャッシュ・検索・実行するためのCLIツール（Rust実装版）

## 特徴

- ⚡ **高速性**: Rustによる実装で、キャッシュ操作と検索が高速
- 🔄 **差分更新**: 効率的な差分キャッシュ更新をサポート
- 🔍 **多様な検索**: ID、ファイル名、説明文による検索
- ▶️ **実行サポート**: 複数のインタープリタ（bash, python, ruby, node, php, perl）に対応
- 💬 **対話モード**: `read`コマンドなどを使用するスクリプトの対話的実行
- 📦 **uv対応**: PEP 723メタデータに対応した実行

本プロジェクトはlinuxとmacOSをサポートします。  
Windowsは将来対応予定です。

## 📋 前提条件

- Rust toolchain (1.75以降)
- GitHub CLI (`gh`) - 認証済み

## 🔧 [インストール](INSTALL.md)

### セットアップスクリプト（推奨）

```bash
# リポジトリをクローン
git clone https://github.com/7rikazhexde/gist-cache-rs.git
cd gist-cache-rs

# セットアップスクリプトを実行
./script/setup.sh install
```

対話的に以下を実行：
- ✅ 前提条件チェック
- 🔨 リリースビルド
- 📦 インストール方法選択
- 🔄 初回キャッシュ作成
- ⌨️ エイリアス設定（オプション）

### 手動インストール

```bash
# プロジェクトディレクトリで
cargo build --release
cargo install --path .
```

## 🚀 [クイックスタート](QUICKSTART.md)

実際の使用例については [EXAMPLES.md](EXAMPLES.md) にもまとめています。

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
gist-cache-rs run script perl
gist-cache-rs run script php
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

```bash
# 実行せずに内容を表示
gist-cache-rs run --preview backup

# 短縮形
gist-cache-rs run -p data-analysis
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
```

## 🗑️ アンインストール

```bash
# 自動アンインストール
./script/setup.sh uninstall

# 手動アンインストール
cargo uninstall gist-cache-rs
rm -rf ~/.cache/gist-cache/
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

詳細は [INSTALL.md](INSTALL.md) を参照してください。

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
│   └── setup.sh         # セットアップスクリプト
└── README.md
```

## 💾 キャッシュの保存場所

キャッシュファイルは以下の場所に保存されます：

```bash
~/.cache/gist-cache/cache.json
```

## 📄 ライセンス

MIT License
