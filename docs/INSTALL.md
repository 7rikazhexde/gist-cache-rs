# インストールガイド

## 📋 前提条件

### 必須

- **Rust toolchain** (1.75以降)

  ```bash
  rustc --version  # 確認
  ```
  
  インストール方法:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **GitHub CLI** (`gh`) - 認証済み

  ```bash
  gh --version     # 確認
  gh auth status   # 認証状態確認
  ```
  
  認証方法:

  ```bash
  gh auth login
  ```

### 推奨

- Git (リポジトリクローン用)

## 🔧 インストール方法

### 方法1: セットアップスクリプト（推奨）

対話的にすべてのステップを実行します。

```bash
# リポジトリをクローン
git clone https://github.com/7rikazhexde/gist-cache-rs.git
cd gist-cache-rs

# セットアップスクリプトを実行
./script/setup.sh install
```

**実行される処理:**
1. ✅ 前提条件の確認
2. 📁 プロジェクトディレクトリの検出
3. 🔨 リリースビルド
4. 📦 インストール方法の選択
5. ⚙️ インストール実行
6. 🔄 初回キャッシュ作成
7. ⌨️ エイリアス設定（オプション）

### 方法2: cargo install

```bash
cargo build --release
cargo install --path .
```

**インストール先:** `~/.cargo/bin/gist-cache-rs`

**PATH設定:**
通常は自動設定済み。未設定の場合：

```bash
# ~/.bashrc または ~/.zshrc に追加
export PATH="$HOME/.cargo/bin:$PATH"
source ~/.bashrc
```

### 方法3: システムディレクトリ

```bash
cargo build --release
sudo cp target/release/gist-cache-rs /usr/local/bin/
```

**インストール先:** `/usr/local/bin/gist-cache-rs`  
**特徴:** 全ユーザーで共有、sudo権限が必要

### 方法4: ユーザーディレクトリ

```bash
cargo build --release
mkdir -p ~/bin
cp target/release/gist-cache-rs ~/bin/
```

**インストール先:** `~/bin/gist-cache-rs`

**PATH設定:**

```bash
# ~/.bashrc または ~/.zshrc に追加
export PATH="$HOME/bin:$PATH"
source ~/.bashrc
```

### 方法5: シンボリックリンク（開発者向け）

```bash
cargo build --release

# /usr/local/bin にリンク（要sudo）
sudo ln -sf "$(pwd)/target/release/gist-cache-rs" /usr/local/bin/gist-cache-rs

# または ~/bin にリンク
mkdir -p ~/bin
ln -sf "$(pwd)/target/release/gist-cache-rs" ~/bin/gist-cache-rs
```

**特徴:** ビルド後に自動反映、開発時に便利

## ⚙️ インストール後の設定

### 1. 初回キャッシュ作成

```bash
gist-cache-rs update
```

詳細表示:

```bash
gist-cache-rs update --verbose
```

### 2. エイリアス設定（オプション）

#### 自動設定（setup.sh使用時）

インストール時に対話的に設定：

```bash
推奨エイリアス名（gcrsu, gcrsr）を使用しますか？ [Y/n]: y
```

または

```bash
推奨エイリアス名（gcrsu, gcrsr）を使用しますか？ [Y/n]: n
gist-cache-rs update 用のエイリアス名: gcu
gist-cache-rs run 用のエイリアス名: gcr
```

#### 手動設定

```bash
# ~/.bashrc または ~/.zshrc に追加
alias gcrsu='gist-cache-rs update'
alias gcrsr='gist-cache-rs run'

# 反映
source ~/.bashrc
```

## ✅ インストール確認

```bash
# バージョン確認
gist-cache-rs --version

# ヘルプ表示
gist-cache-rs --help

# キャッシュ状態確認
gist-cache-rs update --verbose
```

## 🔍 トラブルシューティング

### command not found: gist-cache-rs

**原因:** PATHが設定されていない

**解決方法:**

```bash
# インストール場所を確認
which gist-cache-rs

# PATHを確認
echo $PATH

# ~/.cargo/bin の場合
export PATH="$HOME/.cargo/bin:$PATH"

# ~/bin の場合
export PATH="$HOME/bin:$PATH"

# 設定を反映
source ~/.bashrc
```

### 権限エラー

**原因:** 実行権限がない

**解決方法:**

```bash
# 実行権限を付与
chmod +x ~/.cargo/bin/gist-cache-rs
# または
chmod +x /usr/local/bin/gist-cache-rs
# または
chmod +x ~/bin/gist-cache-rs
```

### ビルドエラー

**原因:** Rustのバージョンが古い、依存関係の問題

**解決方法:**

```bash
# Rustを最新化
rustup update

# 依存関係を更新
cargo update

# クリーンビルド
cargo clean
cargo build --release
```

### GitHub CLI認証エラー

**エラー:** `GitHub CLI (gh) is not authenticated`

**解決方法:**

```bash
gh auth login
```

### キャッシュが作成されない

**エラー:** `Cache file not found`

**解決方法:**

```bash
# 初回キャッシュ作成
gist-cache-rs update

# 詳細情報を表示
gist-cache-rs update --verbose
```

### レートリミットエラー

**警告:** `レートリミット残量が低いです`

**解決方法:**
- しばらく待ってから再試行
- `--force` オプションを避ける
- 差分更新を使用

## 🗑️ アンインストール

### 自動アンインストール

```bash
./script/setup.sh uninstall
```

対話的に以下を選択：
- バイナリ削除
- キャッシュディレクトリ削除
- エイリアス削除

### 手動アンインストール

```bash
# cargo でインストールした場合
cargo uninstall gist-cache-rs

# システムディレクトリにインストールした場合
sudo rm /usr/local/bin/gist-cache-rs

# ユーザーディレクトリにインストールした場合
rm ~/bin/gist-cache-rs

# キャッシュディレクトリを削除
rm -rf ~/.cache/gist-cache/

# エイリアスを削除（~/.bashrc または ~/.zshrc から該当行を削除）
# 例:
# alias gcrsu='gist-cache-rs update'
# alias gcrsr='gist-cache-rs run'
```

## ➡️ 次のステップ

- [QUICKSTART.md](QUICKSTART.md) - クイックスタートガイド
- [EXAMPLES.md](EXAMPLES.md) - 実用例
- [README.md](../README.md) - プロジェクト概要
