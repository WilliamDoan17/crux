# Architecture

## Overview

Crux is a single Rust binary (`crux`) with no framework. Argument parsing is manual; each subcommand is a module with a `run(args: &[String])` entry point.

## Source layout

```
src/
├── main.rs          — entry point; dispatches on argv[1]
├── commands.rs      — re-exports command submodules
└── commands/
    ├── create.rs    — crux create
    ├── add_test.rs  — crux add-test
    ├── remove_test.rs — crux remove-test
    ├── update_test.rs — crux update-test
    └── run.rs       — crux run
```

## Dispatch

`main.rs` collects `env::args()` and matches on the first argument, calling the corresponding `commands::<module>::run(&args[2..])`. Unknown commands and missing arguments print to stderr and exit 1.

## Command structure

Each command module exports one public function:

```rust
pub fn run(args: &[String])
```

Internal helpers are private to the module. `create.rs` is the most complete example: it validates input via `has_error()`, then delegates to `create_crux_workspace()`, which calls small focused helpers (`create_dir`, `create_file`, `write_to_file`).

## Error handling

All errors follow the same pattern: `eprintln!` to stderr, then `std::process::exit(1)`. No `Result` propagation — errors are terminal.

## Workspace layout

A crux workspace is a directory identified by the presence of a `.crux` marker file:

```
<name>/
├── .crux              — marker file (empty); identifies a crux workspace
├── main.cpp           — user's solution (boilerplate written on create)
├── tests/             — N.in files
├── expected_results/  — N.out files (optional)
├── test_results/      — actual output from runs
└── logs/              — timestamped run logs
```

## Dependencies

| Crate | Use |
|-------|-----|
| `edit` | Opens `$EDITOR` (fallback: nano) for interactive test input/output |

## Implementation status

| Command | Status |
|---------|--------|
| `create` | Complete |
| `add-test` | Stub |
| `remove-test` | Stub |
| `update-test` | Stub |
| `run` | Stub |
