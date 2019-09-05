[![Crate](https://img.shields.io/crates/v/templar.svg)](https://crates.io/crates/templar)
[![Documentation](https://img.shields.io/badge/docs-current-important.svg)](https://docs.rs/templar/)
[![MIT License](https://img.shields.io/github/license/proctorlabs/templar.svg)](LICENSE)

# Templar

Templar is both a library and a CLI tool for working with templates. The usage and style is
inspired by both Jinja2 and Ansible, though it is not intended to be a clone of these tools.

## Templates

```properties
something={{ context.value | base64}}
full_context={{ context | json }} {# Need single line here, but json('pretty') will provide indentation #}
password={{ shell('echo hunter2 | md5sum') }}
```

