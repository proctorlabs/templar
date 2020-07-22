# Templar functions

Functions are used for pulling or creating data from other sources or using other methods.

## Overview

As an example, the file() function will open a file and return the contents as a string. Functions do not require
any data to work, though most need arguments. Functions can be used as an argument to other functions,
to filters, or as the base operation. For example, this is a valid expression:

```template
{ 'filename': 'settings.json', 'content': json(file('settings.json')) } | yml
```

The above will creat a map with two fields "filename" and "content" with content containing the parsed
contents the file `settings.json`. Then we pass this map to the filter `yml` to then render
that map into a serialized YML string.

## Built in functions

- file(str): Open file and read contents to a string
- env(str): Read the named environment variable
- script(str): Execute the string as a shell script. Returns a map with keys "stdout", "stderr", "status"
- command(str, str[]?): Execute the supplied command with the supplied arguments. Returns a map with keys "stdout", "stderr", "status"
- json(str): Parse the supplied JSON string into a map. Requires "json-extension" feature (default on)
- yaml(str): (alias yml) Parse the supplied YML string into a map. Requires "yaml-extension" feature (default on)
