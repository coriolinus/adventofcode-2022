# Advent of Code 2022

Solutions to the exercises at <https://adventofcode.com/2022/>.

Each day is a subproject within a shared workspace; this reduces recompilation required.

Uses [`aoctool`](https://github.com/coriolinus/aoctool) for daily setup, and
[`aoclib`](https://github.com/coriolinus/aoclib/) for shared library functions.

## Running a Day

As each day is a sub-crate built from a template, they all have a similar CLI interface. Each is smart
enough to download its input if it does not exist, but use the existing file if it does. Just run the
desired day by name; it'll run part 1. Part 2 can be run by adding a CLI flag.

```bash
cargo run -p day01 -- --part2
```
