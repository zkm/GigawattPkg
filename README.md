# GigawattPkg

GigawattPkg is a fast Rust wrapper that unifies package operations on Arch Linux and Fedora while keeping output colorful and scriptable.

## Current v0.1 scope

- Arch backends: pacman by default, optional paru
- Fedora backend: dnf
- Commands: search, list, install, remove, update
- Colorful output and JSON output mode
- Config file support at ~/.config/gigawattpkg/config.toml

## Build

```bash
cargo build
```

## Run

```bash
cargo run -p gigawattpkg -- search ripgrep
cargo run -p gigawattpkg -- list
cargo run -p gigawattpkg -- install ripgrep
cargo run -p gigawattpkg -- remove ripgrep
cargo run -p gigawattpkg -- update
```

The alias binary is also available:

```bash
cargo run -p gigawattpkg --bin gwpkg -- search git
```

## Backend selection

- Auto mode: distro detection decides backend
- Arch defaults to pacman
- Use paru explicitly with --use-paru
- Override backend manually with --backend pacman|paru|dnf

## JSON output mode

```bash
cargo run -p gigawattpkg -- search ripgrep --json
```

## Example config

Create ~/.config/gigawattpkg/config.toml:

```toml
prefer_paru = false
color = true

[theme]
primary = "cyan"
accent = "magenta"
warning = "yellow"
success = "green"
error = "red"
icon_search = ""
icon_install = ""
icon_remove = ""
icon_update = ""
```

## Notes

Mutating operations run through sudo so you can use the tool without launching the whole process as root.
