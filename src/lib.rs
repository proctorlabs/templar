/*!
Templar is both a Rust library and a CLI tool for working with templates. The usage and style is
inspired by both Jinja2 and Ansible, though it is not intended to be a clone of either of these.

The goal of the project is to provide fast and flexible dynamic templating, particularly for use with
configurations and local tooling. Despite this, it likely can be adapted for HTML and front end rendering
as well.

# Examples

The templating syntax is likely familiar considering the frameworks that it is based on. For instance, a
simple template may look like this:

```properties
user_name={{ user.name }} {# Replace with the context property 'name' in 'user' #}
full_context={{ . | json("pretty") }} {# Dump the entire context as JSON, '.' is the root node #}
password={{ script('echo hunter2 | md5sum') }} {# Execute a shell command and calculate the MD5 sum #}
```

In addition to simple replacements, more complex expressions can be used.

```markdown
The calculated result is {{ 100 * 5 / 10 }} {#- Prints '50' #}

Today's guest list:
{%- for person in ['Bob', 'Joe', 'Jen', 'Amy')] %}
* {{ person }} will come to the party!
{%- endif %} {#- This will loop everyone in the inline array above, but they array could also come from the context #}
```

# Another templating framework?

Well... yes.

There are many great templating frameworks out there, however they are mostly intended for web or HTML rendering. This leads
to a few drawbacks when used for other purposes.

* Templar has first class support for parsed configuration files. You can create a context directly from a config that is parsed with
  serde or, alternatively, use it for templating serde sub-elements.
* You can opt to parse expressions directly instead of an entire template.
* Context values are lazily processed on access.
* Context values can refer to other context values.
* Extending the base functionality is easy.
* Support for dynamic context nodes that are recalculated on every access. e.g. repeated calls to a template with this content
  `{% if user.isRoot %} {{ do_something() }} {% end if %}` would change if the `user.isRoot` value changes.

# Template Syntax

Much of the syntax is based on the wonderful [Jinja2](https://jinja.palletsprojects.com/en/2.10.x/) project. Here are some
of the currently supported features.

* Value replacement can be done using the `{{ }}` syntax.
    * Literals supported are strings (single, double, or backtick quoted), boolean, numbers (currently parsed as i64), null, arrays, and maps
    * Identifiers that start with an alphabetic character can be referred to directly e.g. `{{ some.value.path }}`
    * The root node can be referred to with `.` allowing things like `{{ . | json }}` to be used to dump the entire context as JSON
    * Identifiers of non-standard type, e.g. starting with a non-alphabetic character, spaces, etc. can be referred to using the
      bracket syntax. e.g. `{{ .['565'] }}`. This also allows array access and identifier of non-standard types (such as boolean).
    * Inline arrays: `{{ [1,2,3,4] }}` and complex nesting also possible e.g. `{{ [1,2, script("echo 'hello world!'"), (5 + 5 | base64)] }}
    * Inline maps: `{{ {'key': 'value', 'otherKey': { 'nested': 'map' } } }}`
* Control flow can be done using the `{% %}` syntax
    * If/else if: `{% if 10/2 == 5 %}The world is sane!{% else if false %}What universe are we in?{% end if %}`
    * Scoping can be done manually: `{% scope %}I'm in a scope!{% end scope %}`
    * For loops: `{% for thing in lots.of.stuff %} {{ thing['name'] }} {% end for %}`. For loops always enter a new scope.
* Comments use the `{# #}` syntax and will be remitted from the output.
* Whitespace control can be accomplished by adding a `-` to any of the above blocks e.g. `{{- 'no whitespace! -}}`.
    * Whitespace control can be added to one or both sides of the tags. All spaces, new lines, or other whitespace on the side with the `-`
      on it will be removed as if the block is immediately next to the other element.

As documentation is still in progress, see the [kitchen sink](./examples/kitchen_sink.tmpl) for examples of template usage.

# Expression syntax

Everything inside the standard `{{ }}` block is an expression. Each block holds exactly one expression, but that expression can be chained with
many individual operations. A quick overview:

* Math operations: `+ - * / %` these operations are only valid with numeric types
* Conditionals: `== != < <= > >= && ||`
* Value setting: `=` the left side of this operation must be some identifier e.g. `{{ some.val.path = 'hello world!' }}`
* String concatenation: `~` e.g. `{{ 'Hello' ~ ' ' ~ 'world!' }}` prints "Hello world!"
* Functions: `ident()` e.g. `{{ env('USER') }}` would retrieve the value of the environment variable "USER".
* Filters: `|` e.g. `{{ 'hello world' | upper }}` would use the 'upper' filter to print "HELLO WORLD"

As documentation is still in progress, see the [expression tests](./src/test/expressions.rs) for examples of expression usage.

# Performance

Templar prefers rendering performance over parsing performance. While you should take most benchmarks with a grain of salt, simple templates
render in a few microseconds. On my AMD Ryzen 2700U processor, I can render a simple template about 300,000 times a second on a single thread.

Templates vary a lot though and templates that call out to shell commands or do other complex things will get less performance.

# API

Full API documentation can be found on [docs.rs](https://docs.rs/templar/)
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
        error::TemplarError,
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
