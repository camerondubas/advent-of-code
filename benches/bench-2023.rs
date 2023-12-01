use aoc::{
    get_input,
    year_2023::day_01::{part_1, part_2},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day_01(c: &mut Criterion) {
    let input = get_input("2023", "01", None);
    c.bench_function("2023 01 Part 1", |b| b.iter(|| part_1(black_box(&input))));
    c.bench_function("2023 01 Part 2", |b| b.iter(|| part_2(black_box(&input))));
}

criterion_group! {
  name =  year_2023;
  config = Criterion::default().sample_size(1000);
  targets = day_01
}
criterion_main!(year_2023);
