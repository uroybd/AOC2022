# Advent of Code 2022 Solutions
---
**Language**: Rust

Every day has a dedicated file in the `solutions` directory, and two inputs (one the actual input, another the example suffixed with 'e') in the `inputs` directory.

Shamelessly stole and adapted [the scaffolding mechanism found here](https://github.com/fspoettel/advent-of-code-rust) to my need.


## Benchmarks

### Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Solutions](#solutions)

### Benchmark Results

#### Solutions

             `Part 01`                 `Part 02`                         |
|:-------------|:--------------------------|:--------------------------------- |
**`Day 01`** `42.89 us` (**1.00x**)  `43.42 us` (**1.01x slower**)   |
**`Day 02`** `259.37 us` (**1.00x**) `251.52 us` (**1.03x faster**)  |
**`Day 03`** `207.90 us` (**1.00x**) `283.56 us` (*1.36x slower*)    |
**`Day 04`** `110.05 us` (**1.00x**) `104.09 us` (**1.06x faster**)  |
**`Day 05`** `114.57 us` (**1.00x**) `120.04 us` (**1.05x slower**)  |
**`Day 06`** `16.38 us` (**1.00x**)  `29.80 us` (*1.82x slower*)     |
**`Day 07`** `4.27 ms` (**1.00x**)   `4.24 ms` (**1.01x faster**)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

