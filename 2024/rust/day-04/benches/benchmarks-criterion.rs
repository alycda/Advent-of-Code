use criterion::{
    criterion_group, criterion_main, Criterion,
};
use ornaments::{Part, Solution};
use day_04::{CrossPattern, Day4 as Day, XmasPattern};

fn criterion_benchmark_part1(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    let mut group = c.benchmark_group("day_04::part1");
    group.bench_with_input("part1", input, |b, input| b.iter(|| Day::<XmasPattern>::parse(input).solve(Part::One)));

    group.finish();
}

fn criterion_benchmark_part2(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    let mut group = c.benchmark_group("day_04::part2");
    group.bench_with_input("part2", input, |b, input| b.iter(|| Day::<CrossPattern>::parse(input).solve(Part::Two)));
    group.finish();
}

// fn criterion_benchmark_part2(c: &mut Criterion) {
//     let input = include_str!("../input.txt");

//     let mut group = c.benchmark_group("day_04::part2");
//     group.bench_with_input("part2", input, |b, input| {
//         b.iter(|| part2::process(input))
//     });
//     // group.bench_with_input(
//     //     "part2_nom",
//     //     input,
//     //     |b, input| b.iter(|| part2_nom::process(input)),
//     // );
//     // group.bench_with_input(
//     //     "part2_aho_corasick",
//     //     input,
//     //     |b, input| {
//     //         b.iter(|| part2_aho_corasick::process(input))
//     //     },
//     // );
//     group.finish();
// }

criterion_group!(
    benches,
    criterion_benchmark_part1,
    criterion_benchmark_part2
);
criterion_main!(benches);