use ornaments::Solution;
use miette::Context;

// use crate::Day1;
use day_01::Day1;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let result = Day1::parse(file).solve(ornaments::Part::One).context("process part 1")?;

    println!("{}", result);
    Ok(())
}