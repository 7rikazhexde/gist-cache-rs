# gist-cache-rs 機能検証テスト設計書

## テスト目的

gist-cache-rsの2層キャッシング機能が設計通りに正しく動作することを確認する。

## テスト対象Gist

- **Gist ID**: 7bcb324e9291fa350334df8efb7f0deb
- **ファイル名**: hello_args.sh
- **説明**: Bash引数テストスクリプト
- **URL**: https://gist.github.com/7rikazhexde/7bcb324e9291fa350334df8efb7f0deb

## 前提条件

- gist-cache-rsがインストール済み
- GitHub CLIが認証済み
- メタデータキャッシュが最新（`gist-cache-rs update`実行済み）

## テストケース一覧

### TC1: 初回実行（コンテンツキャッシュなし）

**目的**: 初回実行時にGitHub APIから取得し、コンテンツキャッシュが作成されることを確認

**前提条件**:

- メタデータキャッシュは存在する
- hello_args.shのコンテンツキャッシュは存在しない

**手順**:

1. コンテンツキャッシュを削除: `rm -rf ~/.cache/gist-cache/contents/7bcb324e9291fa350334df8efb7f0deb/`
2. 実行: `gist-cache-rs run hello_args.sh bash arg1 arg2 arg3`
3. キャッシュファイルの存在確認: `ls ~/.cache/gist-cache/contents/7bcb324e9291fa350334df8efb7f0deb/`

**期待結果**:

- メッセージ「情報: キャッシュが存在しないため、GitHub APIから取得します...」が表示される
- スクリプトが正常に実行される
- 引数が正しく表示される（arg1, arg2, arg3）
- コンテンツキャッシュファイルが作成される（`~/.cache/gist-cache/contents/7bcb324e9291fa350334df8efb7f0deb/hello_args.sh`）

---

### TC2: 2回目実行（コンテンツキャッシュあり）

**目的**: 2回目以降の実行時にキャッシュから高速に読み込まれることを確認

**前提条件**:

- TC1完了（コンテンツキャッシュが存在する）

**手順**:

1. 実行: `gist-cache-rs run hello_args.sh bash test1 test2`
2. 実行時間を体感的に確認

**期待結果**:

- 「情報: キャッシュが存在しないため...」のメッセージは**表示されない**
- スクリプトが即座に実行される（ネットワーク待機なし）
- 引数が正しく表示される（test1, test2）
- キャッシュから読み込まれるため、TC1より高速

---

### TC3: update コマンドによるメタデータ更新（変更なし）

**目的**: Gistに変更がない場合、コンテンツキャッシュが維持されることを確認

**前提条件**:

- TC2完了（コンテンツキャッシュが存在する）

**手順**:

1. updateコマンド実行: `gist-cache-rs update --verbose`
2. キャッシュファイルの存在確認: `ls -la ~/.cache/gist-cache/contents/7bcb324e9291fa350334df8efb7f0deb/`
3. 実行: `gist-cache-rs run hello_args.sh bash check`

**期待結果**:

- updateコマンドで「更新なし」または「更新: 0件」と表示される
- コンテンツキャッシュファイルが削除されない
- 実行時にキャッシュから読み込まれる（APIメッセージなし）

---

### TC4: Gist更新後の動作

**目的**: Gistが更新された場合、update後にコンテンツキャッシュが削除され、次回実行時に最新版が取得されることを確認

**前提条件**:

- TC3完了（コンテンツキャッシュが存在する）

**手順**:

1. GitHub上でhello_args.shを編集（例: コメント行を追加）
2. キャッシュファイルのタイムスタンプを記録: `stat ~/.cache/gist-cache/contents/7bcb324e9291fa350334df8efb7f0deb/hello_args.sh`
3. updateコマンド実行: `gist-cache-rs update --verbose`
4. キャッシュファイルの存在確認: `ls ~/.cache/gist-cache/contents/7bcb324e9291fa350334df8efb7f0deb/`
5. 実行: `gist-cache-rs run hello_args.sh bash updated`

**期待結果**:

- updateコマンドで「更新: 1件」と表示される
- コンテンツキャッシュディレクトリが削除される（`contents/7bcb324e9291fa350334df8efb7f0deb/`が存在しない）
- 実行時に「情報: キャッシュが存在しないため、GitHub APIから取得します...」が表示される
- 最新版のスクリプトが実行される（編集内容が反映されている）
- 新しいコンテンツキャッシュが作成される

---

### TC5: --forceオプションの動作

**目的**: run --forceが実行前に自動的にupdateを実行することを確認

**前提条件**:

- TC4完了（コンテンツキャッシュが存在する）

**手順**:

1. GitHub上でhello_args.shを再度編集（例: 別のコメントを追加）
2. **updateコマンドを実行せず**に、--forceオプション付きで実行: `gist-cache-rs run --force hello_args.sh bash force_test`

**期待結果**:

- 実行前に自動的にメタデータキャッシュが更新される（内部処理）
- Gistが更新されているため、コンテンツキャッシュが削除される
- 最新版のスクリプトが実行される（2回目の編集内容が反映されている）
- 新しいコンテンツキャッシュが作成される

---

### TC6: cache listコマンド

**目的**: キャッシュ一覧が正しく表示されることを確認

**前提条件**:

- TC5完了（コンテンツキャッシュが存在する）

**手順**:

1. `gist-cache-rs cache list`を実行

**期待結果**:

- hello_args.sh (ID: 7bcb324e9291fa350334df8efb7f0deb)が一覧に表示される
- 説明文、ファイル名、更新日時が表示される
- 合計キャッシュ数が表示される

---

### TC7: cache sizeコマンド

**目的**: キャッシュサイズが正しく表示されることを確認

**前提条件**:

- TC6完了

**手順**:

1. `gist-cache-rs cache size`を実行

**期待結果**:

- キャッシュされたGist数が表示される
- 合計サイズが表示される（KB単位など）
- キャッシュディレクトリのパスが表示される

---

### TC8: cache clearコマンド

**目的**: 全キャッシュが削除されることを確認

**前提条件**:

- TC7完了（コンテンツキャッシュが存在する）

**手順**:

1. `gist-cache-rs cache clear`を実行
2. 確認プロンプトで`y`を入力
3. キャッシュディレクトリを確認: `ls ~/.cache/gist-cache/contents/`

**期待結果**:

- 確認プロンプトが表示される
- 「全キャッシュを削除しました」というメッセージが表示される
- contentsディレクトリが空になる
- 次回実行時は初回実行として動作する

---

## テスト実行順序

1. TC1: 初回実行（キャッシュなし）
2. TC2: 2回目実行（キャッシュあり）
3. TC3: update（変更なし）
4. TC4: Gist更新後の動作 ← **GitHub上での編集が必要**
5. TC5: --forceオプション ← **GitHub上での再編集が必要**
6. TC6: cache list
7. TC7: cache size
8. TC8: cache clear

## 注意事項

- TC4とTC5ではGitHub上でのGist編集が必要
- 編集内容は軽微なもの（コメント追加など）で十分
- テスト実行前にメタデータキャッシュを最新化しておく（`gist-cache-rs update`）
- 各テストケース間で状態が引き継がれるため、順序通りに実行すること
