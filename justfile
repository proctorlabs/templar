target := `echo -n "${TARGET:-x86_64-unknown-linux-gnu}"`
build_dir := `echo -n $PWD/target/${TARGET:-x86_64-unknown-linux-gnu}/release`
package_dir := `echo -n $PWD/target/package`
cargo := `echo -n "${CARGO:-cargo}"`
bin_name := 'templar'

release: tag publish
default: build

_readme: setup-cargo

_validate:
    #!/usr/bin/env bash
    set -Eeou pipefail
    cd templar

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
    #!/usr/bin/env bash
    set -Eeou pipefail
    cd templar

    rustup toolchain install stable
    rustup target add '{{ target }}'

    # DOGFOODING
    cargo install templar --features bin

    # Other stuff
    cargo install cargo-deb
    cargo install cargo-readme
    cargo install cargo-strip
    cargo install mdbook

build:
    #!/usr/bin/env bash
    set -Eeou pipefail
    cd templar

    cargo build --features bin

changelog:
    #!/usr/bin/env bash
    set -Eeou pipefail
    git log --pretty=format:'%d %s' --no-merges | grep -E '(tag:|#chg)' | sed 's/.*#chg /- /g' | sed 's/ (tag:/\n## Release/g' | sed 's/) .*/\n/g'

run +args="":
    #!/usr/bin/env bash
    set -Eeou pipefail
    cd templar

    cargo run --features bin -- {{args}}

watch +args="":
    #!/usr/bin/env bash
    set -Eeou pipefail
    cd templar

    watchexec -w src just run -- {{args}}

build-release:
    #!/usr/bin/env bash
    set -Eeou pipefail
    cd templar

    echo 'Building for {{ target }}'
    {{cargo}} build --features bin --release --target '{{ target }}'

package-tar: build-release
    #!/usr/bin/env bash
    set -Eeou pipefail
    cd templar

    mkdir -p '{{ package_dir }}'
    cargo strip --target '{{ target }}' || true
    tar -C '{{ build_dir }}' -cvJf '{{ package_dir }}/{{ bin_name }}-{{ target }}.tar.xz' '{{ bin_name }}'

package-deb: build-release
    #!/usr/bin/env bash
    set -Eeou pipefail
    cd templar

    cp -f target/{{ target }}/release/templar target/release/templar
    cargo deb --no-build --no-strip -o "{{ package_dir }}/{{ bin_name }}-{{ target }}.deb"

book:
    #!/usr/bin/env bash
    set -Eeou pipefail
    cd templar

    mdbook build docs

serve-book:
    #!/usr/bin/env bash
    set -Eeou pipefail
    cd templar

    mdbook serve docs

package: package-tar package-deb

dry-run: _validate
    #!/usr/bin/env bash
    set -Eeou pipefail
    cd templar

    cargo publish --all-features --dry-run

tag: _validate
    #!/usr/bin/env bash
    set -Eeou pipefail
    git tag "v$(templar expression -i Cargo.toml '.[`package`][`version`]')"
    git push --tags

publish: _validate
    #!/usr/bin/env bash
    set -Eeou pipefail
    cd templar

    cargo publish --all-features
