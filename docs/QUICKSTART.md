# ⚡ クイックスタートガイド

5分でgist-cache-rsを始めるための最小限のガイドです。

## ステップ1: ✅ 前提条件の確認

```bash
# Rustがインストールされているか確認
rustc --version

# GitHub CLIがインストールされているか確認
gh --version

# GitHub CLIで認証されているか確認
gh auth status
```

未インストールの場合は[INSTALL.md](INSTALL.md)を参照してください。

## ステップ2: 📥 インストール

```bash
# リポジトリをクローン
git clone https://github.com/7rikazhexde/gist-cache-rs.git
cd gist-cache-rs

# ビルドとインストール
cargo build --release
cargo install --path .

# インストール確認
gist-cache-rs --version
```

その他のインストール方法は [INSTALL.md](INSTALL.md) を参照してください。

## ステップ3: 🔄 初回キャッシュ作成

```bash
# キャッシュを作成
gist-cache-rs update

# 詳細表示付き（推奨）
gist-cache-rs update --verbose
```

**出力例:**

```bash
Gistキャッシュを更新しています...
モード: 強制全件更新
レートリミット残量: 4999
GitHub APIからGist情報を取得中...
取得したGist数: 42
新規/更新: 42件
キャッシュ更新が完了しました
総Gist数: 42
```

## ステップ4: 🚀 Gistを検索して実行

### 👁️ プレビュー（実行せず内容確認）

```bash
# キーワードで検索してプレビュー
gist-cache-rs run --preview backup
```

### ▶️ 実際に実行

```bash
# Bashスクリプトを実行
gist-cache-rs run backup bash

# Python スクリプトを実行
gist-cache-rs run data_analysis.py python3

# uvでPythonスクリプトを実行
gist-cache-rs run ml-script uv
```

### 📝 引数を渡して実行

```bash
# スクリプトに引数を渡す
gist-cache-rs run backup bash /src /dst

# Pythonスクリプトに引数を渡す
gist-cache-rs run data_analysis.py python3 input.csv --output result.json
```

## ステップ5: ⚡ エイリアス設定（オプション）

より便利に使用するため、エイリアスを設定します：

```bash
# ~/.bashrc に追加
echo 'alias gcrsu="gist-cache-rs update"' >> ~/.bashrc
echo 'alias gcrsr="gist-cache-rs run"' >> ~/.bashrc
source ~/.bashrc

# これで短縮形で使用可能
gcrsu                # キャッシュ更新
gcrsr backup bash    # Gist実行
```

## 📚 よく使うコマンド

### 🔄 キャッシュ管理

```bash
# 差分更新（通常）
gist-cache-rs update

# 強制全件更新
gist-cache-rs update --force

# 詳細表示
gist-cache-rs update --verbose
```

### 🗂️ コンテンツキャッシュ管理

```bash
# キャッシュ一覧表示
gist-cache-rs cache list

# キャッシュサイズ確認
gist-cache-rs cache size

# 全キャッシュ削除
gist-cache-rs cache clear
```

### 🔍 Gist検索・実行

```bash
# 基本的な検索と実行
gist-cache-rs run keyword

# プレビュー（実行せず内容確認）
gist-cache-rs run -p keyword

# 対話モード（readコマンドなど使用時）
gist-cache-rs run -i interactive-script

# ダウンロードフォルダに保存
gist-cache-rs run --download backup bash

# プレビュー後にダウンロード
gist-cache-rs run -p --download script

# ファイル名で検索
gist-cache-rs run --filename setup.sh

# 説明文で検索
gist-cache-rs run --description deployment

# 実行前に最新情報を取得（強制更新）
gist-cache-rs run --force backup bash
```

### 🔧 インタープリタ指定

引数指定などはスクリプトに依存します。

```bash
# Bash（デフォルト）
gist-cache-rs run script bash arg1 arg2 ...

# Python3
gist-cache-rs run script python3 arg1 arg2 ...

# Ruby
gist-cache-rs run script ruby arg1 arg2 ...

# Node.js
gist-cache-rs run script node arg1 arg2 ...

# uv（PEP 723対応）
gist-cache-rs run script uv arg1 arg2 ...
```

## 💼 実用例

[EXAMPLES.md](EXAMPLES.md)を確認してください。

## 🔧 トラブルシューティング

### ❌ キャッシュが見つからない

```bash
# エラー: Cache file not found
→ gist-cache-rs update を実行
```

### 🔐 GitHub認証エラー

```bash
# エラー: GitHub CLI is not authenticated
→ gh auth login を実行
```

### 🚫 コマンドが見つからない

```bash
# コマンドが見つからない場合
→ which gist-cache-rs でパスを確認
→ ~/.cargo/bin または /usr/local/bin がPATHに含まれているか確認
```

### 🔎 検索結果が見つからない

```bash
# キャッシュが古い可能性
→ gist-cache-rs update で最新化
```

## 🎯 関連情報

- [README.md](../README.md) - 詳細な機能説明
- [INSTALL.md](INSTALL.md) - インストール詳細
- [EXAMPLES.md](EXAMPLES.md) - 実例集（実際の使用例）

## ❓ ヘルプ

```bash
# 全体のヘルプ
gist-cache-rs --help

# サブコマンドのヘルプ
gist-cache-rs update --help
gist-cache-rs run --help

# 引数なしで実行してもヘルプが表示される
gist-cache-rs run
```
