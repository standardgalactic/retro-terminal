# Retro Terminal

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/standardgalactic/retro-terminal/releases)

Recreation of historical terminal interfaces, text consoles, and command-line interaction throughout the history of computing.

Current version: `0.1.0`

## Overview

Retro Terminal explores historical command-line environments as software archaeology. The project recreates terminals, shells, editing environments, communication protocols, and interaction models spanning the early decades of interactive computing.

## Current core capabilities

- Fixed-size screen buffer with row/column cursor tracking.
- Printable text rendering with line wrapping and vertical scrolling.
- Control handling: newline, carriage return, tab expansion, cross-line backspace, and clear screen.
- Command execution API for deterministic terminal state transitions including absolute/relative cursor movement and line clearing.
- Theme presets inspired by classic displays (amber, green phosphor, and IBM DOS).
- ANSI CSI support for common cursor movement and clear operations plus SGR style/color controls.
- Unit tests for wrapping, scrolling, cursor movement, control behavior, ANSI parsing, and style state.

## Development

This repository uses a standard make interface:

- make init
- make lint
- make test
- make benchmark
- make docs
- make format
- make release

## Running the interactive terminal demo

Run directly with Cargo:

```bash
cargo run
```

Or use the management script:

```bash
./scripts/manage.sh run
```

## Management script

The `scripts/manage.sh` helper wraps common development and release tasks:

- `./scripts/manage.sh clean` — remove build artifacts
- `./scripts/manage.sh build` — compile debug binary
- `./scripts/manage.sh run` — run the interactive terminal demo
- `./scripts/manage.sh release` — build optimized binary
- `./scripts/manage.sh version` — display current package version
- `./scripts/manage.sh bump` — bump patch version (`0.1.0 -> 0.1.1`)
- `./scripts/manage.sh minor` — bump minor version (`0.1.0 -> 0.2.0`)
- `./scripts/manage.sh major` — bump major version (`0.1.0 -> 1.0.0`)
- `./scripts/manage.sh lint` — run `make lint`
- `./scripts/manage.sh test` — run `make test`
- `./scripts/manage.sh format` — run `make format`
- `./scripts/manage.sh docs` — run `make docs`
