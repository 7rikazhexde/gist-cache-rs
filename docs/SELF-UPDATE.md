# Self-Update機能の設計

## 概要

`gist-cache-rs self update` コマンドを追加し、ユーザーがアプリケーション自身を簡単に更新できるようにします。

## 目的

- セットアップスクリプトを再実行せずに、バイナリのみを更新する
- エイリアス設定やPATH設定などの環境設定を変更しない
- クロスプラットフォーム対応（Linux、macOS、Windows）

## 設計方針

### セットアップスクリプトとの違い

| 機能 | セットアップスクリプト | self update コマンド |
|------|----------------------|---------------------|
| 前提条件の確認 | ✓ | - |
| ビルド＆インストール | ✓ | ✓ |
| PATH設定 | ✓（オプション） | - |
| エイリアス設定 | ✓（オプション） | - |
| 初回キャッシュ作成 | ✓（オプション） | - |

**self updateは「バイナリの更新のみ」に特化**します。これにより：
- エイリアス設定を再実行しない
- PATH設定を変更しない
- キャッシュをクリアしない
- 既存の環境設定を保持

## 実装方針

### アプローチ1: GitHub Releasesからバイナリをダウンロード（推奨）

**メリット**:
- 高速（ビルド不要）
- ネットワーク経由で最新版を取得
- ユーザーのビルド環境に依存しない

**デメリット**:
- GitHub Releasesにプラットフォーム別バイナリをアップロードする必要がある
- リリースプロセスの追加作業

**実装方法**:
- `self_update` crate を使用（https://crates.io/crates/self_update）
- GitHub Releasesから対応するプラットフォームのバイナリをダウンロード
- 現在実行中のバイナリを置き換え

**セキュリティ考慮事項**:
- HTTPS通信（TLS検証）
- リリースタグの検証
- （将来的に）バイナリ署名の検証

### アプローチ2: ソースからビルド（フォールバック）

**メリット**:
- 最新のmain/masterブランチを取得可能
- リリースが存在しない場合でも更新可能
- 既存のツール（git、cargo）を活用

**デメリット**:
- ビルドに時間がかかる
- Rustツールチェーンが必要
- ディスク容量を消費

**実装方法**:
1. リポジトリのクローンまたはpull
2. `cargo install --path .` でビルド＆インストール

### 推奨実装：ハイブリッドアプローチ

1. **デフォルト**: GitHub Releasesからダウンロード
2. **フォールバック**: `--from-source` フラグでソースからビルド
3. **オプション**: `--check` で更新の有無のみ確認

## コマンド設計

### 基本コマンド

```bash
# 最新版に更新（GitHub Releasesから）
gist-cache-rs self update

# ソースからビルドして更新
gist-cache-rs self update --from-source

# 更新の有無を確認（実際には更新しない）
gist-cache-rs self update --check

# 強制更新（バージョンが同じでも更新）
gist-cache-rs self update --force

# 特定のバージョンに更新
gist-cache-rs self update --version 0.5.0
```

### オプション

| オプション | 説明 |
|-----------|------|
| `--from-source` | GitHub Releasesではなく、ソースからビルドして更新 |
| `--check` | 更新の有無のみ確認（実際には更新しない） |
| `--force` | バージョンが同じでも強制的に更新 |
| `--version <VERSION>` | 特定のバージョンに更新 |
| `--verbose` | 詳細な進捗情報を表示 |

## 処理フロー

### GitHub Releasesからの更新

```
1. 現在のバージョンを取得
2. GitHub API で最新リリースを確認
3. 新しいバージョンが存在するか確認
   - なければ終了
4. プラットフォームに対応するアセットを検索
   - Linux: gist-cache-rs-linux-x86_64.tar.gz
   - macOS: gist-cache-rs-macos-x86_64.tar.gz / gist-cache-rs-macos-aarch64.tar.gz
   - Windows: gist-cache-rs-windows-x86_64.zip
5. アセットをダウンロード
6. アーカイブを展開
7. 現在のバイナリを新しいバイナリで置き換え
8. パーミッション設定（Unix系のみ）
9. 完了メッセージを表示
```

### ソースからの更新

```
1. 現在のバージョンを取得
2. git コマンドの存在を確認
3. リポジトリの場所を確認
   a. 環境変数 GIST_CACHE_REPO でオーバーライド可能
   b. デフォルト: cargo metadata から取得
   c. フォールバック: git clone
4. git pull で最新版を取得
5. cargo install --path . でビルド＆インストール
6. 完了メッセージを表示
```

## プラットフォーム対応

### Linux

- バイナリ名: `gist-cache-rs`
- インストール先: `~/.cargo/bin/gist-cache-rs`
- アセット名: `gist-cache-rs-linux-x86_64.tar.gz`

### macOS

- バイナリ名: `gist-cache-rs`
- インストール先: `~/.cargo/bin/gist-cache-rs`
- アセット名:
  - Intel: `gist-cache-rs-macos-x86_64.tar.gz`
  - Apple Silicon: `gist-cache-rs-macos-aarch64.tar.gz`

### Windows

- バイナリ名: `gist-cache-rs.exe`
- インストール先: `%USERPROFILE%\.cargo\bin\gist-cache-rs.exe`
- アセット名: `gist-cache-rs-windows-x86_64.zip`

## 依存関係

### 新規追加

```toml
[dependencies]
self_update = "0.41"  # GitHub Releasesからの自動更新
```

### 既存の依存関係

- `anyhow` / `thiserror`: エラー処理
- `tokio`: 非同期処理
- `clap`: CLI引数パース

## エラー処理

