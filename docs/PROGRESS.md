# Progress

## Phase 0: Setup

- [x] Create `src/commands/` with all command files
- [x] Create `src/commands.rs` and declare all submodules
- [x] Stub each command with `pub fn run(args: &[String])`
- [x] Wire `main.rs` dispatcher
- [x] `cargo build` passes
- [x] Smoke test routing

## Phase 1: v1

- [x] `crux create`: scaffold folders and blank files
- [x] `crux create`: write `main.cpp` boilerplate (`#include <bits/stdc++.h>`, empty `main()`)
- [x] `crux add-test`: open `$EDITOR` (fallback to nano) for input, then expected output
- [x] `crux remove-test`: delete `tests/N.in`, `expected_results/N.out`, `test_results/N.out`
- [ ] `crux run`: compile `main.cpp` into `bin/solution` once
- [ ] `crux run`: execute binary for each test via stdin/stdout redirection
- [ ] `crux run`: diff `test_results/N.out` against `expected_results/N.out` where available
- [ ] `crux run`: print formatted results (PASS / FAIL / RUN)
- [ ] `crux run`: write timestamped log to `logs/`
- [ ] `crux update-test`: open `$EDITOR` for input and/or expected output, blank keeps existing
