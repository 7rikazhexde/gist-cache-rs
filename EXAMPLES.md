# 💡 実例集

gist-cache-rsの実際の使用例を紹介します。

## 🚀 基本的な使い方

### キャッシュ更新

```bash
# 初回または全件更新
$ gist-cache-rs update --verbose
Gistキャッシュを更新しています...
モード: 差分更新
レートリミット残量: 4966
既存のキャッシュを検出しました
GitHubユーザー（キャッシュ再利用）: your-username
最終更新日時: 2025-10-26T02:22:04Z
GitHub APIからGist情報を取得中...
取得したGist数: 1
差分マージ完了: 既存 124 + 差分 1 → 総数 124
更新: 1件
キャッシュ更新が完了しました
総Gist数: 124

# 更新がない場合
$ gist-cache-rs update --verbose
Gistキャッシュを更新しています...
モード: 差分更新
レートリミット残量: 4964
既存のキャッシュを検出しました
GitHubユーザー（キャッシュ再利用）: your-username
最終更新日時: 2025-10-26T02:35:44Z
GitHub APIからGist情報を取得中...
取得したGist数: 0
更新なし
キャッシュ更新が完了しました
総Gist数: 124
```

---

## 🐚 Bashスクリプトの例

### 例1: 連番フォルダ作成スクリプト

**Gistの説明:** 指定パスに連番付き（開始番号〜終了番号）のフォルダを100件単位で作成するスクリプト

#### 📋 プレビューモードで内容確認

```bash
$ gist-cache-rs run -p create_folder
Description: 指定パスに連番付き（開始番号〜終了番号）のフォルダを100件単位で作成するスクリプト #bash
Files: create_folders.sh

=== Gist内容 ===
--- create_folders.sh ---
#!/bin/bash
# 指定パスに連番付き（開始番号〜終了番号）のフォルダを100件単位で作成するスクリプト

show_usage() {
  echo "使い方: $0 [接頭辞] [保存先] [開始番号] [終了番号]"
  echo ""
  echo "引数を省略した場合は対話的に入力できます"
  # ... (以下省略)
}
# ... (スクリプト本体)
```

#### 🎯 部分一致検索で複数候補から選択

```bash
$ gist-cache-rs run -p create
複数のGistが見つかりました:

 1. 指定パスに連番付き（開始番号〜終了番号）のフォルダを100件単位で作成するスクリプト #bash | create_folders.sh
 2. Create GitHub Gist with CLI | create_gist.sh
 3. Create multiple directories | create_dirs.sh
 4. Create backup archive | create_backup.sh
 5. Create project template | create_template.sh
 6. Create Docker container | create_container.sh
 7. Create test data | create_testdata.py

番号を選択してください (1-7): 1

Description: 指定パスに連番付き（開始番号〜終了番号）のフォルダを100件単位で作成するスクリプト #bash
Files: create_folders.sh
# ... (内容が表示される)
```

#### 💬 対話モードで実行

```bash
$ gist-cache-rs run -i create_folder
Description: 指定パスに連番付き（開始番号〜終了番号）のフォルダを100件単位で作成するスクリプト #bash
Files: create_folders.sh
実行中: create_folders.sh (bash)

使い方: /tmp/create_folders.sh [接頭辞] [保存先] [開始番号] [終了番号]

引数を省略した場合は対話的に入力できます

例: /tmp/create_folders.sh aaa /path/to/directory 1000 1500

------------------------------------------------------
 ~$ /tmp/create_folders.sh aaa bbb 0 200
 フォルダを作成: ./bbb/aaa_No.0-99 (範囲: 0-99)
 フォルダを作成: ./bbb/aaa_No.100-200 (範囲: 100-200)
------------------------------------------------------

対話モードで実行しますか？ (y/N): y

=== 対話モード ===
接頭辞を入力してください: test1
保存先ディレクトリを入力してください: ./test
開始番号を入力してください: 0
終了番号を入力してください: 1000

フォルダを作成: ./test/test1_No.0-99 (範囲: 0-99)
フォルダを作成: ./test/test1_No.100-199 (範囲: 100-199)
フォルダを作成: ./test/test1_No.200-299 (範囲: 200-299)
フォルダを作成: ./test/test1_No.300-399 (範囲: 300-399)
フォルダを作成: ./test/test1_No.400-499 (範囲: 400-499)
フォルダを作成: ./test/test1_No.500-599 (範囲: 500-599)
フォルダを作成: ./test/test1_No.600-699 (範囲: 600-699)
フォルダを作成: ./test/test1_No.700-799 (範囲: 700-799)
フォルダを作成: ./test/test1_No.800-899 (範囲: 800-899)
フォルダー作成: ./test/test1_No.900-999 (範囲: 900-999)
フォルダを作成: ./test/test1_No.1000-1000 (範囲: 1000-1000)
処理が完了しました。
```

