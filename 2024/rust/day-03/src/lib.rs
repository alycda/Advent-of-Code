//! Day 3: Mull It Over
//! 
//! Handles multiplication instructions with enable/disable states
//! 
//! --- Part One ---
//! 
//! multiply some numbers (that match a valid pattern) and sum the results
//! 
//! --- Part Two ---
//! 
//! * The do() instruction enables future mul instructions.
//!     - At the beginning of the program, mul instructions are enabled.
//! * The don't() instruction disables future mul instructions.
//!     - Only the most recent do() or don't() instruction applies.

use nom::{
    bytes::complete::{tag, take_until}, character::complete::{char, digit1}, error::ErrorKind, sequence::{preceded, terminated, tuple}, IResult
};

use ornaments::{Solution, AocError};

pub use crate::Day3 as Day;

/// Represents a multiplication operation with two operands
#[derive(Debug, Clone, Copy)]
pub struct Product(usize, usize);

impl Product {
    /// Creates a new Product from string representations of numbers
    /// 
    /// # Panics
    /// Panics if either string cannot be parsed as usize
    pub fn new(a: &str, b: &str) -> Self {
        Self(a.parse().expect("is a number"), b.parse().expect("is a number"))
    }

    /// Computes the product of the two numbers
    pub fn value(&self) -> usize {
        self.0 * self.1
    }
}

/// Parses multiplication expressions in the EXACT format "mul(x,y)"
fn parse_mul(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = take_until("mul(")(input)?;
    preceded(
        tag("mul("),
        terminated(
            tuple((
                digit1,
                // consume the comma
                preceded(char(','), digit1),
            )),
            char(')'),
        ),
    )(input)
}

/// Represents possible instructions in the program
#[derive(Debug)]
enum Instruction {
    Mul(Product),
    Do,
    Dont,
}

#[derive(Debug, Clone)]
pub struct Day3(Vec<Product>);

impl Day3 {
    fn parse_all_products(mut input: &str) -> nom::IResult<&str, Vec<Product>, nom::error::Error<&str>> {
        let mut products = Vec::new();

        while !input.is_empty() {
            match parse_mul(input) {
                Ok((remainder, (a, b))) => {
                    products.push(Product::new(a, b));
                    input = remainder;
                }
                // Handle various parsing errors by skipping invalid input
                Err(nom::Err::Error(err)) => {
                    match err.code {
                        ErrorKind::Char if input.len() > 1 => {
                            input = &input[1..];
                        }
                        ErrorKind::TakeUntil => {
                            input = "";
                        }
                        _ => return Err(nom::Err::Error(err)),
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok((input, products))
    }

    fn _handle_parse_error<'a>(input: &'a str, err: nom::error::Error<&str>) -> miette::Result<&'a str, AocError> {
        match err.code {
            ErrorKind::Char if input.len() > 1 => Ok(&input[1..]),
            ErrorKind::TakeUntil => Ok(""),
            // _ => Err(AocError::ParseError(format!("Parse error: {err:?}"))),
            _ => todo!(),
        }
    }
}

impl std::ops::Deref for Day3 {
    type Target = Vec<Product>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Solution for Day3 {
    type Output = usize;
    type Item = Vec<Product>;

    fn parse(input: &str) -> Self {
        Day3::nom_parser(input)
            .map(|(_, products)| Self(products))
            .unwrap_or_else(|e| panic!("Failed to parse input: {e}"))
    }

    fn nom_parser(input: &str) -> nom::IResult<&str, Self::Item, nom::error::Error<&str>> {
        Day3::parse_all_products(input)
    }

    /// sums all products
    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let output: Self::Output = self.iter()
            .map(|p| p.value())
            .sum();

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", Day3::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", Day3::parse(input).solve(Part::Two)?);
        Ok(())
    }
}
