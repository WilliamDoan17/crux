# Progress

## v1

- [ ] `crux create`: scaffold folders and blank files
- [ ] `crux create`: write `main.cpp` boilerplate (`#include <bits/stdc++.h>`, empty `main()`)
- [ ] `crux add-test`: open `$EDITOR` (fallback to nano) for input, then expected output
- [ ] `crux remove-test`: delete `tests/N.in`, `expected_results/N.out`, `test_results/N.out`
- [ ] `crux run`: compile `main.cpp` into `bin/solution` once
- [ ] `crux run`: execute binary for each test via stdin/stdout redirection
- [ ] `crux run`: diff `test_results/N.out` against `expected_results/N.out` where available
- [ ] `crux run`: print formatted results (PASS / FAIL / RUN)
- [ ] `crux run`: write timestamped log to `logs/`
- [ ] `crux update-test`: open `$EDITOR` for input and/or expected output, blank keeps existing
