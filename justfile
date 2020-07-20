target := 'x86_64-unknown-linux-gnu'

_readme: setup-cargo

_validate:
    #!/usr/bin/env bash
    set -Eeou pipefail
    
    echo 'Making sure all changes have been committed...'
    if [[ $(git diff --stat) != '' ]]; then
        echo 'Working tree dirty, not allowing publish until all changes have been committed.'
        #exit 1
    fi

    echo 'Running "cargo check"'
    cargo check --all-features --tests --examples --bins --benches

    echo 'Running unit tests'
    cargo test --all-features

@setup-cargo:
    # DOGFOODING
    cargo install templar --features bin
    # Other stuff
    cargo install cargo-deb
    cargo install cargo-readme

build:
    cargo build --features bin

dry-run: _validate
    cargo publish --all-features --dry-run

tag: _validate
    #!/usr/bin/env bash
    set -Eeou pipefail
    echo "Would tag v$(templar expression -i Cargo.toml '.[`package`][`version`]')"

publish: _validate
    #!/usr/bin/env bash
    set -Eeou pipefail
    cargo publish --all-features
