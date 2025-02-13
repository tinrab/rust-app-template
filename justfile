run:
    RUSTFLAGS="-Awarnings" \
        cargo run --bin rust-app-template

    # nightly
    # RUSTFLAGS="-Awarnings -Z threads=8" \
    #     cargo +nightly run --bin rust-app-template

build:
    RUSTFLAGS="-Awarnings" \
        cargo build

lint:
    cargo fmt --all -- --check

    cargo clippy \
        --no-default-features \
        --features panic_handler -- \
        -D warnings \
        -D trivial_casts \
        -D trivial_numeric_casts \
        -D unused_extern_crates \
        -D unused_import_braces \
        -D unused_qualifications \
        -D clippy::all \
        -D clippy::correctness \
        -D clippy::suspicious \
        -D clippy::complexity \
        -D clippy::perf \
        -D clippy::style \
        -A clippy::missing_safety_doc

test:
    RUSTFLAGS="-Awarnings" cargo test

clean:
    cargo clean
