name := "bump"

export RUST_BACKTRACE := "1"
export RUST_LOG := "trace"

default:
    just --list

# Install depend tools
deps:
    rustup component add rustfmt
    rustup component add clippy
    rustup component add rust-src
    cargo install cargo-outdated
    cargo install cargo-audit
    cargo install cargo-llvm-cov
    rustup show

# Execute a main.rs
run *arg="major 1.3.3": fix fmt clippy
    cargo run -- {{ arg }}

# Run the tests
test: fix fmt clippy
    cargo test -- --nocapture

# Check syntax, but don't build object files
check: fix fmt clippy
    cargo check

# Build all project
build:
    cargo build

# Build all project
release-build:
    cargo build --release

# Check module version
check-lib:
    cargo outdated -R

# Update modules
update:
    cargo update

# Remove the target directory
clean:
    cargo clean

# Run fmt
fix:
    cargo fix --allow-staged --allow-dirty

# Run fmt
fmt:
    cargo fmt

# Run fmt
fmt-check:
    cargo fmt --all -- --check

# Run clippy
clippy:
    cargo clippy --all-features -- -D warnings

# Run benchmark
bench:
    cargo bench

# Audit your dependencies for crates with security vulnerabilities reported
audit:
    cargo audit

# Build container
container version:
    docker buildx build --platform=linux/amd64,linux/arm64 -t ghcr.io/watawuwu/{{name}}:{{version}} --push .

# SouceCode base coverage
coverage:
    cargo llvm-cov --open
