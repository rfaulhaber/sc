# sc

A terminal-based stack calculator, written in Rust.

## Installing

Currently you must manually build this project using `cargo`.
You can do this by running:

```
cargo install --git https://github.com/rfaulhaber/sc
```

## Use

While the program is running, add numbers or the result of expressions to the stack.
The program will echo the result.

For example, if I add `1`, `2`, and `1 2 +`, the output would be:

```
1
1
2
2
1 2 +
3
```