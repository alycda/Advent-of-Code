use ornaments::Solution;
use miette::Context;

use day_06::Day6;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input.txt");
    let result = Day6::parse(file).solve(ornaments::Part::Two).context("process part 2")?;

    println!("{}", result);
    Ok(())

    // > 668 && > 1906 < 6083
    // ! 2247
    // = 1984
    // ?? 2246 ??
}