**ポイント:**

- 📝 `-i` オプションで対話モードを有効化
- 💬 スクリプト内の`read`コマンドが正しく動作
- ✅ ユーザー入力を受け付けながらスクリプトが実行される

---

## 🐍 Pythonスクリプトの例

### 例2: Pandas/NumPyデータ分析（PEP 723対応）

**Gistの説明:** data_analysis.py - Pandas/NumPy使用例 #python #pandas #numpy #uv #pep723 #csv

#### 🏷️ タグで検索（プレビュー）

```bash
$ gist-cache-rs run -p '#pep723'
複数のGistが見つかりました:

 1. data_analysis.py - Pandas/NumPy使用例 #python #pandas #numpy #uv #pep723 #csv | data_analysis.py
 2. uv_test.py - UV一時インストールテスト #python #pandas #numpy #uv #pep723 | uv_test.py

番号を選択してください (1-2): 1

Description: data_analysis.py - Pandas/NumPy使用例 #python #pandas #numpy #uv #pep723 #csv
Files: data_analysis.py

=== Gist内容 ===
--- data_analysis.py ---
#!/usr/bin/env python3
# /// script
# dependencies = ["pandas", "numpy"]
# ///

import pandas as pd
import numpy as np
import sys
import os

def main() -> None:
    print(f"Pandas version: {pd.__version__}")
    print(f"NumPy version: {np.__version__}")
    
    if len(sys.argv) < 2:
        print("エラー: CSVファイルのパスを指定してください (例: input.csv)")
        sys.exit(1)
    
    csv_file = sys.argv[1]
    
    if not os.path.exists(csv_file):
        print(f"エラー: ファイル '{csv_file}' が見つかりません。")
        sys.exit(1)
    
    # CSVファイルを読み込み
    try:
        df = pd.read_csv(csv_file)
        print(f"\nCSVファイル '{csv_file}' を読み込みました (行数: {len(df)})")
        print("\nDataFrame (最初の5行):")
        print(df.head())
        
        # 簡単なデータ分析
        print(f"\n列の数: {len(df.columns)}")
        print(f"\n平均値:\n{df.mean(numeric_only=True)}")
        
        # ランダムデータを追加例として生成（オプション）
        if len(df) > 0:
            print(f"\nランダム列 'Random' を追加:")
            df["Random"] = np.random.randint(1, 100, len(df))
            print(df[["Random"]].head())
            
    except Exception as e:
        print(f"エラー: CSV処理中に例外が発生しました: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
```

#### 📦 uvで実行（依存関係自動管理）

```bash
$ gist-cache-rs run 723 uv sample/input.csv
複数のGistが見つかりました:

 1. data_analysis.py - Pandas/NumPy使用例 #python #pandas #numpy #uv #pep723 #csv | data_analysis.py
 2. uv_test.py - UV一時インストールテスト #python #pandas #numpy #uv #pep723 | uv_test.py

番号を選択してください (1-2): 1

Description: data_analysis.py - Pandas/NumPy使用例 #python #pandas #numpy #uv #pep723 #csv
Files: data_analysis.py
実行中: data_analysis.py (python3)

Pandas version: 2.3.3
NumPy version: 2.3.4

CSVファイル 'sample/input.csv' を読み込みました (行数: 5)

DataFrame (最初の5行):
    A   B
0  77  28
1   5  65
2  47  34
3  84  82
4  65  46

列の数: 2

平均値:
A    55.6
B    51.0
dtype: float64

ランダム列 'Random' を追加:
   Random
0      67
1      70
2       7
3      74
4      60
```

**ポイント:**

- 📦 PEP 723メタデータ（`# /// script`）により依存関係を定義
- ⚡ `uv`が自動的にpandas、numpyをインストール
- 🔧 引数`sample/input.csv`がスクリプトに渡される
- 🎯 グローバル環境を汚さず一時的に実行

---

## 🔍 検索テクニック

### キーワード検索のコツ

#### 1. 部分一致検索

```bash
# "create" を含むすべてのGistを検索
$ gist-cache-rs run create

# "data" を含むすべてのGistを検索
$ gist-cache-rs run data
```

#### 2. タグ検索

```bash
# ハッシュタグで絞り込み
$ gist-cache-rs run '#bash'
$ gist-cache-rs run '#python'
$ gist-cache-rs run '#pep723'
```

#### 3. ファイル名検索

```bash
# ファイル名で直接検索
$ gist-cache-rs run --filename data_analysis.py
$ gist-cache-rs run --filename create_folders.sh
```

#### 4. 説明文検索

