use ornaments::{Part, Solution};
use day_05::Day5 as Day;

fn main() {
    // Run registered benchmarks.
    divan::main();
}


#[divan::bench]
fn part1() {
    Day::parse(divan::black_box(include_str!("../input.txt",))).solve(Part::One).unwrap();
}

#[divan::bench]
fn part2() {
    Day::parse(divan::black_box(include_str!("../input.txt",))).solve(Part::Two).unwrap();
}