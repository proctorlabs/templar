# Templar CLI

The CLI can be used to run expressions or execute templates to STDOUT or an output file.

## Usage

Tu process a template, `templar template <args>`

```bash
templar-template 0.1.0
Execute a template and render the output

USAGE:
    templar template <file>

OPTIONS:
    -h, --help                Prints help information
    -i, --input <input>...    File to parse and load into the templating context
    -o, --output <output>     Output to send the result to, defaults to stdout
    -s, --set <set>...        Directly set a variable on the context

ARGS:
    <file>    Template file(s) to open
```

To run an expression directly, `templar expression <args>`

```bash
templar-expression 0.1.0
Execute an expression and render the output

USAGE:
    templar expression <text>

OPTIONS:
    -h, --help                Prints help information
    -i, --input <input>...    File to parse and load into the templating context
    -o, --output <output>     Output to send the result to, defaults to stdout
    -s, --set <set>...        Directly set a variable on the context

ARGS:
    <text>    The expression to run
```
