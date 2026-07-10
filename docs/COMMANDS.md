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
- If the solution binary fails to launch, or exits having written to stderr: reports ERROR

If the solution binary fails to launch, or exits having written to stderr, the failure is written to `test_results/N.out` as:

```
Error:
<error message>
```

instead of the solution's stdout, so it's distinguishable from legitimate program output.

Output format:

```
[PASS]  test 1  (12ms)
[FAIL]  test 2  (8ms)
  --- expected
  +++ actual
  @@ -1 +1 @@
  -42
  +41
[RUN]   test 3  (5ms)
[ERROR] test 4  (3ms)
  Error:
  <error message>

2/4 tests passed
```

Writes a timestamped log to `logs/` with the full run output.
