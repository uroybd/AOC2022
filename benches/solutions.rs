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
use aoc_2022::solutions::day12::{solution_day_12_01, solution_day_12_02};
use aoc_2022::solutions::day13::{solution_day_13_01, solution_day_13_02};
use aoc_2022::solutions::day14::{solution_day_14_01, solution_day_14_02};
use aoc_2022::solutions::day15::{solution_day_15_01, solution_day_15_02};
use aoc_2022::solutions::day16::{solution_day_16_01, solution_day_16_02};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn all_solutions(c: &mut Criterion) {
    let mut g = c.benchmark_group("Solutions");

    // day 01
    let file_path: String = String::from("src/inputs/day01.txt");
    g.bench_function("Day 01/Part 01", |b| {
        b.iter(|| solution_day_01_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 01/Part 02", |b| {
        b.iter(|| solution_day_01_02(black_box(file_path.clone())))
    });

    // Day 02
    let file_path: String = String::from("src/inputs/day02.txt");
    g.bench_function("Day 02/Part 01", |b| {
        b.iter(|| solution_day_02_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 02/Part 02", |b| {
        b.iter(|| solution_day_02_02(black_box(file_path.clone())))
    });

    // Day 03
    let file_path: String = String::from("src/inputs/day03.txt");
    g.bench_function("Day 03/Part 01", |b| {
        b.iter(|| solution_day_03_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 03/Part 02", |b| {
        b.iter(|| solution_day_03_02(black_box(file_path.clone())))
    });

    // Day 04
    let file_path: String = String::from("src/inputs/day04.txt");
    g.bench_function("Day 04/Part 01", |b| {
        b.iter(|| solution_day_04_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 04/Part 02", |b| {
        b.iter(|| solution_day_04_02(black_box(file_path.clone())))
    });

    // Day 05
    let file_path: String = String::from("src/inputs/day05.txt");
    g.bench_function("Day 05/Part 01", |b| {
        b.iter(|| solution_day_05_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 05/Part 02", |b| {
        b.iter(|| solution_day_05_02(black_box(file_path.clone())))
    });

    // Day 06
    let file_path: String = String::from("src/inputs/day06.txt");
    g.bench_function("Day 06/Part 01", |b| {
        b.iter(|| solution_day_06_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 06/Part 02", |b| {
        b.iter(|| solution_day_06_02(black_box(file_path.clone())))
    });

    // Day 07
    let file_path: String = String::from("src/inputs/day07.txt");
    g.bench_function("Day 07/Part 01", |b| {
        b.iter(|| solution_day_07_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 07/Part 02", |b| {
        b.iter(|| solution_day_07_02(black_box(file_path.clone())))
    });

    // Day 08
    let file_path: String = String::from("src/inputs/day08.txt");
    g.bench_function("Day 08/Part 01", |b| {
        b.iter(|| solution_day_08_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 08/Part 02", |b| {
        b.iter(|| solution_day_08_02(black_box(file_path.clone())))
    });

    // Day 09
    let file_path: String = String::from("src/inputs/day09.txt");
    g.bench_function("Day 09/Part 01", |b| {
        b.iter(|| solution_day_09_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 09/Part 02", |b| {
        b.iter(|| solution_day_09_02(black_box(file_path.clone())))
    });

    // Day 10
    let file_path: String = String::from("src/inputs/day10.txt");
    g.bench_function("Day 10/Part 01", |b| {
        b.iter(|| solution_day_10_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 10/Part 02", |b| {
        b.iter(|| solution_day_10_02(black_box(file_path.clone())))
    });

    // Day 11
    let file_path: String = String::from("src/inputs/day11.txt");
    g.bench_function("Day 11/Part 01", |b| {
        b.iter(|| solution_day_11_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 11/Part 02", |b| {
        b.iter(|| solution_day_11_02(black_box(file_path.clone())))
    });

    // Day 12
    let file_path: String = String::from("src/inputs/day12.txt");
    g.bench_function("Day 12/Part 01", |b| {
        b.iter(|| solution_day_12_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 12/Part 02", |b| {
        b.iter(|| solution_day_12_02(black_box(file_path.clone())))
    });

    // Day 13
    let file_path: String = String::from("src/inputs/day13.txt");
    g.bench_function("Day 13/Part 01", |b| {
        b.iter(|| solution_day_13_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 13/Part 02", |b| {
        b.iter(|| solution_day_13_02(black_box(file_path.clone())))
    });

    // Day 14
    let file_path: String = String::from("src/inputs/day14.txt");
    g.bench_function("Day 14/Part 01", |b| {
        b.iter(|| solution_day_14_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 14/Part 02", |b| {
        b.iter(|| solution_day_14_02(black_box(file_path.clone())))
    });

    // Day 15
    let file_path: String = String::from("src/inputs/day15.txt");
    g.bench_function("Day 15/Part 01", |b| {
        b.iter(|| solution_day_15_01(black_box(file_path.clone()), black_box(2000000)))
    });
    g.bench_function("Day 15/Part 02", |b| {
        b.iter(|| solution_day_15_02(black_box(file_path.clone()), black_box(4000000)))
    });

    // Day 16
    let file_path: String = String::from("src/inputs/day16.txt");
    g.bench_function("Day 16/Part 01", |b| {
        b.iter(|| solution_day_16_01(black_box(file_path.clone())))
    });
    g.bench_function("Day 16/Part 02", |b| {
        b.iter(|| solution_day_16_02(black_box(file_path.clone())))
    });

    g.finish()
}

criterion_group!(solutions, all_solutions);
criterion_main!(solutions);
