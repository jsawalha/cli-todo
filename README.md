# cli-todo

A command-line todo list application built with Rust and SQLite.

## Demo

[![asciicast](https://asciinema.org/a/900394.svg)](https://asciinema.org/a/900394)

## Features

- Add todos with a title and description
- List all todos in a formatted table
- Toggle todos as complete/incomplete
- Delete individual todos or wipe the entire list

## Dependencies

- [rusqlite](https://github.com/rusqlite/rusqlite) — SQLite database interface
- [comfy-table](https://github.com/Nukesor/comfy-table) — formatted terminal tables

## Build

```bash
cargo build --release
```

The binary will be at `target/release/cli-todo`.

## Usage

```bash
cli-todo add <title> <description>   # Add a new todo
cli-todo list                        # List all todos
cli-todo toggle <id>                 # Toggle completed status
cli-todo delete <id>                 # Delete a todo by ID
cli-todo erase                       # Delete all todos
cli-todo help                        # Show available commands
```

## Running Tests

```bash
cargo test
```

Tests use an in-memory SQLite database so nothing is written to disk.
