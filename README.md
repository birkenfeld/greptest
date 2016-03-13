# greptest

Small benchmark program for Rust bytes-regex vs PCRE in a grep-like setting
(matching against individual lines from a file).

## Building

Build features:

* `pcre` - use PCRE, not regex
* `jit` - use PCRE JIT (only matters when feature `pcre` active)

## Running

Run as `greptest pattern filename`.

It is recommended to use a large file, e.g. one created by concatenating all
code files from a Rust checkout, or a similarly large project, to see
significant differences.
