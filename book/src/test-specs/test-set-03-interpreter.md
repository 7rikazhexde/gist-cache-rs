# gist-cache-rs Functional Verification Test Design Document (Test Set 3: Interpreter Operation Verification)

## Test Objective

To confirm that gist-cache-rs correctly launches each interpreter, passes arguments, and executes scripts.

## Target Functionality

- Bash execution (bash)
- Python execution (python3)
- Ruby execution (ruby)
- Node.js execution (node)
- PHP execution (php)
- Perl execution (perl)
- PowerShell execution (pwsh)
- TypeScript execution (ts-node, deno, bun)
- UV execution (uv run) - PEP 723 compatible

## Prerequisites

- gist-cache-rs is installed
- GitHub CLI is authenticated
- Metadata cache is up-to-date (`gist-cache-rs update` has been executed)
- Each interpreter is installed on the system
- Test Gists for hello_args series exist

## Test Case List

### TC1: Bash Execution

**Objective**: Confirm that Bash scripts are executed correctly.

**Prerequisites**:

- hello_args.sh (ID: 7bcb324e9291fa350334df8efb7f0deb) exists.

**Steps**:

1. Execute with Bash: `gist-cache-rs run --id 7bcb324e9291fa350334df8efb7f0deb bash arg1 arg2 arg3`
2. Check execution result.

**Expected Result**:

- Bash version is displayed.
- Number of arguments "3" is displayed.
- Arguments are displayed correctly (arg1, arg2, arg3).
- "Could not calculate as a number" is displayed as arguments are not numerical.

**Verification Items**:

- Bash interpreter launches correctly.
- Arguments are passed correctly.
- Script executes successfully.

---

### TC2: Python Execution

**Objective**: Confirm that Python scripts are executed correctly.

**Prerequisites**:

- hello_args.py exists.

**Steps**:

1. Execute with Python: `gist-cache-rs run --filename hello_args.py python3 10 20 30`
2. Check execution result.

**Expected Result**:

- Python version is displayed.
- Number of arguments "3" is displayed.
- Arguments are displayed correctly (10, 20, 30).
- If the Python script has a sum calculation function, the sum "60" is displayed.

**Verification Items**:

- Python interpreter launches correctly.
- Arguments are passed correctly.
- Script executes successfully.

---

### TC3: Ruby Execution

**Objective**: Confirm that Ruby scripts are executed correctly.

**Prerequisites**:

- hello_args.rb exists.

**Steps**:

1. Execute with Ruby: `gist-cache-rs run --filename hello_args.rb ruby test1 test2`
2. Check execution result.

**Expected Result**:

- Ruby version is displayed.
- Number of arguments "2" is displayed.
- Arguments are displayed correctly (test1, test2).

**Verification Items**:

- Ruby interpreter launches correctly.
- Arguments are passed correctly.
- Script executes successfully.

---

### TC4: Node.js Execution

**Objective**: Confirm that Node.js scripts are executed correctly.

**Prerequisites**:

- hello_args.js or hello_args_2.js exists.

**Steps**:

1. Execute with Node.js: `gist-cache-rs run --filename hello_args.js node hello world`
2. Check execution result.

**Expected Result**:

- Node.js version is displayed.
- Number of arguments "2" is displayed.
- Arguments are displayed correctly (hello, world).

**Verification Items**:

- Node.js interpreter launches correctly.
- Arguments are passed correctly.
- Script executes successfully.

---

### TC5: PHP Execution

**Objective**: Confirm that PHP scripts are executed correctly.

**Prerequisites**:

- hello_args.php exists.

**Steps**:

1. Execute with PHP: `gist-cache-rs run --filename hello_args.php php 100 200`
2. Check execution result.

**Expected Result**:

- PHP version is displayed.
- Number of arguments "2" is displayed.
- Arguments are displayed correctly (100, 200).
- Confirmed numerical calculation function: `100 + 200 = 300`.

**Verification Items**:

- PHP interpreter launches correctly.
- Arguments are passed correctly.
- Script executes successfully.

---

### TC6: Perl Execution

**Objective**: Confirm that Perl scripts are executed correctly.

**Prerequisites**:

- hello_args.pl exists.

**Steps**:

1. Execute with Perl: `gist-cache-rs run --filename hello_args.pl perl foo bar baz`
2. Check execution result.

**Expected Result**:

- Perl version is displayed.
- Number of arguments "3" is displayed.
- Arguments are displayed correctly (foo, bar, baz).

**Verification Items**:

- Perl interpreter launches correctly.
- Arguments are passed correctly.
- Script executes successfully.

---

### TC7: PowerShell Execution

**Objective**: Confirm that PowerShell scripts are executed correctly.

**Prerequisites**:

- hello_args.ps1 (ID: 2cb45541fee10264b615fd641c577a20) exists.
- `pwsh` command is installed (PowerShell Core).

**Steps**:

1. Execute with PowerShell: `gist-cache-rs run --id 2cb45541fee10264b615fd641c577a20 pwsh test1 test2 test3`
2. Check execution result.

**Expected Result**:

- PowerShell version is displayed.
- Number of arguments "3" is displayed.
- Arguments are displayed correctly (test1, test2, test3).
- "Calculation impossible because it contains non-numerical values" is displayed as arguments are not numerical.

**Verification Items**:

- PowerShell interpreter (`pwsh`) launches correctly.
- Arguments are passed correctly.
- Script executes successfully.

**Numerical Argument Test**:

