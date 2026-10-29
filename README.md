# GigawattPkg

[![CI](https://github.com/zkm/GigawattPkg/actions/workflows/ci.yml/badge.svg)](https://github.com/zkm/GigawattPkg/actions/workflows/ci.yml)
[![Release](https://github.com/zkm/GigawattPkg/actions/workflows/release.yml/badge.svg)](https://github.com/zkm/GigawattPkg/actions/workflows/release.yml)
[![Latest release](https://img.shields.io/github/v/release/zkm/GigawattPkg)](https://github.com/zkm/GigawattPkg/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

![Rust 2024 edition](https://img.shields.io/badge/Rust-2024%20edition-000000?logo=rust&logoColor=white)
![clap](https://img.shields.io/badge/clap-5A5A5A)
![rustfmt and clippy enforced](https://img.shields.io/badge/rustfmt%20%2B%20clippy-enforced-DEA584)
![Arch Linux pacman](https://img.shields.io/badge/Arch%20Linux-pacman-1793D1?logo=archlinux&logoColor=white)
![Fedora dnf](https://img.shields.io/badge/Fedora-dnf-51A2DA?logo=fedora&logoColor=white)

GigawattPkg is a fast Rust wrapper that unifies package operations on Arch Linux and Fedora while keeping output colorful and scriptable.

## Current v0.1 scope

- Arch backend: pacman
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
- Arch uses pacman
- Override backend manually with `--backend pacman|dnf`

## JSON output mode

```bash
cargo run -p gigawattpkg -- search ripgrep --json
```

## Example config

Create ~/.config/gigawattpkg/config.toml:

```toml
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

## Development workflow

- Pull requests to `main` are validated by CI (format, clippy, tests, release build).
- We follow a lightweight GitHub Flow; see [CONTRIBUTING.md](CONTRIBUTING.md) for branch naming, PR expectations, and local checks.

## Releases

- Releases are created from pushed tags matching `v*` (for example, `v0.1.1`).
- The Release workflow publishes a GitHub Release with packaged Linux binaries (`gigawattpkg`, `gwpkg`) and a SHA-256 checksum file.
