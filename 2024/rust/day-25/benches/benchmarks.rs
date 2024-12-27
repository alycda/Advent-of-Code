use ornaments::{Part, Solution};
use day_25::Day25 as Day;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    Day::parse(divan::black_box(include_str!("../input.txt",))).solve(Part::One).unwrap();
}