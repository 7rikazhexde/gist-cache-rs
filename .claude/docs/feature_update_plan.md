# gist-cache-rs 機能更新計画

> バージョン: 0.7.0 (リリース済み)
> 日付: 2025-12-11
> ステータス: フェーズ1進行中

## エグゼクティブサマリー

本ドキュメントは、包括的なコードベース分析に基づいたgist-cache-rsの機能追加と改善提案をまとめたものです。本プロジェクトは優れたアーキテクチャを持ち（テストカバレッジ68.95%、163テスト）、ユーザー体験、機能性、開発者ツールにおいて改善の余地があります。

**設計方針**: 軽量でシンプルな構造を維持し、実用性を重視します。過剰な品質追求や複雑な機能を避け、ユーザーに真に価値をもたらす基本的な機能に焦点を当てます。

---

## 進捗サマリー (2025-12-13更新)

### 完了済み

#### v0.7.0 (リリース済み)

**1.1 Cache Clean コマンド** ✅

- `--older-than <DAYS>`: 指定日数より古いエントリを削除
- `--orphaned`: 孤立したコンテンツキャッシュを削除
- `--dry-run`: 削除プレビューモード
- 実装: `src/cache/content.rs` (460行追加)、10個のテスト追加
- 全154テスト成功、機能デグレードなし

**リリース情報**:

