# Commands

## `crux create <name|path>`

Scaffolds a new problem workspace.

```
two-sum/
├── .crux
├── main.cpp
├── tests/
├── expected_results/
├── test_results/
└── logs/
```

- `.crux` — empty marker file that identifies the folder as a crux-managed workspace
- `main.cpp` — boilerplate with `#include <bits/stdc++.h>` and an empty `main()`; user writes everything here
- Directories are created empty; test files are added via `crux add-test`

> `solution.cpp` is not used in v1. It will be introduced alongside `crux build` and `IN_OUT_FORMAT.md`.

---

## `crux add-test <name|path>`

Interactively adds a new test case to crux workspace name|path specified.

- Prompts for input, then expected output (optional)
- input and output uses `$EDITOR`, with a fallback to nano to write
- Input is written to `<name|path>/tests/N.in`
- Expected output (if provided) is written to `<name|path>/expected_results/N.out`
- Test number `N` is the next available integer, unpadded (e.g. `1`, `2`, ..., `10`)
- If no expected output is given, the test still runs but is not diffed

---

## `crux remove-test <name|path> <num>`

Removes a test case by number.

- Deletes `tests/N.in`, `expected_results/N.out`, and `test_results/N.out` if they exist
- Does not renumber remaining tests

---

## `crux update-test <name|path> <num>`

Interactively updates an existing test case.

- Uses `$EDITOR` with a fallback to nano
- Prompts to replace input and/or expected output
- Leaving a prompt blank keeps the existing value

---

## `crux run <name|path>`

Compiles and runs the solution against all test cases.

Compile: `g++ -std=c++17 -O2 -Wall -o <name>/bin/solution <name>/main.cpp`

For each test:

- Runs the binary with `tests/N.in` as stdin
- Writes actual output to `test_results/N.out`
- If `expected_results/N.out` exists: diffs and reports PASS or FAIL
- If no expected output: reports RUN (output written, not diffed)
- If the solution binary launches but exits having written to stderr: reports ERROR
- If the test couldn't run at all (test input file missing/unreadable, or the solution binary couldn't be launched): reports ERROR with no elapsed time

If the solution binary launches but exits having written to stderr, the failure is written to `test_results/N.out` as:

```
Error:
<error message>
```

instead of the solution's stdout, so it's distinguishable from legitimate program output. This case has a measured elapsed time, since the binary did run.

If the test couldn't run at all (e.g. `tests/N.in` is missing, or `bin/solution` failed to launch), no `test_results/N.out` is written and no elapsed time is recorded — this is reported as `(n/a)` instead of a millisecond count.

Output format:

```
[PASS]  test 1  (12ms)
[FAIL]  test 2  (8ms)
--- expected
+++ result
  1 -42
  1 +41
[RUN]   test 3  (5ms)
[ERROR] test 4  (3ms)
  Error:
  <error message>
[ERROR] test 5  (n/a)

2/4 tests passed
```

For a FAIL, each differing line is reported as a `<line number> -<expected>` / `<line number> +<actual>` pair, one pair per differing line (1-indexed, no surrounding context). Since comparison is index-matched (no realignment for insertions/deletions), the line number is always the same for both sides of a pair.

Writes a timestamped log to `logs/` with the full run output. Log filename format: `YYYY-MM-DD_HH-MM-SS.log` (e.g. `2026-07-11_14-30-05.log`).
