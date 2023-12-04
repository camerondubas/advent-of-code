use aoc::{
    get_input,
    year_2022::{self},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day_01(c: &mut Criterion) {
    let input = get_input("2022", "01", None);
    c.bench_function("2022 01 Part 1", |b| {
        b.iter(|| year_2022::day_01::part_1(black_box(&input)))
    });
    c.bench_function("2022 01 Part 2", |b| {
        b.iter(|| year_2022::day_01::part_2(black_box(&input)))
    });
}

pub fn day_02(c: &mut Criterion) {
    let input = get_input("2022", "02", None);

    c.bench_function("2022 02 Part 1", |b| {
        b.iter(|| year_2022::day_02::part_1(black_box(&input)))
    });
    c.bench_function("2022 02 Part 2", |b| {
        b.iter(|| year_2022::day_02::part_2(black_box(&input)))
    });
}

pub fn day_03(c: &mut Criterion) {
    let input = get_input("2022", "03", None);

    c.bench_function("2022 03 Part 1", |b| {
        b.iter(|| year_2022::day_03::part_1(black_box(&input)))
    });
    c.bench_function("2022 03 Part 2", |b| {
        b.iter(|| year_2022::day_03::part_2(black_box(&input)))
    });
}

pub fn day_04(c: &mut Criterion) {
    let input = get_input("2022", "04", None);

    c.bench_function("2022 04 Part 1", |b| {
        b.iter(|| year_2022::day_04::part_1(black_box(&input)))
    });
    c.bench_function("2022 04 Part 2", |b| {
        b.iter(|| year_2022::day_04::part_2(black_box(&input)))
    });
}

pub fn day_05(c: &mut Criterion) {
    let input = get_input("2022", "05", None);

    c.bench_function("2022 05 Part 1", |b| {
        b.iter(|| year_2022::day_05::part_1(black_box(&input)))
    });
    c.bench_function("2022 05 Part 2", |b| {
        b.iter(|| year_2022::day_05::part_2(black_box(&input)))
    });
}

pub fn day_06(c: &mut Criterion) {
    let input = get_input("2022", "06", None);

    c.bench_function("2022 06 Part 1", |b| {
        b.iter(|| year_2022::day_06::part_1(black_box(&input)))
    });
    c.bench_function("2022 06 Part 2", |b| {
        b.iter(|| year_2022::day_06::part_2(black_box(&input)))
    });
}

pub fn day_07(c: &mut Criterion) {
    let input = get_input("2022", "07", None);

    c.bench_function("2022 07 Part 1", |b| {
        b.iter(|| year_2022::day_07::part_1(black_box(&input)))
    });
    c.bench_function("2022 07 Part 2", |b| {
        b.iter(|| year_2022::day_07::part_2(black_box(&input)))
    });
}

pub fn day_08(c: &mut Criterion) {
    let input = get_input("2022", "08", None);

    c.bench_function("2022 08 Part 1", |b| {
        b.iter(|| year_2022::day_08::part_1(black_box(&input)))
    });
    c.bench_function("2022 08 Part 2", |b| {
        b.iter(|| year_2022::day_08::part_2(black_box(&input)))
    });
}

pub fn day_09(c: &mut Criterion) {
    let input = get_input("2022", "09", None);

    c.bench_function("2022 09 Part 1", |b| {
        b.iter(|| year_2022::day_09::part_1(black_box(&input)))
    });
    c.bench_function("2022 09 Part 2", |b| {
        b.iter(|| year_2022::day_09::part_2(black_box(&input)))
    });
}

pub fn day_10(c: &mut Criterion) {
    let input = get_input("2022", "10", None);

    c.bench_function("2022 10 Part 1", |b| {
        b.iter(|| year_2022::day_10::part_1(black_box(&input)))
    });
    c.bench_function("2022 10 Part 2", |b| {
        b.iter(|| year_2022::day_10::part_2(black_box(&input)))
    });
}

pub fn day_11(c: &mut Criterion) {
    let input = get_input("2022", "11", None);

    c.bench_function("2022 11 Part 1", |b| {
        b.iter(|| year_2022::day_11::part_1(black_box(&input)))
    });
    c.bench_function("2022 11 Part 2", |b| {
        b.iter(|| year_2022::day_11::part_2(black_box(&input)))
    });
}

pub fn day_12(c: &mut Criterion) {
    let input = get_input("2022", "12", None);

    c.bench_function("2022 12 Part 1", |b| {
        b.iter(|| year_2022::day_12::part_1(black_box(&input)))
    });

    // // This is too slow to benchmark
    // c.bench_function("2022 04 Part 2", |b| {
    //     b.iter(|| year_2022::day_12::part_2(black_box(&input)))
    // });
}

criterion_group!(
    year_2022, day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10,
    day_11, day_12
);
criterion_main!(year_2022);
