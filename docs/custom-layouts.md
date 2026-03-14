# Custom Layouts

keydrill supports custom keyboard layouts defined as TOML files.

## Layout Discovery

Layouts are loaded in order:

1. **Builtins** — `Colemak`, `Colemak-DH`
2. **XDG directory** — `~/.config/keydrill/layouts/*.toml`
3. **Config paths** — files or directories listed in `[general] layouts`

## Two Approaches

### 1. Reference a builtin

Inherit keys from a builtin layout and define your own levels:

```toml
name = "Colemak (Custom Levels)"
builtin = "Colemak"

[[levels]]
name = "Home Row Basics"
new_keys = ["a", "r", "s", "t"]

[[levels]]
name = "Home Row Extended"
new_keys = ["n", "e", "i", "o"]
```

No `[[keys]]` section needed — keys are copied from the builtin. Available builtins: `Colemak`, `Colemak-DH` (case-insensitive).

### 2. Full key definition

Define every key position explicitly:

```toml
name = "My Layout"

[[keys]]
row = 0
col = 0
normal = "`"

[[keys]]
row = 1
col = 0
normal = "q"

# ... define all keys

[[levels]]
name = "Home Row"
new_keys = ["a", "s", "d", "f"]
```

Each key requires:

| Field | Type | Description |
|-------|------|-------------|
| `row` | integer | Row index: 0 = number row, 1 = top, 2 = home, 3 = bottom |
| `col` | integer | Column index within the row |
| `normal` | char | The character at this position |

## Levels

Each layout must have at least one level. Levels are played in order — each one introduces new keys on top of previous levels.

```toml
[[levels]]
name = "Home Row"
new_keys = ["a", "r", "s", "t"]
words = ["rest", "star", "rats", "arts"]
word_count = 30
random_words = true
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | string | *required* | Display name for the level |
| `new_keys` | list of chars | *required* | Keys introduced in this level |
| `words` | list of strings | `[]` | Custom word list for practice |
| `word_count` | integer | `30` | Number of words per round |
| `random_words` | bool | `true` | Generate random words from available keys to fill the pool |

### Word pool behavior

- **`random_words = true`** (default): Uses your `words` list (if any) plus randomly generated words from all keys available at this level, filling to `word_count`.
- **`random_words = false`**: Uses cumulative words from all levels up to the current one. No random generation.

## Examples

See [`layouts/colemak-custom.toml`](layouts/colemak-custom.toml) for a minimal example using builtin inheritance, and [`layouts/colemak-dh.toml`](layouts/colemak-dh.toml) for a full layout definition.
