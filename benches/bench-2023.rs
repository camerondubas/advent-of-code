use aoc::{get_input, year_2023};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day_01(c: &mut Criterion) {
    let input = get_input("2023", "01", None);
    c.bench_function("2023 01 Part 1", |b| {
        b.iter(|| year_2023::day_01::part_1(black_box(&input)))
    });
    c.bench_function("2023 01 Part 2", |b| {
        b.iter(|| year_2023::day_01::part_2(black_box(&input)))
    });
}

pub fn day_02(c: &mut Criterion) {
    let input = get_input("2023", "02", None);

    c.bench_function("2023 02 Part 1", |b| {
        b.iter(|| year_2023::day_02::part_1(black_box(&input)))
    });
    c.bench_function("2023 02 Part 2", |b| {
        b.iter(|| year_2023::day_02::part_2(black_box(&input)))
    });
}

pub fn day_03(c: &mut Criterion) {
    let input = get_input("2023", "03", None);

    c.bench_function("2023 03 Part 1", |b| {
        b.iter(|| year_2023::day_03::part_1(black_box(&input)))
    });
    c.bench_function("2023 03 Part 2", |b| {
        b.iter(|| year_2023::day_03::part_2(black_box(&input)))
    });
}

pub fn day_04(c: &mut Criterion) {
    let input = get_input("2023", "04", None);

    c.bench_function("2023 04 Part 1", |b| {
        b.iter(|| year_2023::day_04::part_1(black_box(&input)))
    });
    c.bench_function("2023 04 Part 2", |b| {
        b.iter(|| year_2023::day_04::part_2(black_box(&input)))
    });
}

criterion_group! {
  name =  year_2023;
  config = Criterion::default().sample_size(1000);
  targets = day_01, day_02, day_03, day_04
}
criterion_main!(year_2023);
