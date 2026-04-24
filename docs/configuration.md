# Configuration

!!! note "Fill this in"

    Document each configuration option your project exposes. The structure below
    is a starting point — adapt it to your actual config surface.

## Overview

ranger can be configured via:

1. A config file (`ranger.toml` or `.ranger.toml` in the project root)
1. Environment variables prefixed with `{{ranger | upcase}}_`
1. Command-line flags (highest precedence)

## Options

### `option_name`

**Type:** `string`
**Default:** `"default_value"`
**Environment variable:** `{{ranger | upcase}}_OPTION_NAME`

Description of what this option controls and when you would change it.

```toml
# ranger.toml
option_name = "custom_value"
```

______________________________________________________________________

### `another_option`

**Type:** `bool`
**Default:** `false`
**Environment variable:** `{{ranger | upcase}}_ANOTHER_OPTION`

Description of what this option controls.

```toml
another_option = true
```

## Example config file

```toml
# ranger.toml
option_name = "custom_value"
another_option = true
```
