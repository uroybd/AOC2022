use aoc_2022::solutions::day01::{solution_day_01_01, solution_day_01_02};
use aoc_2022::solutions::day02::{solution_day_02_01, solution_day_02_02};
use aoc_2022::solutions::day03::{solution_day_03_01, solution_day_03_02};
use aoc_2022::solutions::day04::{solution_day_04_01, solution_day_04_02};
use aoc_2022::solutions::day05::{solution_day_05_01, solution_day_05_02};
use aoc_2022::solutions::day06::{solution_day_06_01, solution_day_06_02};
use aoc_2022::solutions::day07::{solution_day_07_01, solution_day_07_02};
use aoc_2022::solutions::day08::{solution_day_08_01, solution_day_08_02};
use aoc_2022::solutions::day09::{solution_day_09_01, solution_day_09_02};
use aoc_2022::solutions::day10::{solution_day_10_01, solution_day_10_02};
use aoc_2022::solutions::day11::{solution_day_11_01, solution_day_11_02};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn all_solutions(c: &mut Criterion) {
    let mut g = c.benchmark_group("Solutions");

    // day 01
    let file_path: String = String::from("src/inputs/day01.txt");
    g.bench_function("Part 01/Day 01", |b| {
        b.iter(|| solution_day_01_01(black_box(file_path.clone())))
    });
    g.bench_function("Part 02/Day 01", |b| {
        b.iter(|| solution_day_01_02(black_box(file_path.clone())))
    });

    // Day 02
    let file_path: String = String::from("src/inputs/day02.txt");
    g.bench_function("Part 01/Day 02", |b| {
        b.iter(|| solution_day_02_01(black_box(file_path.clone())))
    });
    g.bench_function("Part 02/Day 02", |b| {
        b.iter(|| solution_day_02_02(black_box(file_path.clone())))
    });

    // Day 03
    let file_path: String = String::from("src/inputs/day03.txt");
    g.bench_function("Part 01/Day 03", |b| {
        b.iter(|| solution_day_03_01(black_box(file_path.clone())))
    });
    g.bench_function("Part 02/Day 03", |b| {
        b.iter(|| solution_day_03_02(black_box(file_path.clone())))
    });

    // Day 04
    let file_path: String = String::from("src/inputs/day04.txt");
    g.bench_function("Part 01/Day 04", |b| {
        b.iter(|| solution_day_04_01(black_box(file_path.clone())))
    });
    g.bench_function("Part 02/Day 04", |b| {
        b.iter(|| solution_day_04_02(black_box(file_path.clone())))
    });

    // Day 05
    let file_path: String = String::from("src/inputs/day05.txt");
    g.bench_function("Part 01/Day 05", |b| {
        b.iter(|| solution_day_05_01(black_box(file_path.clone())))
    });
    g.bench_function("Part 02/Day 05", |b| {
        b.iter(|| solution_day_05_02(black_box(file_path.clone())))
    });

    // Day 06
    let file_path: String = String::from("src/inputs/day06.txt");
    g.bench_function("Part 01/Day 06", |b| {
        b.iter(|| solution_day_06_01(black_box(file_path.clone())))
    });
    g.bench_function("Part 02/Day 06", |b| {
        b.iter(|| solution_day_06_02(black_box(file_path.clone())))
    });

    // Day 07
    let file_path: String = String::from("src/inputs/day07.txt");
    g.bench_function("Part 01/Day 07", |b| {
        b.iter(|| solution_day_07_01(black_box(file_path.clone())))
    });
    g.bench_function("Part 02/Day 07", |b| {
        b.iter(|| solution_day_07_02(black_box(file_path.clone())))
    });

    // Day 08
    let file_path: String = String::from("src/inputs/day08.txt");
    g.bench_function("Part 01/Day 08", |b| {
        b.iter(|| solution_day_08_01(black_box(file_path.clone())))
    });
    g.bench_function("Part 02/Day 08", |b| {
        b.iter(|| solution_day_08_02(black_box(file_path.clone())))
    });

    // Day 09
    let file_path: String = String::from("src/inputs/day09.txt");
    g.bench_function("Part 01/Day 09", |b| {
        b.iter(|| solution_day_09_01(black_box(file_path.clone())))
    });
    g.bench_function("Part 02/Day 09", |b| {
        b.iter(|| solution_day_09_02(black_box(file_path.clone())))
    });

    // Day 10
    let file_path: String = String::from("src/inputs/day10.txt");
    g.bench_function("Part 01/Day 10", |b| {
        b.iter(|| solution_day_10_01(black_box(file_path.clone())))
    });
    g.bench_function("Part 02/Day 10", |b| {
        b.iter(|| solution_day_10_02(black_box(file_path.clone())))
    });

    // Day 11
    let file_path: String = String::from("src/inputs/day11.txt");
    g.bench_function("Part 01/Day 11", |b| {
        b.iter(|| solution_day_11_01(black_box(file_path.clone())))
    });
    g.bench_function("Part 02/Day 11", |b| {
        b.iter(|| solution_day_11_02(black_box(file_path.clone())))
    });
}

// fn day_02(c: &mut Criterion) {

// }

criterion_group!(solutions, all_solutions);
criterion_main!(solutions);
