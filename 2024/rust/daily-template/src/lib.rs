use ornaments::{AocError, Solution};

pub mod part1;
pub mod part2;

pub use crate::DayX as Day;

pub struct DayX;

impl DayX {}

impl Solution for DayX {}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "";
        assert_eq!("0", Day::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "";
        assert_eq!("0", Day::parse(input).solve(Part::Two)?);
        Ok(())
    }
}