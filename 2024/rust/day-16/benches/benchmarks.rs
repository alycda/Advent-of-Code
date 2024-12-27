use ornaments::Solution;
use day_16::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    Day::parse(divan::black_box(include_str!(
        "../input.txt",
    ))).solve(ornaments::Part::One)
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}