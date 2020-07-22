# Templating

Templar's templating syntax is inspired by Jinja2 and Ansible.

## Examples

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
{%- endfor %} {#- This will loop everyone in the inline array above, but they array could also come from the context #}
```
