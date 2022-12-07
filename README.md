# Advent of Code 2022 Solutions
---
**Language**: Rust

Every day has a dedicated file in the `solutions` directory, and two inputs (one the actual input, another the example suffixed with 'e') in the `inputs` directory.

Shamelessly stole and adapted [the scaffolding mechanism found here](https://github.com/fspoettel/advent-of-code-rust) to my need.

## Benchmarks

### Table of Contents

- [Advent of Code 2022 Solutions](#advent-of-code-2022-solutions)
  - [Benchmarks](#benchmarks)
    - [Table of Contents](#table-of-contents)
    - [Benchmark Results](#benchmark-results)
      - [Solutions](#solutions)

### Benchmark Results

#### Solutions

|              | `Part 01`                 | `Part 02`                         |
|:-------------|:--------------------------|:--------------------------------- |
| **`Day 01`** | `42.77 us` (✅ **1.00x**)  | `42.80 us` (✅ **1.00x slower**)   |
| **`Day 02`** | `260.43 us` (✅ **1.00x**) | `252.10 us` (✅ **1.03x faster**)  |
| **`Day 03`** | `172.40 us` (✅ **1.00x**) | `241.60 us` (❌ *1.40x slower*)    |
| **`Day 04`** | `110.45 us` (✅ **1.00x**) | `103.85 us` (✅ **1.06x faster**)  |
| **`Day 05`** | `117.01 us` (✅ **1.00x**) | `116.62 us` (✅ **1.00x faster**)  |
| **`Day 06`** | `16.32 us` (✅ **1.00x**)  | `29.44 us` (❌ *1.80x slower*)     |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