```bash
# 説明文でのみ検索
$ gist-cache-rs run --description "データ分析"
$ gist-cache-rs run --description "Numpy"
```

#### 5. ID直接指定

```bash
# GistのIDで直接実行
$ gist-cache-rs run --id [your_gist_id] uv input.csv
```

---

## 💡 便利なエイリアス

```bash
# ~/.bashrc または ~/.zshrc に追加
alias gcrsu='gist-cache-rs update'
alias gcrsr='gist-cache-rs run'
alias gcrsr-p='gist-cache-rs run -p'
alias gcrsr-i='gist-cache-rs run -i'

# 使用例
gcrsu                        # キャッシュ更新
gcrsr-p data                 # プレビュー
gcrsr-i setup                # 対話モード実行
gcrsr backup bash /src /dst  # 引数付き実行
```

---

## 🗂️ キャッシュ管理の例

### キャッシュ一覧の確認

```bash
$ gist-cache-rs cache list
キャッシュされたGist一覧:

ID: 7bcb324e9291fa350334df8efb7f0deb
  説明: hello_args.sh - 引数表示スクリプト #bash #test
  ファイル: hello_args.sh
  更新日時: 2025-10-26 12:30:45

ID: e3a6336c9f3476342626551372f14d6e
  説明: data_analysis.py - Pandas/NumPy使用例 #python #pep723
  ファイル: data_analysis.py
  更新日時: 2025-10-25 18:22:10

合計: 2件のGistがキャッシュされています
```

### キャッシュサイズの確認

```bash
$ gist-cache-rs cache size
キャッシュサイズ情報:

キャッシュされたGist数: 15件
合計サイズ: 89.45 KB
キャッシュディレクトリ: /home/user/.cache/gist-cache/contents
```

### 全キャッシュの削除

```bash
$ gist-cache-rs cache clear
全キャッシュの削除

15件のGistキャッシュを削除します。よろしいですか？
  この操作は取り消せません。

続行しますか？ (y/N): y

全キャッシュを削除しました
```

---

## 🔄 強制更新オプションの使用例

### 開発中のGistを常に最新版で実行

```bash
# 開発中のスクリプトを編集→実行のサイクルで使用
$ gist-cache-rs run --force test-script bash arg1 arg2

# 内部的に以下の動作を実行：
# 1. メタデータキャッシュを差分更新
# 2. Gistが更新されていればコンテンツキャッシュを削除
# 3. 最新版を取得して実行
# 4. 新しいキャッシュを作成
```

### 検索オプションと組み合わせ

```bash
# 説明文で検索して、常に最新版を実行
$ gist-cache-rs run --force --description "backup script" bash /src /dst

# ファイル名で検索して、最新版を実行
$ gist-cache-rs run --force --filename deploy.sh bash
```

**ポイント:**
- 📡 実行前に自動的に`update`を実行（差分更新）
- ⚡ Gistが更新されていなければ、既存のキャッシュを使用して高速実行
- 🔄 更新されている場合のみ、新しいバージョンを取得

---

## 🎯 Tips & トリック

### 1. 最近更新したGistをすぐ実行

```bash
# キャッシュは更新日時降順でソートされているため、
# 部分一致で最初に見つかったものが最新
$ gist-cache-rs run keyword
```

### 2. 複数ファイルのGist

```bash
# 複数ファイルがある場合、最初のファイルが実行される
$ gist-cache-rs run multi-file-gist
```

### 3. デバッグモード

```bash
# verboseモードでデバッグ情報を表示
$ gist-cache-rs update --verbose

# プレビューで実行前に内容確認
$ gist-cache-rs run -p script-name
```

### 4. エイリアスと組み合わせ

```bash
# 頻繁に使うスクリプトはエイリアス化
alias analyze='gcrsr data_analysis uv'

# 使用例
analyze mydata.csv
```

---

## 🚨 トラブルシューティング

### Q: スクリプトが見つからない

```bash
# キャッシュを更新
$ gist-cache-rs update

# verbose モードで詳細確認
$ gist-cache-rs update --verbose
```

### Q: 対話モードが動作しない

```bash
# -i オプションを使用
$ gist-cache-rs run -i script-name

# bash の場合は -i なしでも動作する場合があります
$ gist-cache-rs run script-name bash
```

### Q: uvでエラーが発生

```bash
# uvがインストールされているか確認
$ which uv

# python3で実行してみる
$ gist-cache-rs run script-name python3
```

---

## 📚 関連ドキュメント

- [README.md](README.md) - プロジェクト概要と基本機能
- [INSTALL.md](INSTALL.md) - インストール方法
- [QUICKSTART.md](QUICKSTART.md) - 5分で始めるガイド
