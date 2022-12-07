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

|              | Part 01                 | Part 02                         |
|:-------------|:--------------------------|:--------------------------------- |
| **Day 01** | 43.06 μs  | 42.94 μs (**1.00x faster**)   |
| **Day 02** | 257.00 μs | 250.77 μs (**1.02x faster**)  |
| **Day 03** | 207.98 μs | 288.11 μs (*1.39x slower*)    |
| **Day 04** | 110.04 μs | 104.25 μs (**1.06x faster**)  |
| **Day 05** | 115.64 μs | 116.19 μs (**1.00x slower**)  |
| **Day 06** | 16.39 μs  | 29.93 μs (*1.83x slower*)     |
| **Day 07** | 4.22 ms    | 4.22 ms (**1.00x slower**)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

