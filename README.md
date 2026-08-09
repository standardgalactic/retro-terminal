# Retro Terminal

Recreation of historical terminal interfaces, text consoles, and command-line interaction throughout the history of computing.

## Overview

Retro Terminal explores historical command-line environments as software archaeology. The project recreates terminals, shells, editing environments, communication protocols, and interaction models spanning the early decades of interactive computing.

## Current core capabilities

- Fixed-size screen buffer with row/column cursor tracking.
- Printable text rendering with line wrapping and vertical scrolling.
- Control handling: newline, carriage return, tab expansion, cross-line backspace, and clear screen.
- Command execution API for deterministic terminal state transitions including absolute/relative cursor movement and line clearing.
- ANSI CSI support for common cursor movement and clear operations.
- Unit tests for wrapping, scrolling, cursor movement, control behavior, and ANSI parsing.

## Development

This repository uses a standard make interface:

- make init
- make lint
- make test
- make benchmark
- make docs
- make format
- make release
