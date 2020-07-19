target := 'x86_64-unknown-linux-gnu'

_validate-publish:
    #!/usr/bin/env bash
    set -Eeou pipefail
    if [[ $(git diff --stat) != '' ]]; then
    echo 'Working tree dirty, aborting'
    exit 1
    fi

install-cargo-addons:
    cargo install cargo-deb

build:
    cargo build --features bin

tag: _validate-publish
    echo 'no'

publish: _validate-publish
    echo 'no'
