# Syntax

Much of the syntax is based on the wonderful [Jinja2](https://jinja.palletsprojects.com/en/2.10.x/) project. Here are some
of the currently supported features.

* Value replacement can be done using the `{{ }}` syntax.
    * Literals supported are strings (single, double, or backtick quoted), boolean, numbers (currently parsed as i64), null, arrays, and maps
    * Identifiers that start with an alphabetic character can be referred to directly e.g. `{{ some.value.path }}`
    * The root node can be referred to with `.` allowing things like `{{ . | json }}` to be used to dump the entire context as JSON
    * Identifiers of non-standard type, e.g. starting with a non-alphabetic character, spaces, etc. can be referred to using the
      bracket syntax. e.g. `{{ .['565'] }}`. This also allows array access and identifier of non-standard types (such as boolean).
    * Inline arrays: `{{ [1,2,3,4] }}` and complex nesting also possible e.g. `{{ [1,2, script("echo 'hello world!'"), (5 + 5 | base64)] }}`
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
