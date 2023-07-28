# list targets (default)
list:
    just --list

# Format source with rustfmt nightly
format:
    cargo +nightly fmt --all

# Run checks on project (run before pusing)
check:
    cargo +nightly fmt --check
    cargo clippy --no-default-features --all
    cargo clippy --all

# generate html coverage report
coverage:
    cargo llvm-cov --html --all

# Create release build
release:
    cd ui && trunk build --release
    cargo build --release