### 想定されるエラー

1. **ネットワークエラー**
   - GitHub APIにアクセスできない
   - アセットをダウンロードできない
   - 対処: リトライまたはエラーメッセージ表示

2. **パーミッションエラー**
   - バイナリを置き換えられない
   - 対処: sudo権限が必要な旨を表示

3. **プラットフォーム未対応**
   - 対応するアセットが存在しない
   - 対処: `--from-source` の使用を提案

4. **バージョン取得エラー**
   - 現在のバージョンを取得できない
   - 対処: `--force` の使用を提案

5. **ビルドエラー**（`--from-source` 使用時）
   - Rustツールチェーンが存在しない
   - ビルドに失敗
   - 対処: セットアップスクリプトの実行を提案

## セキュリティ考慮事項

### 現時点

1. **HTTPS通信**: TLS/SSL検証を実施
2. **GitHub API認証**: GitHub APIのレート制限を考慮
3. **実行中バイナリの置き換え**: プラットフォーム固有の安全な方法を使用

### 将来的に検討

1. **バイナリ署名の検証**: GPG署名またはコード署名
2. **チェックサムの検証**: SHA256ハッシュの比較
3. **ロールバック機能**: 更新失敗時に前のバージョンに戻す

## テスト方針

### ユニットテスト

- バージョン比較ロジック
- プラットフォーム検出
- エラーハンドリング

### 統合テスト

- モックGitHub APIを使用したダウンロードテスト
- ファイル置き換えのテスト（一時ディレクトリで実施）

### E2Eテスト（手動）

- 実際のGitHub Releasesからの更新
- プラットフォーム別の動作確認

## リリースプロセスの変更

### GitHub Actions で自動ビルド＆リリース

新しいリリース時に以下を自動化：

1. プラットフォーム別のバイナリをビルド
   - Linux (x86_64)
   - macOS (x86_64, aarch64)
   - Windows (x86_64)

2. アーカイブを作成
   - Linux/macOS: `.tar.gz`
   - Windows: `.zip`

3. GitHub Releasesにアップロード

**参考**: 既存のRustプロジェクトで使用されているGitHub Actions例
- `rust-lang/cargo`
- `BurntSushi/ripgrep`

## マイルストーン

### Phase 1: 基本実装

- [x] `self update` サブコマンドの追加
- [x] `self_update` crateの統合
- [x] GitHub Releasesからのダウンロード機能
- [x] バイナリ置き換え機能
- [x] 基本的なエラーハンドリング

### Phase 2: ソースビルド対応

- [x] `--from-source` オプションの実装
- [x] git pull + cargo install の統合
- [x] リポジトリパスの検出ロジック
- [x] トラッキング情報がない場合のフォールバック（origin/main）

### Phase 3: 追加機能

- [x] `--check` オプション（更新確認のみ）
- [x] `--version` オプション（特定バージョンへの更新）
- [x] `--force` オプション（強制更新）
- [ ] プログレスバー表示（self_update crateが対応済み）

### Phase 4: CI/CD統合

- [ ] GitHub Actionsでのリリースビルド自動化
- [ ] プラットフォーム別バイナリの作成
- [ ] リリースノートの自動生成

### Phase 5: セキュリティ強化（将来）

- [ ] バイナリ署名の実装
- [ ] チェックサム検証
- [ ] ロールバック機能

## 参考資料

### 類似プロジェクトの実装

- **rustup**: Rust toolchainの自己更新
- **cargo-update**: cargoパッケージの更新
- **ripgrep**: GitHub Releasesからの自己更新

### 使用するcrate

- **self_update** (https://crates.io/crates/self_update)
  - GitHub Releasesからの自動更新をサポート
  - プラットフォーム検出
  - バイナリ置き換え

### ドキュメント

- GitHub Releases API: https://docs.github.com/en/rest/releases
- cargo install: https://doc.rust-lang.org/cargo/commands/cargo-install.html

## FAQ

### Q1: セットアップスクリプトとの使い分けは？

**A**:
- **セットアップスクリプト**: 初回インストール時に使用（PATH設定、エイリアス設定を含む）
- **self update**: インストール後のバージョンアップ時に使用（バイナリのみ更新）

### Q2: なぜセットアップスクリプトを内部で実行しないのか？

**A**: セットアップスクリプトはエイリアス設定やPATH設定を含むため、更新のたびに再実行すると不要な操作が発生します。`self update`はバイナリの更新のみに特化することで、シンプルかつ予測可能な動作を実現します。

### Q3: GitHub Releasesが存在しない場合は？

**A**: `--from-source` オプションを使用してソースからビルドできます。また、従来通り `cargo install --path .` や `cargo install --git` も使用可能です。

### Q4: 更新中にエラーが発生したら？

**A**: 現在のバイナリは保持されるため、再度実行できます。将来的にはロールバック機能を検討します。

### Q5: プラットフォーム固有の注意事項は？

**A**:
- **Windows**: 実行中のバイナリを置き換える場合、一時的に別名で保存してから置き換えます
- **Linux/macOS**: パーミッションを適切に設定（`chmod +x`）します

## まとめ

`gist-cache-rs self update` コマンドは、セットアップスクリプトとは異なり、**バイナリの更新のみ**に特化します。これにより：

1. ✓ エイリアス設定を再実行しない
2. ✓ PATH設定を変更しない
3. ✓ 既存の環境設定を保持
4. ✓ シンプルかつ予測可能な動作
5. ✓ クロスプラットフォーム対応

Phase 1では基本的なGitHub Releasesからの更新を実装し、その後段階的に機能を追加していきます。
