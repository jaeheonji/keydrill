# Configuration

keydrill is configured via a TOML file at:

- **Linux/macOS:** `~/.config/keydrill/config.toml` (or `$XDG_CONFIG_HOME/keydrill/config.toml`)
- **Windows:** `%APPDATA%/keydrill/config.toml`

You can override the path with `--config <path>`.

All fields are optional. See [`config.toml`](config.toml) for a full annotated example.

## `[general]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `qwerty_remap` | bool | `false` | Remap QWERTY physical key positions to the selected layout. Useful if your OS is set to QWERTY but you want to practice another layout. |
| `layouts` | list of strings | `[]` | Extra layout files or directories to load. Paths can be absolute or relative. Directories are scanned for `.toml` files. |

```toml
[general]
qwerty_remap = false
layouts = ["~/my-layouts/custom.toml", "~/my-layouts/extras/"]
```

## `[theme]`

Controls the overall UI colors.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `text` | string | `"reset"` | Default text (level select items, result values) |
| `primary` | string | `"green"` | Key hints in help text, remap ON indicator |
| `secondary` | string | `"dark_gray"` | Help text, labels, dimmed elements |
| `highlight` | string | `"blue"` | Title art, selected items |

## `[theme.word]`

Colors for the typing area.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `current` | string | `"reset"` | Untyped characters in the current word |
| `correct` | string | `"green"` | Correctly typed characters |
| `incorrect` | string | `"red"` | Incorrectly typed characters |
| `queue` | string | `"dark_gray"` | Upcoming words preview |

## `[theme.keyboard.active]` / `[theme.keyboard.inactive]`

Colors for the keyboard visualization. `active` applies to keys in the current level, `inactive` to all other keys.

| Field | Type | Default (active) | Default (inactive) |
|-------|------|-------------------|---------------------|
| `text` | string | `"white"` | `"dark_gray"` |
| `border` | string | `"white"` | `"dark_gray"` |

## `[effect]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | bool | `true` | Enable rainbow cycling animation on active keys |
| `cycle_colors` | list of strings | `[]` | Custom colors for the cycling effect (hex RGB only). Expanded to 8 colors via HSL interpolation. Empty uses Catppuccin Mocha defaults. |

```toml
[effect]
enabled = true
cycle_colors = ["#ff0000", "#00ff00", "#0000ff"]
```

## Color Formats

All color fields accept:

| Format | Example | Description |
|--------|---------|-------------|
| Named | `"red"`, `"dark_gray"`, `"light_cyan"` | Standard terminal color names |
| 256-index | `"0"` through `"255"` | 256-color palette index |
| Hex RGB | `"#ff0000"` | 24-bit RGB color |
| Reset | `"reset"` | Terminal default color |

Available named colors: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, `gray`, `dark_gray`, `light_red`, `light_green`, `light_yellow`, `light_blue`, `light_magenta`, `light_cyan`.
