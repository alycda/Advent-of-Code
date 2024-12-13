use ornaments::Solution;
use miette::Context;

use day_04::{Day4, CrossPattern};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

// #[tracing::instrument]
fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    // let file = include_str!("../../input2.txt");
    // let result = Day4::<CrossPattern>::parse(file).solve(ornaments::Part::Two).context("process part 2")?;

    // println!("{}", result);
    Ok(())
}