1. Execute with PowerShell: `gist-cache-rs run --filename hello_args.ps1 pwsh 10 20 30`
2. Expected Result: Sum "60" is displayed.

---

### TC8: TypeScript Execution (Deno)

**Objective**: Confirm that TypeScript scripts are executed correctly with Deno.

**Prerequisites**:

- hello_args_deno.ts (ID: 9b0e7e1bdf7d24c3f28a80d18f6aaafe) exists.
- `deno` command is installed.

**Steps**:

1. Execute with Deno (no arguments): `gist-cache-rs run --filename hello_args_deno.ts deno`
2. Execute with Deno (string arguments): `gist-cache-rs run --filename hello_args_deno.ts deno test1 test2 test3`
3. Execute with Deno (numerical arguments): `gist-cache-rs run --filename hello_args_deno.ts deno 10 20 30`
4. Check each execution result.

**Expected Result**:

- Deno version is displayed.
- TypeScript version is displayed.
- V8 version is displayed.
- No arguments: Usage example is displayed.
- String arguments: Number of arguments "3", arguments displayed correctly (test1, test2, test3), "Calculation impossible because it contains non-numerical values".
- Numerical arguments: Number of arguments "3", arguments displayed correctly (10, 20, 30), sum "60" is displayed.

**Verification Items**:

- Deno interpreter launches correctly.
- TypeScript file (.ts) is recognized correctly.
- `deno run` command is used correctly.
- Arguments are passed correctly.
- Script executes successfully.

---

### TC9: TypeScript Execution (ts-node)

**Objective**: Confirm that TypeScript scripts are executed correctly with ts-node.

**Prerequisites**:

- hello_args.ts (ID: c3c925384cc8241d8cd30f269af84332) exists.
- `ts-node` command is installed (`npm install -g ts-node typescript`).

**Steps**:

1. Execute with ts-node: `gist-cache-rs run --filename hello_args.ts ts-node hello world`
2. Check execution result.

**Expected Result**:

- TypeScript version is displayed.
- Node.js version is displayed.
- Number of arguments "2" is displayed.
- Arguments are displayed correctly (hello, world).
- "Calculation impossible because it contains non-numerical values" is displayed as arguments are not numerical.

**Verification Items**:

- ts-node interpreter launches correctly.
- TypeScript file (.ts) is recognized correctly.
- Arguments are passed correctly.
- Script executes successfully.

**Notes**:

- `ts-node` executes TypeScript on Node.js, so `npm install -g ts-node typescript` is required.
- If not installed, this test case can be skipped.

---

### TC10: TypeScript Execution (Bun)

**Objective**: Confirm that TypeScript scripts are executed correctly with Bun.

**Prerequisites**:

- hello_args_bun.ts (ID: a3d74a884ff923fc83c047c2cf3d6f08) exists.
- `bun` command is installed.

**Steps**:

1. Execute with Bun: `gist-cache-rs run --filename hello_args_bun.ts bun 100 200`
2. Check execution result.

**Expected Result**:

- Bun version is displayed.
- Number of arguments "2" is displayed.
- Arguments are displayed correctly (100, 200).
- Sum "300" is displayed.

**Verification Items**:

- Bun interpreter launches correctly.
- TypeScript file (.ts) is recognized correctly.
- Arguments are passed correctly.
- Script executes successfully.

**Notes**:

- Bun is a fast JavaScript/TypeScript runtime.
- If not installed, this test case can be skipped.

---

### TC11: UV Execution (PEP 723 Compatible)

**Objective**: Confirm that Python scripts are executed correctly with UV (PEP 723 compatible).

**Prerequisites**:

- hello_args.py exists.
- `uv` command is installed.

**Steps**:

1. Execute with UV: `gist-cache-rs run --filename hello_args.py uv 5 10 15`
2. Check execution result.

**Expected Result**:

- Python version is displayed (Python environment managed by uv).
- Number of arguments "3" is displayed.
- Arguments are displayed correctly (5, 10, 15).
- If the script has a sum calculation function, the sum "30" is displayed.

**Verification Items**:

- UV interpreter (`uv run`) launches correctly.
- PEP 723 metadata is processed correctly.
- Arguments are passed correctly.
- Script executes successfully.

---

## Test Execution Order

1. TC1: Bash Execution
2. TC2: Python Execution
3. TC3: Ruby Execution
4. TC4: Node.js Execution
5. TC5: PHP Execution
6. TC6: Perl Execution
7. TC7: PowerShell Execution
8. TC8: TypeScript Execution (Deno)
9. TC9: TypeScript Execution (ts-node)
10. TC10: TypeScript Execution (Bun)
11. TC11: UV Execution (PEP 723)

## Notes

- Ensure each interpreter is installed on the system.
- Tests for uninstalled interpreters can be skipped.
- TC7 (PowerShell) primarily targets PowerShell Core (`pwsh`) on Linux/macOS.
- TC8-10 (TypeScript) verify operation with respective runtimes.
  - **TC8 (Deno)**: Native TypeScript support, most recommended (operation confirmed).
  - **TC9 (ts-node)**: Executes TypeScript on Node.js, requires installation (`npm install -g ts-node typescript`).
  - **TC10 (Bun)**: Native TypeScript support, fast execution (requires installation).
- TC11 (UV) specifically aims to verify PEP 723 compatibility.
- Output format may vary depending on the specific script implementation.
- Argument handling depends on the specifications of each language.

```
