use ornaments::{Part, Solution};
use day_18::{Day18 as Day, Part1, Part2};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    Day::<Part1>::parse(divan::black_box(include_str!("../input.txt",))).solve(Part::One).unwrap();
}

#[divan::bench]
fn part2() {
    Day::<Part2>::parse(divan::black_box(include_str!("../input.txt",))).solve(Part::Two).unwrap();
}