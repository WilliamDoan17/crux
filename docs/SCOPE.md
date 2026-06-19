# Scope

## v1

Users can:

- Create a problem folder
- Add, remove, and update test cases interactively
- Run tests and see results with diffs and logs

What the package does:

- Scaffolds a problem folder with a static `main.cpp` boilerplate and `solution.cpp`
- Creates and manages test input/output files
- Compiles and runs the solution against test cases
- Writes logs at each run:
  - How many tests passed
  - Per-test result: expected, actual, diff
- Tests without a matching expected output are run and written to `test_results/` but not diffed

## Future

- `IN_OUT_FORMAT.md` DSL in each problem folder describing input/output layout
  - Specifies line breaks, spaces, and other delimiters
  - Handles multi-test-case and grid input patterns
- `crux build` command: parses `IN_OUT_FORMAT.md` and generates `main.cpp` accordingly
- AI-assisted boilerplate generation from the format file
