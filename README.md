# keydrill

![showcase](assets/showcase.gif)

A terminal-based keyboard layout trainer. Born from wanting to learn a new layout and not finding the right tool — so I built one. Heavily inspired by [colemakclub](https://github.com/gnusenpai/colemakclub).

## Features

- **Multi-layout support** — ships with Colemak and Colemak-DH
- **Progressive levels** — learn new keys gradually, from home row outward
- **Live keyboard visualization** — see active keys highlighted in real time
- **WPM & accuracy stats** — track your performance after each round
- **QWERTY remap toggle** — practice on a QWERTY OS setup without switching system layout
- **Custom layouts** — define your own layouts and levels via TOML files
- **Theming** — customize colors for text, keyboard, and word display
- **Animated effects** — rainbow cycling animation on active keys (Catppuccin Mocha palette by default)

## Installation

### Pre-built Binaries

Download the latest release for your platform from [GitHub Releases](https://github.com/jaeheonji/keydrill/releases/latest).

Available for Linux (x86_64), macOS (x86_64, aarch64).

### crates.io

```sh
cargo install keydrill
```

### Build from Source

Requires [Rust](https://www.rust-lang.org/tools/install) **1.85+** (edition 2024).

```sh
git clone https://github.com/jaeheonji/keydrill.git
cd keydrill
cargo install --path .
```

This installs the `keydrill` binary to `~/.cargo/bin/`. Make sure it's in your `PATH`.

## Usage

```sh
keydrill [--debug] [--config <path>]
```

| Flag | Description |
|------|-------------|
| `--debug` | Enable debug logging (writes to `/tmp/keydrill-<uid>/keydrill.log`) |
| `--config <path>` | Use a specific config file instead of the default location |

## Configuration

Config is loaded from `~/.config/keydrill/config.toml` (or `$XDG_CONFIG_HOME/keydrill/config.toml`).

All fields are optional — keydrill uses sensible defaults when no config file exists.

See [docs/configuration.md](docs/configuration.md) for the full reference.

## Custom Layouts

Place `.toml` layout files in `~/.config/keydrill/layouts/` or point to them via the config.

You can either reference a builtin layout and define custom levels, or define a full layout from scratch.

See [docs/custom-layouts.md](docs/custom-layouts.md) for details.

## License

[MIT](LICENSE)
