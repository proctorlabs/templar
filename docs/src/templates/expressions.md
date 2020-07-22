# Expressions

Everything inside the standard `{{ }}` block is an expression. Each block holds exactly one expression, but that expression can be chained with
many individual operations. A quick overview:

* Math operations: `+ - * / %` these operations are only valid with numeric types
* Equality: `== != < <= > >= && ||`
* Value setting: `=` the left side of this operation must be some identifier e.g. `{{ some.val.path = 'hello world!' }}`
* String concatenation: `~` e.g. `{{ 'Hello' ~ ' ' ~ 'world!' }}` prints "Hello world!"
* Functions: `ident()` e.g. `{{ env('USER') }}` would retrieve the value of the environment variable "USER".
* Filters: `|` e.g. `{{ 'hello world' | upper }}` would use the 'upper' filter to print "HELLO WORLD"

As documentation is still in progress, see the [expression tests](./src/test/expressions.rs) for examples of expression usage.
