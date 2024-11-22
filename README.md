# A `libasm` tester in Rust

This is a tester for the `libasm` project of school 42.
It benefits from the speed of Rust's testing framework, allowing to launch all tests in under a second.

## Setup (for correctors)

1. Download the release of this tester inside the corrected's libasm project.
2. Add this in the Makefile of the corrected (of course, change the `${OBJS}` if needed):
```Makefile
.PHONY: dynamic_lib
dynamic_lib:
    cc -shared -o libasm.o ${OBJS}
```
3. Run `make dynamic_lib`. If it fails, you can lecture them about how it's important to create [Position-Independent Code](https://en.wikipedia.org/wiki/Position-independent_code).
    - If you feel generous enough, add `wrt ..plt` after every `call [function]` instruction they use, and proceed
4. Run `./libasm_tester`. If there's a lot of red, it might be because the person you correct didn't do the bonus, in which case you can run `./libasm_tester mandatory`.
5. To run valgrind (because you definitely want to run valgrind), use this command:
```sh
valgrind --leak-check=full --trace-children=yes --show-leak-kinds=all --suppressions=./valgrind_suppression_files/patch_rusty_and_cargo_test.txt ./libasm_tester mandatory
```