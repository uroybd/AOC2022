use aoc_2022::solutions::day01::{solution_day_01_01, solution_day_01_02};
use aoc_2022::solutions::day02::{solution_day_02_01, solution_day_02_02};
use aoc_2022::solutions::day03::{solution_day_03_01, solution_day_03_02};
use aoc_2022::solutions::day04::{solution_day_04_01, solution_day_04_02};
use aoc_2022::solutions::day05::{solution_day_05_01, solution_day_05_02};
use aoc_2022::solutions::day06::{solution_day_06_01, solution_day_06_02};
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
}


// fn day_02(c: &mut Criterion) {
    
// }




criterion_group!(solutions, all_solutions);
criterion_main!(solutions);
