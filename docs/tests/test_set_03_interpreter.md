# gist-cache-rs 機能検証テスト設計書（テストセット3：インタープリタ動作検証）

## テスト目的

gist-cache-rsが各インタープリタを正しく起動し、引数を渡して実行できることを確認する。

## テスト対象機能

- Bash実行（bash）
- Python実行（python3）
- Ruby実行（ruby）
- Node.js実行（node）
- PHP実行（php）
- Perl実行（perl）
- PowerShell実行（pwsh）
- UV実行（uv run）- PEP 723対応

## 前提条件

- gist-cache-rsがインストール済み
- GitHub CLIが認証済み
- メタデータキャッシュが最新（`gist-cache-rs update`実行済み）
- 各インタープリタがシステムにインストール済み
- テスト用のhello_args系Gistが存在する

## テストケース一覧

### TC1: Bash実行

**目的**: Bashスクリプトが正しく実行されることを確認

**前提条件**:
- hello_args.sh (ID: 7bcb324e9291fa350334df8efb7f0deb) が存在する

**手順**:
1. Bashで実行: `gist-cache-rs run --id 7bcb324e9291fa350334df8efb7f0deb bash arg1 arg2 arg3`
2. 実行結果を確認

**期待結果**:
- Bashバージョンが表示される
- 引数の数「3」が表示される
- 引数が正しく表示される（arg1, arg2, arg3）
- 数値以外のため「数値として計算できませんでした」が表示される

**検証項目**:
- Bashインタープリタが正しく起動する
- 引数が正しく渡される
- スクリプトが正常に実行される

---

### TC2: Python実行

**目的**: Pythonスクリプトが正しく実行されることを確認

**前提条件**:
- hello_args.py が存在する

**手順**:
1. Pythonで実行: `gist-cache-rs run --filename hello_args.py python3 10 20 30`
2. 実行結果を確認

**期待結果**:
- Pythonバージョンが表示される
- 引数の数「3」が表示される
- 引数が正しく表示される（10, 20, 30）
- 数値のため合計「60」が表示される（Pythonスクリプトが合計計算機能を持つ場合）

**検証項目**:
- Pythonインタープリタが正しく起動する
- 引数が正しく渡される
- スクリプトが正常に実行される

---

### TC3: Ruby実行

**目的**: Rubyスクリプトが正しく実行されることを確認

**前提条件**:
- hello_args.rb が存在する

**手順**:
1. Rubyで実行: `gist-cache-rs run --filename hello_args.rb ruby test1 test2`
2. 実行結果を確認

**期待結果**:
- Rubyバージョンが表示される
- 引数の数「2」が表示される
- 引数が正しく表示される（test1, test2）

**検証項目**:
- Rubyインタープリタが正しく起動する
- 引数が正しく渡される
- スクリプトが正常に実行される

---

### TC4: Node.js実行

**目的**: Node.jsスクリプトが正しく実行されることを確認

**前提条件**:
- hello_args.js または hello_args_2.js が存在する

**手順**:
1. Node.jsで実行: `gist-cache-rs run --filename hello_args.js node hello world`
2. 実行結果を確認

**期待結果**:
- Node.jsバージョンが表示される
- 引数の数「2」が表示される
- 引数が正しく表示される（hello, world）

**検証項目**:
- Node.jsインタープリタが正しく起動する
- 引数が正しく渡される
- スクリプトが正常に実行される

---

### TC5: PHP実行

**目的**: PHPスクリプトが正しく実行されることを確認

**前提条件**:
- hello_args.php が存在する

**手順**:
1. PHPで実行: `gist-cache-rs run --filename hello_args.php php 100 200`
2. 実行結果を確認

**期待結果**:
- PHPバージョンが表示される
- 引数の数「2」が表示される
- 引数が正しく表示される（100, 200）

**検証項目**:
- PHPインタープリタが正しく起動する
- 引数が正しく渡される
- スクリプトが正常に実行される

---

### TC6: Perl実行

**目的**: Perlスクリプトが正しく実行されることを確認

**前提条件**:
- hello_args.pl が存在する

**手順**:
1. Perlで実行: `gist-cache-rs run --filename hello_args.pl perl foo bar baz`
2. 実行結果を確認

**期待結果**:
- Perlバージョンが表示される
- 引数の数「3」が表示される
- 引数が正しく表示される（foo, bar, baz）

**検証項目**:
- Perlインタープリタが正しく起動する
- 引数が正しく渡される
- スクリプトが正常に実行される

---

### TC7: PowerShell実行

**目的**: PowerShellスクリプトが正しく実行されることを確認

**前提条件**:
- hello_args.ps1 (ID: 2cb45541fee10264b615fd641c577a20) が存在する
- pwshコマンドがインストール済み（PowerShell Core）

**手順**:
1. PowerShellで実行: `gist-cache-rs run --id 2cb45541fee10264b615fd641c577a20 pwsh test1 test2 test3`
2. 実行結果を確認

**期待結果**:
- PowerShellバージョンが表示される
- 引数の数「3」が表示される
- 引数が正しく表示される（test1, test2, test3）
- 数値以外のため「数値以外が含まれているため、計算できませんでした」が表示される

**検証項目**:
- PowerShellインタープリタ（pwsh）が正しく起動する
- 引数が正しく渡される
- スクリプトが正常に実行される

**数値引数のテスト**:
1. PowerShellで実行: `gist-cache-rs run --filename hello_args.ps1 pwsh 10 20 30`
2. 期待結果: 合計「60」が表示される

---

### TC8: UV実行（PEP 723対応）

**目的**: UV（PEP 723対応）でPythonスクリプトが正しく実行されることを確認

**前提条件**:
- hello_args.py が存在する
- uvコマンドがインストール済み

**手順**:
1. UVで実行: `gist-cache-rs run --filename hello_args.py uv 5 10 15`
2. 実行結果を確認

**期待結果**:
- Pythonバージョンが表示される（uvが管理するPython環境）
- 引数の数「3」が表示される
- 引数が正しく表示される（5, 10, 15）
- 数値のため合計「30」が表示される（スクリプトが合計計算機能を持つ場合）

**検証項目**:
- UVインタープリタ（`uv run`）が正しく起動する
- PEP 723メタデータが正しく処理される
- 引数が正しく渡される
- スクリプトが正常に実行される

---

## テスト実行順序

1. TC1: Bash実行
2. TC2: Python実行
3. TC3: Ruby実行
4. TC4: Node.js実行
5. TC5: PHP実行
6. TC6: Perl実行
7. TC7: PowerShell実行
8. TC8: UV実行（PEP 723）

## 注意事項

- 各インタープリタがシステムにインストールされていることを確認すること
- インストールされていないインタープリタのテストはスキップ可能
- TC7（PowerShell）は主にLinux/macOS上のPowerShell Core（pwsh）を対象とする
- TC8（UV）は特にPEP 723対応の検証が目的
- 各スクリプトの実装内容により、出力形式が異なる場合がある
- 引数の処理方法は各言語の仕様に依存する
