# crux

A competitive programming workspace tool for C++. Scaffold problem folders, manage test cases, and run tests against your solution — all from the command line.

## Commands

```
crux create <name|path>                Create a new problem workspace
crux run <name|path>                   Compile and run tests
crux add-test <name|path>              Add a new test case interactively
crux remove-test <name|path> <num>     Remove a test case by number
crux update-test <name|path> <num>     Update an existing test case
```

## Workspace Structure

Running `crux create two-sum` produces:

```
two-sum/
├── solution.cpp          ← write your function here
├── main.cpp              ← boilerplate: reads input, calls solution, writes output
├── tests/
│   └── 01.in
├── expected_results/
│   └── 01.out
├── test_results/
│   └── 01.out            ← actual output from last run
└── logs/
    └── 2026-06-17T14:32:00.txt
```

## Workflow

1. `crux create two-sum`
2. Write your function in `solution.cpp`
3. Wire up I/O in `main.cpp`
4. `crux add-test two-sum` to add test cases
5. `crux run two-sum` to test

## Test Output

```
[PASS] test 01  (12ms)
[FAIL] test 02  (8ms)
  --- expected
  +++ actual
  @@ -1 +1 @@
  -42
  +41

2/3 tests passed
```

Tests without a matching `expected_results/N.out` are compiled and run — actual output is written to `test_results/` but not diffed. Useful for inspecting output before committing to an expected result.

## `main.cpp` Boilerplate

```cpp
#include <bits/stdc++.h>
using namespace std;

// TODO: forward-declare your solution function
// e.g. int solve(int n, vector<int>& a);

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);

    // read input

    // call solution

    // write output

    return 0;
}
```

## Installation

```
cargo install crux
```

## Requirements

- Rust toolchain (`cargo`)
- `g++` (C++17 or later)
