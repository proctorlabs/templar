# Installation

We try to supply easy install methods for different platforms on a best-effort basis.

## Binary Install

The primary way Templar is distributed is in binary form from [Github](https://github.com/proctorlabs/templar/releases).
Binaries are available for a number of platforms and can be quickly installed via script. A simple script install can
be like this:

```bash
# Make sure to pick the architecture and variant for your platform
curl -sL https://github.com/proctorlabs/templar/releases/download/v0.3.0/templar-x86_64-unknown-linux-gnu.tar.xz |
    tar -xJ -C /usr/local/bin && chmod +x /usr/local/bin/templar
```

## Cargo (source)

If Cargo is available, templar can be quickly installed with this command:

```bash
cargo install --all-features templar
```
