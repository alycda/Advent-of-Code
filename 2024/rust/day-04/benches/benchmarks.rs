use ornaments::{Part, Solution};
use day_04::{CrossPattern, Day4 as Day, XmasPattern};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    Day::<XmasPattern>::parse(divan::black_box(include_str!("../input.txt",))).solve(Part::One).unwrap();
}

#[divan::bench]
fn part2() {
    Day::<CrossPattern>::parse(divan::black_box(include_str!("../input.txt",))).solve(Part::Two).unwrap();
}