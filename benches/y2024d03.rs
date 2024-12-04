use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn y2024d03p01(c: &mut Criterion) {
    let input = aoc::y2024::d03::get_input().unwrap();
    c.bench_function("y2024d03p01", |b| b.iter(|| aoc::y2024::d03::part_one(black_box(&input))));
}

fn y2024d03p02(c: &mut Criterion) {
    let input = aoc::y2024::d03::get_input().unwrap();
    c.bench_function("y2024d03p02", |b| b.iter(|| aoc::y2024::d03::part_two(black_box(&input))));
}

criterion_group!(benches, y2024d03p01, y2024d03p02);
criterion_main!(benches);
