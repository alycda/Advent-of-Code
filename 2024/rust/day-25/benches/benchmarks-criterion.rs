use criterion::{
    criterion_group, criterion_main, Criterion,
};
use ornaments::{Part, Solution};
use day_25::*;

fn criterion_benchmark_part1(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    let mut group = c.benchmark_group("day_25::part1");
    group.bench_with_input("part1", input, |b, input| {
        b.iter(|| Day::parse(input).solve(Part::One))
    });

    group.finish();
}

criterion_group!(
    benches,
    criterion_benchmark_part1
);
criterion_main!(benches);