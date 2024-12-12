
use ornaments::{Part, Solution};
use day_01::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    Day1::parse(divan::black_box(include_str!("../input1.txt",))).solve(Part::One).unwrap();
}

#[divan::bench]
fn part2() {
    Day1::parse(divan::black_box(include_str!("../input1.txt",))).solve(Part::Two).unwrap();
}
