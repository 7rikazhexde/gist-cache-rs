# 📥 インストールガイド

gist-cache-rsのインストール方法を詳しく説明します。

## 📋 前提条件

### 1. 🦀 Rust Toolchain

```bash
# Rustupを使用したインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# インストール後、シェルを再起動するか以下を実行
source $HOME/.cargo/env

# バージョン確認
rustc --version
cargo --version
```

**期待される出力:**

```text
rustc 1.75.0 (またはそれ以降)
cargo 1.75.0 (またはそれ以降)
```

### 2. 🐙 GitHub CLI (gh)

#### Ubuntu/Debian

```bash
curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
sudo apt update
sudo apt install gh
```

#### macOS (Homebrew)

```bash
brew install gh
```

#### 🔐 認証

```bash
# GitHub CLIで認証
gh auth login

# 認証状態の確認
gh auth status
```

## 🚀 インストール方法

### 方法1: ⚙️ cargoを使用したインストール（推奨）

```bash
# プロジェクトディレクトリに移動
cd ~/dev/rust/gist-cache-rs

# インストール（~/.cargo/binにバイナリが配置されます）
cargo install --path .

# インストール確認
which gist-cache-rs
gist-cache-rs --version
```

**~/.cargo/bin がPATHに含まれていることを確認:**

```bash
echo $PATH | grep ".cargo/bin"

# 含まれていない場合は追加
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 方法2: 📦 手動ビルドとコピー

```bash
# リリースビルド
cargo build --release

# システムにコピー
sudo cp target/release/gist-cache-rs /usr/local/bin/

# または、ユーザーローカルにコピー
mkdir -p ~/bin
cp target/release/gist-cache-rs ~/bin/

# ~/binがPATHに含まれていることを確認
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 方法3: 🔗 シンボリックリンク（開発者向け）

```bash
# リリースビルド
cargo build --release

# シンボリックリンク作成
sudo ln -s $(pwd)/target/release/gist-cache-rs /usr/local/bin/gist-cache-rs

# または
mkdir -p ~/bin
ln -s $(pwd)/target/release/gist-cache-rs ~/bin/gist-cache-rs
```

## ⚙️ 初期設定

### 1. 🔐 GitHub認証確認

```bash
# 認証状態の確認
gh auth status

# 未認証の場合
gh auth login
# ブラウザまたはトークンで認証を完了
```

### 2. 💾 キャッシュディレクトリの確認

キャッシュは以下のディレクトリに保存されます（初回更新時に自動作成）：

```bash
~/.cache/gist-cache/
```

### 3. 🔄 初回キャッシュ更新

```bash
# キャッシュを作成（詳細表示付き）
gist-cache-rs update --verbose
```

**成功すると以下のように表示されます:**

```bash
Gistキャッシュを更新しています...
モード: 強制全件更新
レートリミット残量: 4999
GitHubユーザー: your-username
GitHub APIからGist情報を取得中...
取得したGist数: 42
新規/更新: 42件
キャッシュ更新が完了しました
総Gist数: 42
```

## ⚡ エイリアス設定（オプション）

より便利に使用するため、エイリアスを設定できます。

### Bash

`~/.bashrc` に追加:

```bash
# Gist Cache エイリアス
alias gcurs='gist-cache-rs update'
alias grcrs='gist-cache-rs run'
```

反映:

```bash
source ~/.bashrc
```

### Zsh

`~/.zshrc` に追加:

```bash
# Gist Cache エイリアス
alias gcurs='gist-cache-rs update'
alias grcrs='gist-cache-rs run'
```

反映:

```bash
source ~/.zshrc
```

### Fish

`~/.config/fish/config.fish` に追加:

```fish
# Gist Cache エイリアス
alias gcurs='gist-cache-rs update'
alias grcrs='gist-cache-rs run'
```

反映:

```fish
source ~/.config/fish/config.fish
```

## ✅ 動作確認

### 1. 🔍 コマンドの確認

```bash
# バージョン表示
gist-cache-rs --version

# ヘルプ表示
gist-cache-rs --help
gist-cache-rs update --help
gist-cache-rs run --help
```

### 2. 🔄 キャッシュ更新テスト

```bash
# 詳細モードで更新
gist-cache-rs update -v
```

### 3. 🔎 検索テスト

```bash
# プレビューモードで検索（実行はしない）
gist-cache-rs run --preview "keyword"
```

## 🗑️ アンインストール

### cargoでインストールした場合

```bash
cargo uninstall gist-cache-rs
```

### 手動でコピーした場合

```bash
# システムインストールの場合
sudo rm /usr/local/bin/gist-cache-rs

# ユーザーローカルの場合
rm ~/bin/gist-cache-rs
```

### キャッシュディレクトリの削除

```bash
rm -rf ~/.cache/gist-cache
```

## 🔧 トラブルシューティング

### ❌ ビルドエラー

```bash
# 依存関係の問題
cargo clean
cargo build --release

# Rustのバージョンアップ
rustup update
```

### 🚫 gh コマンドが見つからない

```bash
# GitHub CLIのインストール確認
which gh

# インストールされていない場合は「前提条件」セクションを参照
```

### 🛣️ パスが通らない

```bash
# 現在のPATH確認
echo $PATH

# .bashrc/.zshrcを確認
cat ~/.bashrc | grep PATH

# 手動でPATHに追加
export PATH="$HOME/.cargo/bin:$PATH"
```

### 🔒 権限エラー

```bash
# バイナリに実行権限を付与
chmod +x target/release/gist-cache-rs

# またはインストール先に適切な権限を設定
sudo chmod +x /usr/local/bin/gist-cache-rs
```

### 🌐 ネットワークエラー（ビルド時）

依存関係のダウンロードでネットワークエラーが発生する場合：

```bash
# プロキシ設定が必要な場合
export https_proxy=your-proxy-url

# または .cargo/config.toml に設定
```

## 🎯 次のステップ

インストールが完了したら、[QUICKSTART.md](QUICKSTART.md) を参照して、実際の使い方を学んでください。