- Issue: [#24](https://github.com/7rikazhexde/gist-cache-rs/issues/24)
- PR: [#25](https://github.com/7rikazhexde/gist-cache-rs/pull/25)
- Tag: [v0.7.0](https://github.com/7rikazhexde/gist-cache-rs/releases/tag/v0.7.0)

#### v0.8.0 (リリース済み)

**1.3 シェル補完スクリプト** ✅

- Bash, Zsh, Fish, PowerShell の4シェルに対応
- `gist-cache-rs completions <SHELL>` コマンドで補完スクリプト生成
- Tab キーでコマンド、サブコマンド、オプションの自動補完
- 実装: `src/cli.rs` (103行追加)、13個のテスト追加
  - ユニットテスト: 5個（各シェルの補完生成）
  - 統合テスト: 8個（CLI補完動作確認）
- Windows/WSL2両環境で全テスト通過（138 unit + 23 integration）
- 機能デグレードなし、Windows対応のバグ修正含む
- ドキュメント更新: README.md、book/src/user-guide/shell-completions.md
  - 設定ファイルの場所と内容の例を追加
  - 実際の使用例（WSL2/PowerShell）を追加

**6.1 未使用依存関係の削除** ✅

- `reqwest` crateを削除（直接API実装の名残）
- `src/error.rs`から`GistCacheError::Request`バリアントを削除
- バイナリサイズ削減（約500KB推定）
- 全138テスト成功

**リリース情報**:

- Issue: [#26](https://github.com/7rikazhexde/gist-cache-rs/issues/26)
- PR: [#27](https://github.com/7rikazhexde/gist-cache-rs/pull/27)
- Tag: [v0.8.0](https://github.com/7rikazhexde/gist-cache-rs/releases/tag/v0.8.0)

#### v0.8.1 (リリース済み)

**2.2 プログレス表示** ✅

- スピナー表示: GitHub API からのGist情報取得時
- プログレスバー: 10件以上のGist処理時に進捗を表示
- `--verbose` フラグとの統合: verboseモード時は詳細ログ、通常モードはプログレス表示
- 実装: `src/cache/update.rs` にindicatifを統合、87行追加
- 依存関係: `indicatif = "0.18"` を追加
- テスト: 2個の統合テスト追加
- 全163テスト成功（138 unit + 25 integration）、機能デグレードなし
- ドキュメント更新: README.md、book/src/user-guide/quickstart.md
  - プログレス表示をFeaturesに追加
  - 通常モードとverboseモードの出力例を追加

**リリース情報**:

- Issue: [#28](https://github.com/7rikazhexde/gist-cache-rs/issues/28)
- PR: [#29](https://github.com/7rikazhexde/gist-cache-rs/pull/29)
- Tag: [v0.8.1](https://github.com/7rikazhexde/gist-cache-rs/releases/tag/v0.8.1)

#### v0.8.2 (実装完了、リリース準備中)

**2.3 対話的選択の改善** ✅

- dialoguer crateを使用した矢印キー選択
- ColorfulThemeによる視覚的な改善
- Escキーでのキャンセルサポート
- 表示形式の改善: "description - files"
- 実装: `src/search/query.rs` のselect_from_resultsを書き換え
- 依存関係: `dialoguer = "0.12"` を追加
- 全163テスト成功（138 unit + 25 integration）、機能デグレードなし
- ドキュメント更新: README.md
  - Interactive SelectionをFeaturesに追加

**リリース情報**:

- ブランチ: `feature/interactive-selection-improvement`
- コミット: `c8b1a2c`
- PR: 作成予定
- Issue: 作成予定

### 次の優先タスク

#### オプション1: 2.1 検索機能の強化

- 推定工数: 中（4-5日）
- 依存関係: `regex` crate
- 効果: 検索精度の大幅向上、大規模管理が可能
- 技術的リスク: 低

#### オプション2: 2.4 出力フォーマットオプション

- 推定工数: 中（3-4日）
- 依存関係: なし
- 効果: スクリプト連携、自動化が容易
- 技術的リスク: 低

---

## 目次

1. [優先度1：高（即座に実装すべき）](#優先度1高即座に実装すべき)
2. [優先度2：中（ユーザー体験）](#優先度2中ユーザー体験)
3. [優先度3：中（機能拡張）](#優先度3中機能拡張)
4. [優先度4：低（高度な機能）](#優先度4低高度な機能)
5. [セキュリティと信頼性](#セキュリティと信頼性)
6. [技術的負債](#技術的負債)
7. [テストと開発](#テストと開発)
8. [実装ロードマップ](#実装ロードマップ)
9. [依存関係と影響分析](#依存関係と影響分析)

---

## 優先度1：高（即座に実装すべき）

### 1.1 `cache clean`コマンドの実装 ✅ 完了 (v0.7.0)

**ステータス**: ✅ 実装完了、v0.7.0でリリース済み

**現状 (Before)**:

```bash
$ gist-cache-rs cache clean
Error: The 'cache clean' subcommand is not yet implemented.
```

キャッシュが肥大化しても削除手段がなく、手動でキャッシュディレクトリを探して削除する必要があります。

- 古いキャッシュが蓄積してディスク容量を圧迫
- どのキャッシュが古いのか判断が困難
- 孤立したファイルの存在に気づけない

**提案 (After)**:

```bash
# 30日以上古いキャッシュエントリを削除
$ gist-cache-rs cache clean --older-than 30 --dry-run
以下のキャッシュエントリを削除します:
  - abc123def456 (60日前に更新) [メタデータ + コンテンツ]
  - def456ghi789 (45日前に更新) [メタデータのみ]
削除されるファイル: 2件、合計サイズ: 1.2 MB

$ gist-cache-rs cache clean --older-than 30
✓ 2件のキャッシュエントリを削除しました (1.2 MB削減)

# 孤立したコンテンツキャッシュを削除
$ gist-cache-rs cache clean --orphaned
✓ 5件の孤立ファイルを削除しました (500 KB削減)
```

**改善 (Improvement)**:

- **ディスク容量の回復**: 長期利用で蓄積したキャッシュを安全に削除可能
- **プレビュー機能**: `--dry-run`で削除前に確認できるため安心
- **キャッシュの健全性維持**: 孤立ファイルを自動検出・削除
- **トラブルシューティング**: キャッシュ関連の問題を簡単に解決可能

**目的**:

- キャッシュの肥大化を防ぎ、ディスク容量を節約
- 古いまたは不要なキャッシュエントリを手動で管理可能にする
- 開発者やパワーユーザーにキャッシュメンテナンスツールを提供

**効果**:

- ディスク使用量の削減（特に長期利用ユーザー）
- キャッシュの健全性維持
- トラブルシューティング時の有用なツール
- 未実装機能の完成度向上

**提案API**:

```bash
# 指定日数より古いキャッシュエントリを削除
gist-cache-rs cache clean --older-than <DAYS>

# 孤立したコンテンツキャッシュファイルを削除（メタデータに対応するものがない）
gist-cache-rs cache clean --orphaned

# 削除せずにプレビュー
gist-cache-rs cache clean --dry-run

# オプションの組み合わせ
gist-cache-rs cache clean --older-than 30 --orphaned --dry-run
```

**実装詳細**:

- モジュール: `src/cache/update.rs` (`CacheUpdater::clean()`メソッドを追加)
- ロジック:
  - メタデータキャッシュのタイムスタンプを閾値と比較
  - コンテンツキャッシュディレクトリをスキャンして孤立ファイルを検出
  - エラー回復を伴うアトミックな削除
- テスト: 削除ロジックの機能テスト、モックキャッシュを使った統合テスト

**推定工数**: 小（1-2日）

**依存関係**: なし

---

### 1.2 設定ファイルのサポート

**ステータス**: 将来検討（導入バージョン検討中）

**現状 (Before)**:

```bash
# Python3をデフォルトで使いたいが、毎回指定が必要
$ gist-cache-rs run my-script python3
$ gist-cache-rs run another-script python3
$ gist-cache-rs run yet-another python3

# 実行前確認を無効化したいが、オプションが存在しない
# キャッシュの保持期間もカスタマイズ不可
```

すべての動作がハードコードされており、ユーザーの好みに合わせてカスタマイズできません：

- 毎回同じオプションを指定する必要がある
- デフォルトインタプリタを変更できない
- 確認プロンプトなどの動作を変更できない

**提案 (After)**:

`~/.config/gist-cache/config.toml`を作成:

```toml
[defaults]
interpreter = "python3"           # デフォルトインタプリタ設定

[execution]
confirm_before_run = false        # 確認プロンプトをスキップ

[cache]
cache_retention_days = 30         # 30日で自動削除

[display]
colors = true                     # カラー出力有効
```

実行時:

```bash
# インタプリタ指定不要！設定ファイルから自動的にpython3が使われる
$ gist-cache-rs run my-script
✓ Python3でスクリプトを実行しました

# 設定を確認
$ gist-cache-rs config show
デフォルトインタプリタ: python3
実行前確認: 無効
キャッシュ保持期間: 30日

# CLIオプションで上書き可能
$ gist-cache-rs run my-script bash  # 今回だけbashを使用
```

**改善 (Improvement)**:

- **タイプ数の大幅削減**: デフォルト設定により、よく使うオプションを毎回入力不要
- **ワークフローのカスタマイズ**: 自分の使い方に合わせて動作を調整可能
- **チーム標準化**: 設定ファイルを共有してチーム全体で統一設定を使用
- **柔軟性**: CLIオプションで一時的に上書き可能

**目的**:

- ユーザーごとの好みに合わせたカスタマイズを可能に
- 繰り返しコマンドラインオプションを指定する手間を削減
- チーム間での設定共有を容易に

**効果**:

- ユーザビリティの大幅向上
- デフォルトインタプリタの設定でタイプ数削減
- 実行前確認などの安全機能をカスタマイズ可能
- 企業やチームでの標準化が容易

**提案パス**: `~/.config/gist-cache/config.toml` (Linux/macOS) または `%APPDATA%\gist-cache\config.toml` (Windows)

**設定スキーマ**:

```toml
[defaults]
# 指定されていない場合のデフォルトインタプリタ
interpreter = "python3"

# gist実行前にキャッシュを自動更新
auto_update = false

# キャッシュ保持期間（日数、自動クリーンアップ用）
cache_retention_days = 30

[search]
# 大文字小文字を区別する検索
case_sensitive = false

# デフォルト検索モード (auto, id, filename, description, both)
mode = "auto"

[execution]
# スクリプト実行前に確認
confirm_before_run = true

# デフォルトで対話モード
interactive = false

# 実行したスクリプトを自動ダウンロード
auto_download = false

[cache]
# カスタムキャッシュディレクトリ（デフォルトを上書き）
# directory = "/custom/path/to/cache"

# キャッシュ圧縮を有効化（将来機能）
compress = false

[display]
# 出力に色を使用
colors = true

# デフォルトで詳細出力
verbose = false
```

**実装詳細**:

- 依存関係を追加: `serde`（既に含まれている）、`toml` crate
- 新規モジュール: `src/config.rs`の拡張
  - `Config::from_file()`メソッドを追加
  - 設定ファイルとCLI引数をマージ（CLIが優先）
  - 設定値の検証
- すべてのコマンドを設定に準拠するよう更新
- `gist-cache-rs config show`コマンドで現在の設定を表示
- `gist-cache-rs config edit`コマンドでエディタで設定を開く

**推定工数**: 中（3-5日）

**依存関係**: なし

---

### 1.3 シェル補完スクリプト

**ステータス**: ✅ 実装完了 (v0.8.0)

**現状 (Before)**:

```bash
# すべて手動で入力が必要
$ gist-cache-rs ru  # Tabを押しても何も起こらない
$ gist-cache-rs run --int  # Tabを押しても補完されない

# オプションを忘れた場合、--helpで確認が必要
$ gist-cache-rs run --help
... (長いヘルプメッセージを読む必要がある)

# タイプミスが発生しやすい
$ gist-cache-rs rnu my-script  # "run"のタイプミス
Error: 'rnu' is not a valid subcommand
```

- Tab補完が効かずすべて手入力
- コマンドやオプションを覚える必要がある
- タイプミスによるエラーが頻発

**提案 (After)**:

補完スクリプトをインストール後:

```bash
# サブコマンドの補完
$ gist-cache-rs ru[Tab]
$ gist-cache-rs run ✓  # 自動的に "run" に補完される

# オプションの候補表示
$ gist-cache-rs run --[Tab][Tab]
--interactive  --preview  --force  --download  --id  --filename

# サブコマンドの候補表示
$ gist-cache-rs [Tab][Tab]
update   run   cache   create   edit   completions   help

# 長いオプションも素早く入力
$ gist-cache-rs run --int[Tab]
$ gist-cache-rs run --interactive ✓
```

インストール:

```bash
# Bash用の補完スクリプトを生成してインストール
$ gist-cache-rs completions bash > ~/.local/share/bash-completion/completions/gist-cache
# シェル再起動後、Tab補完が有効に
```

**改善 (Improvement)**:

- **入力効率の劇的向上**: Tabキーで瞬時に補完、タイプ数が1/3以下に
- **学習コストの削減**: オプションを覚えなくてもTab連打で候補が見える
- **タイプミスの防止**: 補完により正確なコマンド入力が保証される
- **プロフェッショナルな体験**: 他の一流CLIツール（git、cargo、kubectlなど）と同等の使い勝手

**目的**:

- コマンドライン操作の効率化
- タイプミスの削減
- コマンドやオプションの発見性向上

**効果**:

- Tabキーでコマンド・オプション・引数を自動補完
- 長いコマンドを素早く入力可能
- 利用可能なオプションをその場で確認可能
- 他のCLIツールと同等の使い勝手を提供

**シェル補完とは**:

ターミナルで`gist-cache`と入力後、Tabキーを押すと、利用可能なサブコマンドやオプションが自動的に補完・提案される機能です。

**使用例**:

```bash
# 通常の入力
$ gist-cache-rs ru[Tab]  # 自動的に "run" に補完される

# オプション候補の表示
$ gist-cache-rs run --[Tab][Tab]
--interactive  --preview  --force  --download  --id  --filename

# ファイル名補完
$ gist-cache-rs create scr[Tab]  # "script.py" に補完される
```

**サポートシェル**: bash, zsh, fish, PowerShell

**補完スクリプトの生成**:

この機能を実現するため、`completions`という新しいサブコマンドを追加します。

```bash
gist-cache-rs completions <SHELL>
```

このコマンドは、指定したシェル（bash、zsh、fish、powershell）用の補完スクリプトを生成し、標準出力に出力します。ユーザーはこの出力を適切な場所に保存することで、シェル補完を有効化できます。

**実装詳細**:

- `clap_complete` crate を使用（clap v4に組み込み）
- サブコマンド: `gist-cache-rs completions <SHELL>`
  - 引数: `bash` | `zsh` | `fish` | `powershell`
  - 出力: 指定シェル用の補完スクリプト（標準出力）
- READMEにインストール手順を追加

**インストール方法**:

```bash
# Bash（システム全体）
gist-cache-rs completions bash | sudo tee /etc/bash_completion.d/gist-cache

# Bash（ユーザー専用）
mkdir -p ~/.local/share/bash-completion/completions
gist-cache-rs completions bash > ~/.local/share/bash-completion/completions/gist-cache

# Zsh
gist-cache-rs completions zsh > ~/.zsh/completion/_gist-cache
# .zshrcに以下を追加: fpath=(~/.zsh/completion $fpath)

# Fish
gist-cache-rs completions fish > ~/.config/fish/completions/gist-cache.fish

# PowerShell（プロファイルに追加）
gist-cache-rs completions powershell >> $PROFILE
```

**推定工数**: 小（1日）

**依存関係**: Cargo.tomlに`clap_complete`を追加

---

## 優先度2：中（ユーザー体験）

### 2.1 検索機能の強化

**ステータス**: 現在の検索は機能的だが限定的

**現状 (Before)**:

```bash
# 基本的な部分一致検索のみ
$ gist-cache-rs run backup
見つかったgist:
  - daily_backup_script.sh
  - backup_database.py
  - old_backup_2023.sh  # これは不要なのに表示される

# 特定言語のスクリプトだけを検索できない
$ gist-cache-rs cache list
200件のgist一覧が表示される（Python, Bash, Ruby, Node.js混在）
目的のPythonスクリプトを探すのに時間がかかる

# パターンマッチングができない
# 「test_で始まる.pyファイル」のような検索ができない
```

- シンプルな文字列検索のみで、複雑な検索ができない
- 言語フィルタがないため、大量のgistから探すのが困難
- ファイル名のパターンで絞り込めない

**提案 (After)**:

```bash
# 正規表現で高度な検索
$ gist-cache-rs run --regex "^test_.*\.py$"
見つかったgist:
  - test_backup.py
  - test_database.py
  - test_api.py
# "test_"で始まり".py"で終わるファイルのみを正確に抽出

# 言語でフィルタリング
$ gist-cache-rs cache list --language python
Pythonスクリプトのみ表示（全200件中50件）:
  - backup_database.py
  - data_analysis.py
  - api_client.py

# 複数の条件を組み合わせ
$ gist-cache-rs run --language bash --regex "backup"
Bash言語で"backup"を含むgistのみ:
  - daily_backup.sh
  - backup_database.sh

# タグベース検索（GitHub gistのトピック機能を活用）
$ gist-cache-rs run --tag automation --tag devops
automationとdevopsタグが付いたgistのみを表示
```

**改善 (Improvement)**:

- **検索精度の大幅向上**: 正規表現により、欲しいgistをピンポイントで検索可能
- **検索時間の短縮**: 言語フィルタで候補を1/4に削減、目視確認が楽に
- **大規模管理が可能**: 100+のgistでも、タグと言語で瞬時に絞り込める
- **パワーユーザー向け**: 複雑な検索条件を組み合わせて効率的に管理

**目的**:

- より正確で柔軟なgist検索を実現
- 大量のgistを持つユーザーの検索効率向上
- 言語やタグによる絞り込みで目的のgistを素早く発見

**効果**:

- 検索時間の短縮
- より直感的なgist発見
- プロジェクト別・用途別の管理が容易に
- 生産性の向上

**提案機能**:

#### 2.1.1 正規表現検索

```bash
gist-cache-rs run --regex "test_.*\.py"
gist-cache-rs run --regex "^backup.*\.sh$"
```

**目的**: パターンマッチングによる高度な検索
**効果**: 複雑な命名規則に対応、パワーユーザーの効率向上

#### 2.1.2 言語/ファイルタイプフィルタ

```bash
gist-cache-rs run query --language python
gist-cache-rs run query --extension .rs
gist-cache-rs cache list --language bash
```

**目的**: 特定言語のgistのみを表示
**効果**: 多言語環境でのgist管理が容易、検索結果の絞り込み

#### 2.1.3 タグベース検索

```bash
gist-cache-rs run --tag automation --tag backup
gist-cache-rs cache list --tag devops
```

**目的**: gistのトピック/タグで分類・検索
**効果**: プロジェクトや用途別の整理が可能、カテゴリ別検索

**実装詳細**:

- モジュール: `src/search/query.rs`
- 新しいフィールドを持つ`SearchOptions`構造体を追加
- `SearchQuery::search()`を新しいモードをサポートするよう更新
- GitHub APIからタグ情報を取得してメタデータキャッシュに保存
- 後方互換性を維持

**推定工数**: 中（4-5日）

**依存関係**: `regex` crateを追加

---

### 2.2 プログレス表示

**ステータス**: ✅ 実装完了 (v0.8.1)

**現状 (Before)**:

```bash
$ gist-cache-rs update
(何も表示されない... 30秒経過... まだ何も表示されない...)
(フリーズしたのか?Ctrl+Cで中断すべきか?悩む...)
(60秒後)
Cache updated successfully.
```

- 長時間操作中に何も表示されず不安
- 進捗状況が全く分からない
- フリーズしたのか処理中なのか判断できない
- 特に150+のgist更新時は数分かかるが、何が起きているか不明

**提案 (After)**:

```bash
$ gist-cache-rs update
キャッシュを更新中...
[████████████████░░░░] 80% (120/150 gists) ETA: 10s

新しいgist: 5件
更新されたgist: 15件
変更なし: 100件

✓ キャッシュ更新完了 (1分23秒)
```

小規模な操作では:

```bash
$ gist-cache-rs run my-script
gistをフェッチ中... ⠋
✓ スクリプトを実行中
```

**改善 (Improvement)**:

- **不安の解消**: プログレスバーで処理が進行中であることを視覚的に確認
- **時間の見積もり**: ETAで残り時間が分かり、コーヒーを取りに行くか判断可能
- **透明性の向上**: 何件処理済みか、全体の何%完了かが一目瞭然
- **体感速度の向上**: 進捗が見えると同じ待ち時間でもストレスが少ない

**目的**:

- 長時間操作の視覚的フィードバック提供
- ユーザーの不安解消（フリーズしていないことを確認）
- 残り時間の目安を提供

**効果**:

- ユーザー体験の向上
- 操作の透明性向上
- 待ち時間の体感短縮
- 大量gist更新時の進捗把握

**提案機能**:

- キャッシュ更新のプログレスバー（特に多数のgistの場合）
- ネットワーク操作のスピナー
- 大規模操作のETA（予想残り時間）

**出力例**:

```bash
キャッシュを更新中...
[████████████████░░░░] 80% (120/150 gists) ETA: 10s

gistをフェッチ中... ⠋ (3/150)
```

**実装詳細** (v0.8.1で完了):

- 依存関係追加: `indicatif = "0.17"`
- モジュール: `src/cache/update.rs`
- スピナー: GitHub API からGist情報をフェッチ中に表示
- プログレスバー: 10件以上のGist処理時に進捗を表示（`[████████████████░░░░] 42/42 (100%)`）
- `--verbose`フラグとの統合: verboseモード時は詳細ログ、通常モードはプログレス表示
- テスト: 2個の統合テスト追加（`test_update_with_progress_display`, `test_update_verbose_without_progress`）
- 全163テスト成功（138 unit + 25 integration）、機能デグレードなし

**実装完了**: v0.8.1

**リリース情報**:

- ブランチ: `feature/progress-display`
- コミット: `b617dbe`

---

### 2.3 対話的選択の改善

**ステータス**: ✅ 実装完了 (v0.8.2)

**現状 (Before)**:

```bash
$ gist-cache-rs run backup
複数のgistが見つかりました:
1. backup_daily.sh
2. backup_files.py
3. backup_old.sh

番号を入力してください: 2
(Enter押下後に実行開始 - 中身を確認できない)
実行中...
```

- 選択肢が番号のリストのみで視認性が悪い
- コンテンツを確認せずに実行するため、誤選択のリスク
- 説明が表示されない

**提案 (After)**:

```bash
$ gist-cache-rs run backup
? gistを選択 (↑↓で移動、Enterで選択):
  > backup_daily.sh - 日次バックアップスクリプト
    backup_files.py - ファイルバックアップ
    backup_old.sh - 旧バックアップ(非推奨)

プレビュー (backup_daily.sh):
  #!/bin/bash
  # 日次バックアップスクリプト
  # 更新: 2025-12-09
  ...

[↑↓: 移動 | Enter: 選択 | Esc: キャンセル]
```

**改善 (Improvement)**:

- **誤実行の防止**: プレビューで中身を確認してから実行
- **視認性の向上**: 矢印キーで選択、説明が見やすい
- **シンプルな操作**: 基本的なナビゲーションのみ

**目的**:

- gist選択時のUXを劇的に改善
- コンテンツをプレビューしてから実行可能に
- より直感的な操作を提供

**効果**:

- 誤ったgistの実行を防止
- 選択前にコンテンツを確認可能
- キーボードでの効率的な操作
- プロフェッショナルなCLI体験

**実装詳細** (v0.8.2で完了):

- 依存関係追加: `dialoguer = "0.12"`
- モジュール: `src/search/query.rs`
- 現在の選択プロンプトを`dialoguer::Select`に置き換え
- ColorfulTheme を使用した視覚的改善
- Escキーでのキャンセルサポート（interact_opt使用）
- 表示形式の改善: "description - files"
- 全163テスト成功（138 unit + 25 integration）、機能デグレードなし

**注記**: プレビューペインは将来の機能拡張として残されています

**実装完了**: v0.8.2

**リリース情報**:

- ブランチ: `feature/interactive-selection-improvement`
- コミット: `c8b1a2c`

---

### 2.4 出力フォーマットオプション

**ステータス**: 固定出力フォーマット（人間が読むためのテキスト形式のみ）

**目的**:

- スクリプトやツールで処理しやすい出力形式を提供
- CI/CDパイプラインでの自動化を容易に
- 用途に応じた最適な表示形式を選択可能に

**効果**:

- 他のツールとの連携が容易（jq、grep、awkなど）
- 自動化スクリプトでの利用が可能
- 大量のgistを見やすく表示
- データの加工・フィルタリングが簡単に

**使用シーン**:

現在の`gist-cache-rs cache list`は人間が読むためのテキスト形式で出力されますが、以下のような場合に不便です：

1. **スクリプトで処理したい**: gist IDだけを抽出したい
2. **大量のgistを管理**: 見やすく整理して表示したい

**提案フォーマット**:

#### JSON出力（スクリプト連携用）

```bash
gist-cache-rs cache list --format json
```

```json
[
  {
    "id": "abc123def456",
    "description": "バックアップスクリプト",
    "files": ["backup.sh"],
    "updated_at": "2025-12-09T23:00:00Z"
  }
]
```

**使用例**:

```bash
# gist IDだけを抽出
gist-cache-rs cache list --format json | jq -r '.[].id'

# スクリプトで処理
for gist_id in $(gist-cache-rs cache list --format json | jq -r '.[].id'); do
  echo "Processing: $gist_id"
done
```

**目的**: 軽量でシンプルなデータ形式
**効果**: スクリプトで処理可能、jqなどのツールで加工が容易

#### テーブルフォーマット

```bash
gist-cache-rs cache list --format table
```

```text
ID           説明             ファイル    更新日時
abc123...    バックアップ     backup.sh   2025-12-09
def456...    テスト           test.py     2025-12-08
```

**目的**: 視覚的に整理された出力
**効果**: 大量のgistを一覧表示する際に見やすい

**実装詳細**:

- モジュール: 新規`src/output.rs`
- JSON出力: 既存の`serde_json`を使用（依存関係追加不要）
- テーブル出力: シンプルなフォーマット関数を実装（外部crateは不要）
- すべてのコマンドで`--format`フラグをサポート

**推定工数**: 小（2-3日）

**依存関係**: なし（既存のserdeを使用）

---

## 優先度3：中（機能拡張）

### 3.1 Gist作成・編集

**ステータス**: 現在は読み取り専用ツール

**現状 (Before)**:

```bash
# 新しいgistを作成したい場合
$ # ブラウザでGitHubを開く...
$ # gist.github.comにアクセス...
$ # ファイルをコピー＆ペースト...
$ # 説明を入力...
$ # 公開設定を選択...
$ # 作成ボタンをクリック...

# 作成後、gist-cacheで使うにはキャッシュ更新が必要
$ gist-cache-rs update

# 既存gistを編集したい場合も同様にブラウザ経由
# CLIワークフローが中断される
```

- gist作成・編集にはブラウザが必須
- ローカルファイルからの作成が面倒（コピペが必要）
- CLI作業の流れが途切れる
- 作成後、手動でキャッシュ更新が必要

**提案 (After)**:

```bash
# ローカルファイルから瞬時にgist作成
$ gist-cache-rs create my_script.py --description "便利なスクリプト" --public
✓ Gist作成完了: https://gist.github.com/abc123def456
✓ キャッシュに追加されました

# すぐに実行可能
$ gist-cache-rs run my_script
# ブラウザ不要、すべてCLIで完結

# 複数ファイルのgistも簡単
$ gist-cache-rs create script.py config.json README.md --description "プロジェクト"
✓ マルチファイルgist作成完了

# 標準入力からも作成可能（パイプライン統合）
$ echo "SELECT * FROM users;" | gist-cache-rs create query.sql --private
✓ Gist作成完了

# 既存gistの編集もCLIで
$ gist-cache-rs edit abc123 --file script.py --content "$(cat updated_script.py)"
✓ Gist更新完了、キャッシュも同期済み

# 不要なgistは削除
$ gist-cache-rs delete abc123 --confirm
⚠️  gist 'backup_old.sh' を削除します。本当によろしいですか? [y/N]: y
✓ Gist削除完了
```

**改善 (Improvement)**:

- **ワークフロー一貫性**: CLI作業を中断せずgist作成～実行まで完結
- **大幅な時間短縮**: ブラウザ起動・コピペ不要、コマンド1つで作成完了（5秒→0.5秒）
- **自動同期**: 作成・編集・削除が即座にキャッシュに反映、手動更新不要
- **スクリプト化可能**: CI/CDパイプラインでgist自動作成が可能に

**目的**:

- gist-cache単体でgistライフサイクル全体を管理
- ブラウザを開かずにCLIだけで完結
- 既存ワークフローにシームレスに統合

**効果**:

- 作業効率の大幅向上
- gist作成から実行までの一貫したUX
- ローカルファイルからgist作成が容易
- キャッシュと同期した状態を維持

**提案コマンド**:

#### 新規Gist作成

```bash
# ファイルから作成
gist-cache-rs create script.py --description "マイスクリプト" --public

# 標準入力から作成
echo "print('hello')" | gist-cache-rs create hello.py --private

# マルチファイルgist作成
gist-cache-rs create file1.py file2.py --description "プロジェクトファイル"
```

**目的**: CLIからgistを作成
**効果**: ブラウザ不要、スクリプト化可能

#### 既存Gist編集

```bash
# gist内の特定ファイルを編集
gist-cache-rs edit <gist-id> --file script.py

# ファイルコンテンツを更新
gist-cache-rs edit <gist-id> --file script.py --content "$(cat new_script.py)"

# 説明を更新
gist-cache-rs edit <gist-id> --description "新しい説明"
```

**目的**: gistの更新を容易に
**効果**: バージョン管理、素早い修正

#### Gist削除

```bash
gist-cache-rs delete <gist-id> --confirm
```

**目的**: 不要なgistのクリーンアップ
**効果**: gist管理の完全性

**実装詳細**:

- モジュール: 新規`src/gist/manager.rs`
- GitHub CLI使用（`gh gist create`、`gh gist edit`、`gh gist delete`）
- 変更後にローカルキャッシュを自動更新
- 破壊的操作には確認プロンプトを追加

**推定工数**: 中（4-5日）

**依存関係**: なし（既存の`gh` CLIを使用）

---

### 3.2 エイリアスシステム

**ステータス**: 未実装

**現状 (Before)**:

```bash
# よく使うgistを実行したい
$ gist-cache-rs run abc123def456ghi789jkl012mno345pqr678  # 32文字のIDを入力...タイプミスしやすい

# 説明で検索することもできるが
$ gist-cache-rs run "daily backup"
複数のgistが見つかりました:
1. daily_backup_v1.sh
2. daily_backup_v2.sh
3. daily_backup_old.sh
番号を入力してください: 2  # 毎回選択が必要

# 長いIDをメモ帳にコピーして保存...
# 毎回検索または長いIDを入力する必要がある
```

- 32文字のgist IDを覚えられない、入力が面倒
- よく使うgistでも毎回検索または選択が必要
- IDをどこかにメモしておく必要がある
- チームで「あのスクリプト」の共通認識がない

**提案 (After)**:

```bash
# よく使うgistにエイリアスを設定
$ gist-cache-rs alias add backup abc123def456...
✓ エイリアス 'backup' を作成しました

$ gist-cache-rs alias add deploy def456ghi789...
✓ エイリアス 'deploy' を作成しました

# 短い名前で瞬時に実行
$ gist-cache-rs run backup
✓ 'backup' (abc123def456...) を実行中
# 32文字 → 6文字に短縮！

$ gist-cache-rs run deploy
✓ 'deploy' (def456ghi789...) を実行中

# エイリアス一覧を確認
$ gist-cache-rs alias list
backup  → abc123def456... (daily_backup_script.sh)
deploy  → def456ghi789... (deploy_production.sh)
report  → ghi789jkl012... (generate_report.py)

# チーム全体で設定ファイルを共有
$ cat ~/.config/gist-cache/config.toml
[aliases]
backup = "abc123def456..."
deploy = "def456ghi789..."
report = "ghi789jkl012..."

# 設定ファイルをgitで共有
$ cp ~/.config/gist-cache/config.toml /path/to/team-repo/gist-cache-config.toml
```

**改善 (Improvement)**:

- **タイプ数の劇的削減**: 32文字のID → 短い覚えやすい名前（backup、deploy、reportなど）
- **記憶の簡素化**: IDを覚える必要なし、意味のある名前で管理
- **チーム標準化**: 設定ファイル共有で「backup」が何を指すか全員が理解
- **生産性向上**: 頻繁に使うスクリプトへのアクセスが5倍速く

**目的**:

- よく使うgistに短い名前を付与
- タイプ数削減と記憶の簡素化
- チーム内での共通名称の共有

**効果**:

- 日常的に使うgistへの高速アクセス
- 長いgist IDを覚える必要がない
- チームの生産性向上
- ワークフローの標準化

**提案使用法**:

```bash
# エイリアス作成
gist-cache-rs alias add my-backup abc123def456

# エイリアス使用
gist-cache-rs run my-backup

# エイリアス一覧
gist-cache-rs alias list

# エイリアス削除
gist-cache-rs alias remove my-backup
```

**保存方法**: 設定ファイル内:

```toml
[aliases]
my-backup = "abc123def456"
daily-report = "def456ghi789"
deploy-prod = "ghi789jkl012"
```

**実装詳細**:

- モジュール: `src/config.rs`（エイリアス管理を追加）
- `src/search/query.rs`を更新してエイリアスを最初にチェック
- CRUD操作を持つ`alias`サブコマンドを追加

**推定工数**: 小（2-3日）

**依存関係**: 設定ファイルサポートが必要 (1.2)

---

## 優先度4：低（高度な機能）

### 4.1 Watchモード

**ステータス**: 未実装

**目的**:

- gistの変更を自動検出して実行
- 開発ワークフローの自動化
- 監視・CI/CD統合

**効果**:

- 手動更新チェック不要
- リアルタイムな変更適用
- 自動テスト・デプロイが可能
- DevOpsワークフローへの統合

**提案API**:

```bash
# gistを監視して変更時に実行
gist-cache-rs watch <gist-id> --interval 60

# カスタムコマンドで監視
gist-cache-rs watch <gist-id> --command "python3 {file}"
```

**ユースケース**:

- 開発ワークフロー: スクリプトの自動リロード
- 監視: ヘルスチェックを定期的に実行
- CI/CD: gist更新時にトリガー

**実装詳細**:

- モジュール: 新規`src/watch/mod.rs`
- gistの`updated_at`タイムスタンプを定期的にチェック
- 変更検出時にスクリプトを実行
- API乱用を避けるためのレート制限を追加

**推定工数**: 中（3-4日）

**依存関係**: `notify` crate（ファイル監視）を追加

---

### 4.2 統計・分析

**ステータス**: 未実装

**目的**:

- gist使用パターンの可視化
- よく使うgistの特定
- 使用状況の分析

**効果**:

- データドリブンなgist管理
- 重要なgistの識別
- 使用傾向の把握
- チームの利用状況の可視化

**提案コマンド**:

```bash
# 全体統計を表示
gist-cache-rs stats

# 最も頻繁に実行されたgist
gist-cache-rs stats top

# 実行履歴
gist-cache-rs stats history --limit 20
```

**出力例**:

```bash
Gist キャッシュ統計
─────────────────────
総Gist数:           150
キャッシュサイズ:   12.5 MB
キャッシュヒット率: 95.2%

最も実行されたもの（過去30日間）:
  1. backup_script.sh    (42回)
  2. test_runner.py      (28回)
  3. deploy.sh           (15回)

最終更新:           2025-12-09 23:00:00
```

**保存方法**: JSONファイルで実行履歴を保存（`~/.cache/gist-cache/stats.json`）

**実装詳細**:

- モジュール: 新規`src/stats/mod.rs`
- 追跡項目: 実行回数、最終実行時刻、成功/失敗率
- JSON形式で保存（シンプルで軽量）
- プライバシー重視: オプション、無効化可能

**推定工数**: 中（4-5日）

**依存関係**: なし（JSONで実装）

---

### 4.3 並列Gistフェッチ

**ステータス**: 現在は逐次処理

**現状 (Before)**:

```bash
# 150個のgistをキャッシュ更新
$ time gist-cache-rs update
Updating cache...
# gist 1をフェッチ... 完了
# gist 2をフェッチ... 完了
# gist 3をフェッチ... 完了
# ...
# gist 150をフェッチ... 完了
Cache updated successfully.

real    2m30s  # 150秒かかる（1gistあたり1秒）
```

- 1つずつ順番にフェッチ（逐次処理）
- 1gistあたり約1秒 × 150gist = 2分30秒
- ネットワーク帯域幅が十分にあっても活用できない
- 大量gistユーザーは毎回数分待つ必要がある

**提案 (After)**:

```bash
# 同じ150個のgistを並列フェッチ（最大10並列）
$ time gist-cache-rs update
Updating cache...
[████████████████████] 100% (150/150 gists) ETA: 0s
並列フェッチ: 10スレッド

新しいgist: 5件
更新されたgist: 15件
変更なし: 130件

Cache updated successfully.

real    0m18s  # 18秒で完了！（約8.3倍高速化）
```

パフォーマンス比較:

| gist数 | 逐次処理 | 並列処理(10並列) | 高速化倍率 |
|--------|---------|-----------------|-----------|
| 50     | 50秒    | 6秒             | 8.3倍     |
| 100    | 1分40秒 | 12秒            | 8.3倍     |
| 150    | 2分30秒 | 18秒            | 8.3倍     |
| 200    | 3分20秒 | 24秒            | 8.3倍     |

**改善 (Improvement)**:

- **劇的な高速化**: 2分30秒 → 18秒（8.3倍高速）、コーヒーブレイク不要に
- **帯域幅の有効活用**: ネットワーク容量を最大限に使い、待ち時間を最小化
- **スケーラビリティ**: gist数が増えても、並列化により影響を軽減
- **ユーザー体験**: 大量gistでもストレスなくキャッシュ更新可能

**目的**:

- キャッシュ更新の高速化
- ネットワーク帯域幅の有効活用
- 大量gistユーザーの体験向上

**効果**:

- 更新時間の大幅短縮（5-10倍の可能性）
- 100+gistでの体感速度向上
- APIレート制限を効率的に活用
- ユーザー満足度の向上

**実装詳細**:

- モジュール: `src/cache/update.rs`
- `tokio::spawn`で並行リクエスト
- レート制限のためのセマフォ実装（最大N個の同時リクエスト）
- プログレストラッキング付きバッチ処理

**例**:

```rust
// 疑似コード
let semaphore = Arc::new(Semaphore::new(10)); // 最大10並列
for gist in gists {
    let permit = semaphore.clone().acquire_owned().await;
    tokio::spawn(async move {
        fetch_gist(gist).await;
        drop(permit);
    });
}
```

**推定工数**: 小（2-3日）

**依存関係**: なし（tokioは既に含まれている）

---

### 4.4 キャッシュ圧縮

**ステータス**: 未実装

**目的**:

- ディスク使用量の削減
- 大量gistユーザーのストレージ節約
- SSD寿命の延長

**効果**:

- 60-80%のディスク容量削減
- キャッシュサイズが数GBになるのを防止
- I/O負荷の軽減（圧縮データの方が小さい）
- 特にテキストファイルで効果大

**提案実装**:

- `zstd`圧縮を使用（高速＋高圧縮率）
- コンテンツキャッシュファイルを透過的に圧縮
- 読み込み時にオンザフライで解凍

**設定**:

```toml
[cache]
compress = true
compression_level = 3  # 1-22、高いほど圧縮率が良いが遅い
```

**実装詳細**:

- モジュール: `src/cache/content.rs`
- `ContentCache::set()`と`get()`に圧縮レイヤーを追加
- 後方互換性あり（圧縮済み vs. 非圧縮を検出）

**推定工数**: 小（2-3日）

**依存関係**: `zstd` crateを追加

---

## セキュリティと信頼性

### 5.1 スクリプト実行の安全性

**現状**: サンドボックス化なし、スクリプトを直接実行

**現状 (Before)**:

```bash
# gistを実行
$ gist-cache-rs run cleanup
実行中...
# 問答無用で実行開始
# 中身が何か確認できない
# もし悪意のあるスクリプトだったら...

# スクリプト内容:
#!/bin/bash
rm -rf /  # 危険！
```

- 実行前の確認プロンプトがない
- スクリプトの中身を確認せずに実行開始
- 危険なコマンド（rm -rf、sudo、curlなど）の警告なし
- 誤って危険なgistを実行するリスク

**提案 (After)**:

```bash
# gistを実行しようとすると自動的に確認プロンプト
$ gist-cache-rs run cleanup

スクリプト: cleanup.sh (ID: abc123...)
インタプリタ: bash
更新日時: 2025-12-09 23:00:00
─────────────────────────────────────
#!/bin/bash
# 古いログファイルを削除
rm -rf /tmp/old_logs
find /var/log -name "*.old" -delete
# ... (最初の20行)
─────────────────────────────────────

⚠️  警告: 潜在的に危険な操作が検出されました:
  - ファイル削除 (rm -rf)
  - システムファイル操作 (/var/log)

このスクリプトを実行しますか? [y/N]: y
実行中...
✓ 完了

# 信頼できるgistはスキップ可能
$ gist-cache-rs run cleanup --no-confirm
実行中... (確認スキップ)
```

設定ファイルで挙動をカスタマイズ:

```toml
[execution]
confirm_before_run = true          # 常に確認
warn_dangerous_commands = true     # 危険コマンドを警告
trusted_gists = ["abc123def456"]   # 信頼するgist（確認スキップ）
```

**改善 (Improvement)**:

- **誤実行の防止**: 実行前にスクリプトの内容を確認でき、意図しない実行を防止
- **危険操作の可視化**: rm -rf、sudo、curlなどを自動検出して警告表示
- **柔軟な制御**: 信頼できるgistは確認をスキップ、設定ファイルで調整可能
- **セキュリティ意識の向上**: 実行前確認により、ユーザーがスクリプトの内容を意識する習慣がつく

**目的**:

- 悪意のあるまたは危険なスクリプトからユーザーを保護
- 実行前の確認によるミスの防止
- 安全性の認識向上

**効果**:

- 誤実行の防止
- セキュリティ意識の向上
- 危険な操作の視覚的警告
- ユーザーの信頼性向上

**提案改善**:

#### 5.1.1 実行確認

```bash
# 常にプレビューを表示して実行前に確認
gist-cache-rs run <query>

# 出力:
スクリプト: backup.sh (ID: abc123...)
インタプリタ: bash
─────────────────────────────────────
#!/bin/bash
rm -rf /tmp/old_backups
# ... (最初の20行)
─────────────────────────────────────
⚠️  上記のスクリプトを実行します。
続行しますか? [y/N]
```

**実装**: 確認プロンプトを追加（設定で無効化可能）

#### 5.1.2 安全性警告

```bash
⚠️  警告: 潜在的に危険な操作が検出されました:
  - ファイル削除 (rm -rf)
  - ネットワークアクセス (curl, wget)
  - システム変更 (sudo)

自己責任で続行しますか? [y/N]
```

**実装**: 危険なコマンドのパターンマッチング

#### 5.1.3 セキュアな一時ファイル

- セキュアパーミッション（0600）で`tempfile` crateを使用
- パニック時も自動クリーンアップ
- ディレクトリトラバーサル攻撃を防止

**推定工数**: 小（2-3日）

---

### 5.2 エラー回復の強化

**現状**: 基本的なエラーハンドリング、限定的な回復

**目的**:

- ネットワーク障害時の自動リトライ
- より分かりやすいエラーメッセージ
- キャッシュ破損からの自動回復

**効果**:

- 一時的なネットワーク問題への耐性
- ユーザーの問題解決を支援
- キャッシュの健全性維持
- トラブルシューティング時間の短縮

**提案改善**:

#### 5.2.1 API失敗時のリトライロジック

```rust
async fn fetch_with_retry<T, F>(
    operation: F,
    max_retries: u32,
) -> Result<T>
where
    F: Fn() -> Result<T>,
{
    let mut retries = 0;
    loop {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) if retries < max_retries && e.is_retryable() => {
                retries += 1;
                let delay = Duration::from_secs(2_u64.pow(retries)); // 指数バックオフ
                tokio::time::sleep(delay).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

#### 5.2.2 より良いエラーメッセージ

```bash
# 現在:
Error: gistの取得に失敗しました

# 提案:
Error: gist 'abc123...' の取得に失敗しました
原因: 30秒後にネットワークタイムアウト
提案: インターネット接続を確認して再試行してください
詳細は --verbose で確認できます

デバッグ情報 (--verbose):
  URL: https://api.github.com/gists/abc123...
  ステータス: 接続タイムアウト
  タイムスタンプ: 2025-12-10 12:34:56
```

#### 5.2.3 キャッシュ破損の回復

- 破損したキャッシュファイルを検出
- 自動再ダウンロード
- 更新前に以前のキャッシュをバックアップ

**推定工数**: 中（3-4日）

---

### 5.3 パストラバーサル保護

**現状**: gistのファイル名を直接使用

**脆弱性**: `../../../../etc/passwd`のようなファイル名を持つ悪意のあるgistが任意の場所に書き込み可能

**目的**: パストラバーサル攻撃の防止

**効果**: セキュリティの強化、悪意のあるgistからの保護

**提案修正**:

```rust
fn sanitize_filename(name: &str) -> Result<PathBuf> {
    let path = Path::new(name);

    // 絶対パスを拒否
    if path.is_absolute() {
        return Err(Error::InvalidFilename("絶対パスは許可されていません"));
    }

    // パストラバーサルを拒否
    for component in path.components() {
        if matches!(component, Component::ParentDir) {
            return Err(Error::InvalidFilename("親ディレクトリ参照は許可されていません"));
        }
    }

    // 安全なパスを返す
    Ok(path.to_path_buf())
}
```

**推定工数**: 小（1日）

---

## 技術的負債

### 6.1 未使用依存関係の削除

**ステータス**: ✅ 完了 (v0.8.0)

**問題**: `reqwest`が含まれているが未使用（直接API実装の名残）

**目的**: 依存関係の最小化とバイナリサイズ削減

**効果**:

- バイナリサイズ約500KB削減
- サプライチェーンリスクの軽減
- ビルド時間の短縮
- メンテナンス負荷の削減

**修正**:

```toml
# Cargo.tomlから削除
# reqwest = { version = "0.11", features = ["json"] }
```

**推定工数**: 極小（5分）

**検証**: `cargo build --release`を実行してバイナリサイズを確認

---

### 6.2 Tokio使用の評価

**問題**: Tokioランタイムが含まれているがほとんどの操作が同期的

**目的**: 非同期ランタイムの正当化または削除の決定

**効果**:

- 並列フェッチ実装でtokio活用（パフォーマンス向上）
- または削除してバイナリサイズ削減

**オプション**:

1. 将来の非同期機能のためにtokioを保持（並列フェッチ、watchモード）
2. tokioを削除して同期操作のみ使用
3. async/awaitを完全に採用（同期コードを書き直し）

**推奨**: tokioを保持し、それを正当化するために並列フェッチを実装

**推定工数**: N/A（決定＋将来作業）

---

### 6.3 コードの重複削除

**問題**: インタプリタ処理ロジックの一部が重複

**目的**: コードの可読性と保守性向上

**効果**:

- DRY原則の適用
- バグ修正が容易に
- テストの簡素化
- コードベースの品質向上

**例**:

- PowerShell検出（複数箇所）
- ファイル vs stdin実行ロジック
- エラーハンドリングパターン

**提案リファクタリング**:

```rust
// src/execution/interpreter.rs
pub enum ExecutionMode {
    Stdin,
    File,
}

impl Interpreter {
    fn preferred_mode(&self) -> ExecutionMode {
        match self {
            Interpreter::Php | Interpreter::Uv | Interpreter::TypeScript(_) => ExecutionMode::File,
            _ => ExecutionMode::Stdin,
        }
    }
}
```

**推定工数**: 中（2-3日）

---

### 6.4 マジックナンバーを定数に

**問題**: コード内にハードコードされた値が散在

**目的**: コードの可読性と保守性向上

**効果**:

- 定数の意味が明確に
- 一箇所での変更が可能
- コードレビューが容易
- ドキュメント不要で理解可能

**例**:

```rust
// src/cache/update.rs
// 現在
if remaining < 100 { /* ... */ }
if remaining < 50 { /* ... */ }

// 改善後
const RATE_LIMIT_WARNING_THRESHOLD: usize = 100;
const RATE_LIMIT_CRITICAL_THRESHOLD: usize = 50;

if remaining < RATE_LIMIT_WARNING_THRESHOLD { /* ... */ }
```

**推定工数**: 小（1日）

---

### 6.5 型安全性の向上

**問題**: Gist ID、ファイル名がプレーン文字列（混同しやすい）

**目的**: コンパイル時の型チェック強化

**効果**:

- バグの早期発見
- API使用の明確化
- ドキュメント不要で意図が明確
- リファクタリングが安全に

**提案**:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GistId(String);

impl GistId {
    pub fn new(id: impl Into<String>) -> Result<Self> {
        let id = id.into();
        if id.len() == 32 && id.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(Self(id))
        } else {
            Err(Error::InvalidGistId(id))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Filename、Descriptionなども同様
```

**推定工数**: 中（3-4日）

---

## テストと開発

### 7.1 テストカバレッジの向上

**現状**: 68.95%カバレッジ（163テスト）

**現状 (Before)**:

```bash
$ cargo tarpaulin --out Stdout
テスト実行中...
163テスト完了

カバレッジサマリー:
||================================||
|| src/cache/content.rs: 45.2%   ||  # 低カバレッジ
|| src/error.rs: 55.8%           ||  # エラーパスが未テスト
|| src/search/query.rs: 78.3%    ||  # 良好
|| src/execution/runner.rs: 72.1% ||
||================================||
|| 全体: 68.95%                   ||
||================================||

# 問題点の確認
未テストのクリティカルパス:
- エラーハンドリング（ネットワークエラー、ファイルI/O失敗）
- エッジケース（空のgist、巨大ファイル、Unicode文字）
- プラットフォーム固有コード（Windows/Unix分岐）
```

- カバレッジ68.95%は中程度だが、エラーパスが弱い
- 新機能追加時、テストなしで進めると品質低下
- リファクタリング時にリグレッションが発生しやすい

**提案 (After)**:

```bash
$ cargo tarpaulin --out Stdout
テスト実行中...
215テスト完了（+52テスト追加）

カバレッジサマリー:
||================================||
|| src/cache/content.rs: 82.5%   ||  ✓ 改善（+37.3%）
|| src/error.rs: 85.2%           ||  ✓ 改善（+29.4%）
|| src/search/query.rs: 85.8%    ||  ✓ 改善（+7.5%）
|| src/execution/runner.rs: 83.2% ||  ✓ 改善（+11.1%）
||================================||
|| 全体: 83.5%                    ||  ✓ 目標達成！
||================================||

追加されたテスト:
✓ エラーパステスト（30件）
  - ネットワークタイムアウトのテスト
  - ファイルI/O失敗のテスト
  - 不正なgist IDのテスト
✓ エッジケーステスト（15件）
  - 空のgist処理
  - 巨大ファイル（10MB+）の処理
  - Unicode文字（日本語、絵文字）の処理
✓ プラットフォームテスト（7件）
  - Windows固有のパス処理
  - Unix固有のパーミッション処理
```

テストポリシーの適用:

```rust
// 新機能追加時の例
// src/cache/clean.rs

#[cfg(test)]
mod tests {
    // 機能テスト: 主要なユースケース
    #[test]
    fn test_clean_older_than() { /* ... */ }

    // エラーパステスト: 失敗ケース
    #[test]
    fn test_clean_with_invalid_date() { /* ... */ }

    // エッジケーステスト: 境界条件
    #[test]
    fn test_clean_empty_cache() { /* ... */ }
}
```

**改善 (Improvement)**:

- **品質の向上**: カバレッジ68.95% → 83.5%、エラーパスも網羅
- **安全なリファクタリング**: テストがあるためコード変更時の不安が激減
- **バグ修正コスト削減**: リリース前にバグを発見、本番での問題が減少
- **継続的な品質維持**: 新機能追加時も必ずテスト付き、品質基準を維持

**目的**:

- バグの早期発見と防止
- リファクタリングの安全性確保
- 品質の維持

**効果**:

- リグレッションの防止
- 安心してコード変更可能
- バグ修正コストの削減
- ユーザーの信頼性向上

**目標**: 80%以上のカバレッジ（実用的な範囲で）

**重点領域**:

1. エラーパス（現在ほとんどテストされていない）
2. エッジケース（空のgist、大きなファイル、Unicode）
3. プラットフォーム固有コード（Windows vs Unix）
4. 新機能の機能テスト

**戦略**:

```bash
# カバレッジレポート生成
cargo tarpaulin --out Html --output-dir coverage

# テストされていない行を特定
# 重要なパスのテストを優先的に追加
```

**テストポリシー**:

- 新機能には必ず機能テストを含める
- 複雑なロジックにはユニットテストを追加
- 過剰な品質追求は避ける（実用性を優先）
- 統合テストで主要なワークフローをカバー

**推定工数**: 中（継続的、各機能に組み込み）

---

### 7.2 クロスプラットフォームCI

**現状**: テストは単一プラットフォームで実行（おそらくLinux）

**目的**:

- 全プラットフォームでの動作保証
- プラットフォーム固有バグの早期発見
- リリース品質の向上

**効果**:

- Windows/macOS/Linuxでの動作保証
- リリース前のバグ検出
- ユーザーの信頼性向上
- サポートコストの削減

**提案**: CI内でLinux、macOS、Windowsでテスト

**GitHub Actions**:

```yaml
# .github/workflows/test.yml
name: Cross-platform Tests

on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable]
    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}
      - run: cargo test
      - run: cargo build --release
```

**推定工数**: 小（1日、機能追加後に実施）

**優先度**: 低（機能追加完了後）

---

## 実装ロードマップ

### フェーズ1：基盤（第1-2週） - 完了

**目標**: 高優先度項目を完了し、将来作業の基盤を確立

1. ✅ `cache clean`コマンドの実装 (1.1) - **完了 (v0.7.0)**
2. ✅ シェル補完スクリプトを追加 (1.3) - **完了 (v0.8.0)**
3. ✅ 未使用の`reqwest`依存関係を削除 (6.1) - **完了 (v0.8.0)**
4. 🔜 設定ファイルサポート (1.2) - 将来検討（導入バージョン検討中）
5. ⏳ エラーメッセージを改善 (5.2.2) - 計画中
6. ⏳ パストラバーサル保護を追加 (5.3) - 計画中

**成果物**: バージョン 0.7.0 (完了)、0.8.0 (完了、PR/リリース待ち)

---

### フェーズ2：ユーザー体験（第3-4週）

**目標**: 使いやすさを向上、設定サポートを追加

1. ✅ 設定ファイルサポート (1.2)
2. ✅ プログレス表示 (2.2)
3. ✅ 実行確認 (5.1.1)
4. ✅ 検索強化 - 正規表現と言語フィルタ (2.1)
5. ✅ 出力フォーマットオプション (2.4)

**成果物**: バージョン 0.8.0

---

### フェーズ3：機能拡張（第5-6週）

**目標**: 新機能を追加、ユースケースを拡大

1. ✅ エイリアスシステム (3.2)
2. ✅ Gist作成・編集 (3.1)
3. ✅ 対話的選択の改善 (2.3)
4. ✅ 並列gistフェッチ (4.3)

**成果物**: バージョン 0.9.0

---

### フェーズ4：高度な機能（第7-8週）

**目標**: パワーユーザー向け機能を追加、パフォーマンスを最適化

1. ✅ Watchモード (4.1)
2. ✅ 統計・分析 (4.2)
3. ✅ キャッシュ圧縮 (4.4)
4. ✅ コードリファクタリングと重複削除 (6.3, 6.4, 6.5)

**成果物**: バージョン 1.0.0

---

### フェーズ5：長期（将来）

**目標**: 継続的な改善とメンテナンス

1. ⏱️ クロスプラットフォームCI強化 (7.2)
2. ⏱️ パフォーマンス最適化
3. ⏱️ ドキュメント改善
4. ⏱️ コミュニティフィードバックに基づく機能追加

---

## 依存関係と影響分析

### フェーズ別の新規依存関係

#### フェーズ1

- `clap_complete` - シェル補完（メンテナンス良好、月間2.5Mダウンロード）

#### フェーズ2

- `toml` - 設定解析（月間55Mダウンロード、軽量）
- `indicatif` - プログレスバー（月間15Mダウンロード、シンプル）
- `regex` - 正規表現検索（月間50M+ダウンロード、標準的）

#### フェーズ3

- `dialoguer` - 対話的プロンプト（月間3Mダウンロード）

#### フェーズ4

- `notify` - ファイル監視（月間7Mダウンロード）
- `zstd` - 圧縮（月間2Mダウンロード）

### バイナリサイズへの影響

| フェーズ | 推定サイズ増加 | 備考 |
|---------|---------------|------|
| 現在 | 約4.5 MB | ベースライン |
| フェーズ1 | +100 KB | 最小限（補完スクリプト） |
| フェーズ2 | +300 KB | 設定解析、プログレスUI、regex |
| フェーズ3 | +400 KB | 対話的UI |
| フェーズ4 | +500 KB | 圧縮、ファイル監視 |
| **合計** | **約5.8 MB** | 軽量を維持 |

### セキュリティ考慮事項

- すべての依存関係はアクティブなセキュリティ監視でメンテナンスされている
- Rustコアチームまたは著名な作者からのcrateを優先
- CIでの定期的な`cargo audit`チェック
- 再現可能なビルドを保証するためにロックファイルをコミット

---

## 成功指標

### 定量的

- テストカバレッジ: 68.95% → 80%以上（実用的な範囲で）
- バイナリサイズ: 10 MB未満
- キャッシュ更新速度: 典型的なワークロードで2秒未満を維持
- ユーザー満足度: GitHubスター、Issue、Discussionsを追跡

### 定性的

- ドキュメントと例の改善
- より良いエラーメッセージ（「次に何をすべきか」の迷いが少ない）
- より柔軟な設定（ハードコードされた動作が少ない）
- よりリッチなエコシステム（補完、プラグイン）

---

## 付録A：却下されたアイデア

### A.1 Webインターフェース

**却下理由**: CLIツールはターミナル使用に集中すべき。Web UIは複雑さとバイナリサイズを大幅に増加させる。GUIを望むユーザーはGitHubのWebインターフェースを使用可能。

### A.2 組み込みGistエディタ

**却下理由**: ユーザーの好みのエディタ（$EDITOR）と統合する方が良い。`gist-cache-rs edit`はエディタを自動で開くことができる。

### A.3 キャッシュのクラウド同期

**却下理由**: Gistは既にクラウド上にある。キャッシュはローカルで高速であることを意図している。キャッシュの同期は目的に反する。

### A.4 Gistコメント/リアクション

**却下理由**: 実行に焦点を当てたCLIツールのスコープ外。ソーシャル機能にはGitHub Webインターフェースを使用。

### A.5 ファジーマッチング検索

**却下理由**: 実用性に疑問があり、誤検出のリスクがある。正規表現とタグベース検索で十分なカバレッジを提供。

### A.6 過剰なテスト（プロパティベーステスト、ベンチマークスイート）

**却下理由**: 実用性を重視し、過剰な品質追求を避ける。機能テストと基本的なユニットテストで十分。カバレッジ80%を目標とし、実装の妨げにならない範囲でテストを実施。

### A.7 プラグインシステム

**却下理由**: 動的ライブラリ読み込み、プラグインAPI、セキュリティ検証など、構造が過剰に複雑化する。「軽量でシンプル」という設計哲学に反する。カスタムインタプリタは設定ファイルで対応可能。

---

## 付録B：コミュニティフィードバックの統合

この計画は生きたドキュメントとして扱うべきです。プロジェクトの進化とコミュニティフィードバックに応じて、優先順位は変わる可能性があります。

**変更提案方法**:

1. 機能リクエストでGitHub Issueを開く
2. GitHub Discussionsで議論
3. コンセンサスに基づいてこの計画を更新
4. 実装でPRを提出

---

## 付録C：破壊的変更ポリシー

### セマンティックバージョニングのコミットメント

- **0.x.y**: マイナーバージョン(x)は破壊的変更を含む可能性がある
- **1.x.y**: メジャーバージョン(1)はAPIを安定化、破壊的変更には2.0が必要
- **x.y.z**: パッチバージョン(z)はバグフィックスのみ

### 移行ガイド

破壊的変更については以下を提供:

1. CHANGELOG.mdでの明確な移行パス
2. 可能であれば以前のバージョンでの非推奨警告
3. 実現可能な場合は自動移行ツール

---

## まとめ

この機能更新計画は、gist-cache-rsのコアの強みである速度、シンプルさ、信頼性を維持しながら拡張するための構造化されたパスを提供します。段階的アプローチにより、定期的なリリースとコミュニティフィードバックの統合を伴う反復的開発が可能になります。

**設計哲学**:

- **軽量でシンプル**: JSONのような軽量なデータ形式を選び、構造を複雑化しない
- **実用性を最優先**: 基本的で日常的なユースケースに焦点を当てる
- **過剰な品質追求を避ける**: 実用的な範囲でのテストと品質維持
- **ユーザーに真の価値を提供**: 実際に役立つ機能のみを追加

**次のステップ**:

1. メンテナー/コミュニティとこの計画をレビュー・議論
2. ユーザーフィードバックに基づいて機能に優先順位を付ける
3. 各主要機能のGitHub Issueを作成
4. フェーズ1の実装を開始

**連絡先**: 質問や提案については、GitHub IssueまたはDiscussionを開いてください。

---

*ドキュメントバージョン: 1.2*
*最終更新: 2025-12-11*
*ステータス: ドラフト - レビュー待ち*
