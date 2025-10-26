# justfile

# 全体チェック
check:
    @just fmt-check
    @just lint
    @just test

# 部分チェック
fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --all-targets -- -D warnings

test:
    cargo test --quiet

# 修正付きフォーマット
fmt:
    cargo fmt --all

# CI用（非対話モード）
ci-check:
    RUSTFLAGS="-D warnings" cargo test --all
