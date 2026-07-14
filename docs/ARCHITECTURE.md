# Architecture

## Overview

Crux is a single Rust binary (`crux`) with no framework. Argument parsing is manual; each subcommand is a module with a `run(args: &[String])` entry point.

## Source layout

```
src/
├── main.rs          — entry point; dispatches on argv[1]
├── commands.rs      — re-exports command submodules
├── workspace.rs     — shared filesystem helpers
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

Internal helpers are private to the module. Commands import shared filesystem utilities from `crate::workspace`.

## Shared filesystem helpers (`workspace.rs`)

Filesystem operations used across commands live in `src/workspace.rs`:

| Function                    | Description                                                 |
| --------------------------- | ----------------------------------------------------------- |
| `create_dir(path)`          | Creates a directory; exits on error                         |
| `create_file(path)`         | Creates an empty file; exits on error                       |
| `write_file(path, content)` | Creates/truncates a file and writes content; exits on error |

## Error handling

All errors follow the same pattern: `eprintln!` to stderr, then `std::process::exit(1)`. No `Result` propagation — errors are terminal.

## Workspace layout

A crux workspace is a directory identified by the presence of a `.crux` marker file:

```
<name>/
├── .crux              — marker file (empty); identifies a crux workspace
├── main.cpp           — user's solution (boilerplate written on create)
├── bin/               — created empty on create; holds the compiled binary
├── tests/             — N.in files
├── expected_results/  — N.out files (optional)
├── test_results/      — actual output from runs
└── logs/              — timestamped run logs
```

`bin/` is scaffolded empty by `crux create`, alongside `tests/`, `expected_results/`, `test_results/`, and `logs/`. `crux run` compiles into `bin/solution`. The binary is always named `solution` regardless of which source file produced it (`main.cpp` today, `solution.cpp` after `crux build` lands), so the name stays stable as the source model evolves.

Test numbers (`N`) are `i16`, supporting up to 32,767 test cases per workspace.

Log files in `logs/` are named `YYYY-MM-DD_HH-MM-SS.log` (e.g. `2026-07-11_14-30-05.log`).

## Dependencies

| Crate  | Use                                                                |
| ------ | ------------------------------------------------------------------ |
| `edit` | Opens `$EDITOR` (fallback: nano) for interactive test input/output |

## Implementation status

| Command       | Status   |
| ------------- | -------- |
| `create`      | Complete |
| `add-test`    | Complete |
| `remove-test` | Stub     |
| `update-test` | Stub     |
| `run`         | Complete |
