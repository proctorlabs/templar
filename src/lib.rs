/*!
Templar is both a library and a CLI tool for working with templates. The usage and style is
inspired by both Jinja2 and Ansible, though it is not intended to be a clone of these tools.

# Templates

```properties
something={{ context.value | base64}}
full_context={{ context | json }} {# Need single line here, but json('pretty') will provide indentation #}
password={{ shell('echo hunter2 | md5sum') }}
```
*/

//#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;

pub(crate) use error::*;
use std::{collections::HashMap, sync::Arc};

pub(crate) use execution::*;

pub use {
    self::{
        context::Context,
        execution::Data,
        extensions::{Filter, Function, GenericFilter, GenericFunction, TemplarResult},
        templar::*,
    },
    unstructured::Document,
};

pub mod error;

#[cfg(test)]
mod test;

mod context;
mod execution;
mod extensions;
mod parser;
mod templar;
