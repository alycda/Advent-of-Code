use ornaments::Solution;
use miette::Context;

use day_18::{Day18 as Day, Part1};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    #[cfg(debug_assertions)]
    panic!("You must run in release mode to get the correct answer");

    let file = include_str!("../../input.txt");
    let result = Day::<Part1>::parse(file).solve(ornaments::Part::One).context("process part 1")?;

    println!("{}", result);
    Ok(())
}