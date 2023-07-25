# Format source with rustfmt nightly
format:
    cargo +nightly fmt

# Run checks on project (run before pusing)
check:
    cargo fmt --check
    cargo check --no-features
    cargo check

# Create release build
release:
    cd ui && trunk build --release
    cargo build --release
