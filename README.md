# Ren

`ren` is a command-line utility that takes [`find`](https://en.wikipedia.org/wiki/Find_(Unix))-formatted lines via standard input, and batch renames them. By default, it outputs a [`diff`](https://en.wikipedia.org/wiki/Diff)-preview of the renamed files and directories to standard output, and with a flag it can rename the files and directories.

[![Rename with `ren`](ren.gif)](https://youtu.be/d-UhiHyWnGQ)

## Example

Output a diff to standard output showing the result of finding all the files containing `foo` in their name, and replacing `foo` to `bar` in their names:

```
find . -name '*foo*' | ren foo bar
```

Add the `-w` (`--write`) flag to rename the files:

```
find . -name '*foo*' | ren foo bar
```

## Installation

`ren` is available via [`cargo`](https://github.com/rust-lang/cargo):

```
cargo install ren-find
```
## Configuration

The default pager is `less`, the `REN_PAGER` environment variable can be used to override the pager (e.g., `export REN_PAGER=delta` in Bash).

## Help

`ren -h` (or `ren --help`, the full `--help` provides slightly longer explanations of some option) will list help for all the command-line flags.

## Acknowledgements

- Much of the functionality, and the overall structure of the source code, was borrowed [`sd`](https://github.com/chmln/sd). `ren` began as a fork of `sd`.
- The code for specifying a custom pager for `ren` was borrowed from [`delta`](https://github.com/dandavison/delta).

