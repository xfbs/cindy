# list targets (default)
list:
    just --list

# Run unit tests
test:
    cargo test --all
    cd ui && cargo test --all

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

# Build Cindy. Builds debug build by default, add `--release` for optimized build.
build MODE="":
    cd ui && trunk build {{MODE}}
    cargo build {{MODE}}

# Build Docker builder image (values `website`, `frontend`, `backend`).
docker-builder NAME:
    docker build . -f docker/{{NAME}}.dockerfile -t cindy-{{NAME}}-builder

# Build frontend with the given arguments (leave empty for debug or set to `--release`)
docker-frontend ARGS="":
    just docker-builder frontend
    docker run -it --rm -v $(pwd):/code --workdir /code/ui cindy-frontend-builder trunk build {{ARGS}}

# Build backend with the given arguments (leave empty for debug or set to `--release`)
docker-backend ARGS="":
    just docker-builder backend
    docker run -it --rm -v $(pwd):/code --workdir /code cindy-backend-builder cargo build {{ARGS}}

# Build the given arguments (leave empty for debug or set to `--release`)
docker-build ARGS="":
    just docker-frontend {{ARGS}}
    just docker-backend {{ARGS}}
