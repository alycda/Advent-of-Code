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

use std::{marker::PhantomData, num::ParseIntError, str::FromStr};

use nom::{bytes::complete::{tag, take_until}, character::complete::{char, digit1}, error::ErrorKind, sequence::{preceded, terminated, tuple}, IResult};
use ornaments::{AocError, Solution};

pub use crate::Day3 as Day;

#[derive(Debug)]
pub struct Part1;

#[derive(Debug)]
pub struct Part2;

#[derive(Debug)]
pub struct Day3<P>(Vec<Product>, PhantomData<P>);

impl<P> std::ops::Deref for Day3<P> {
    type Target = Vec<Product>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a multiplication operation with two operands
#[derive(Debug, Clone, Copy)]
pub struct Product(usize, usize);

impl FromStr for Product {
    // type Err = IResult;
    // type Err = miette::Error;
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (a, b) = input.split_once(",").expect("valid input");
        
        Ok(Product(a.parse()?, b.parse()?))
    }
}

impl Product {
    /// Creates a new Product from string representations of numbers
    /// 
    /// # Panics
    /// Panics if either string cannot be parsed as usize
    pub fn new(a: &str, b: &str) -> Self {
        Self(a.parse().expect("a number"), b.parse().expect("a number"))
    }

    /// Computes the product of the two numbers
    pub fn value(&self) -> usize {
        self.0 * self.1
    }
}

impl<P> Day3<P> {

    /// Parses multiplication expressions in the EXACT format "mul(x,y)"
    fn parse_mul(input: &str) -> IResult<&str, (&str, &str)> {
        let (input, trash) = take_until("mul(")(input)?;

        dbg!(trash);

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

    fn parse_all_mul(mut input: &str) -> IResult<&str, Vec<Product>> {
        let mut products = Vec::new();

        while !input.is_empty() {
        // loop {
            dbg!(input);

            // let (remainder, product) = Self::parse_mul(input).unwrap();
            match Self::parse_mul(input) {
                Ok((remainder, product)) => {
                    dbg!(remainder, product);
                    products.push(Product::new(product.0, product.1));
                    input = remainder;
                },
                // Handle various parsing errors by skipping invalid input
                Err(nom::Err::Error(err)) => {
                    match err.code {
                        ErrorKind::Char if input.len() > 1 => {
                            dbg!(err);
                            input = &input[1..];
                        }
                        ErrorKind::TakeUntil => {
                            dbg!(err);
                            input = "";
                            // break;
                        }
                        // _ => return Err(nom::Err::Error(err)),
                        e => panic!("Error: {:?}", e),
                    }
                }
                Err(_) => break,
            }
        }

        Ok((input, products))
    }

}

impl Solution for Day3<Part1> {
    type Output = usize;
    type Item = Vec<Product>;

    fn parse(input: &str) -> Self {
        // let mut products = Vec::new();
        // let mut input = input;

        // while !input.is_empty() {
        // // loop {
        //     dbg!(input);

        //     // let (remainder, product) = Self::parse_mul(input).unwrap();
        //     match Self::parse_mul(input) {
        //         Ok((remainder, product)) => {
        //             dbg!(remainder, product);
        //             products.push(Product::new(product.0, product.1));
        //             input = remainder;
        //         },
        //         // Handle various parsing errors by skipping invalid input
        //         Err(nom::Err::Error(err)) => {
        //             match err.code {
        //                 ErrorKind::Char if input.len() > 1 => {
        //                     input = &input[1..];
        //                 }
        //                 ErrorKind::TakeUntil => {
        //                     input = "";
        //                     // break;
        //                 }
        //                 // _ => return Err(nom::Err::Error(err)),
        //                 e => panic!("Error: {:?}", e),
        //             }
        //         }
        //         Err(_) => break,
        //     }
        // }

        let (_, products) = Day3::<Part1>::parse_all_mul(input).unwrap();

        Day3(products, PhantomData)
    }

    /// sums all products
    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let output: Self::Output = self.iter()
            .map(|p| p.value())
            .sum();

        Ok(output)
    }

    /// sums all products
    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        unimplemented!("Part 2")
    }
}

impl Solution for Day3<Part2> {
    type Output = usize;
    type Item = Vec<Product>;

    fn parse(input: &str) -> Self {
        dbg!(input);

        let (remainder, part1) = take_until::<_, _, nom::error::Error<&str>>("don't()")(input).unwrap();

        // dbg!(remainder, part1);

        let (_, mut products) = Day3::<Part2>::parse_all_mul(part1).unwrap();
        let mut input = remainder;

        loop {
            dbg!(input);


            let (a, next) = take_until::<_, _, nom::error::Error<&str>>("do()")(input).unwrap();
            dbg!(a, next);
            let (remainder, mut new_products) = Day3::<Part2>::parse_all_mul(a).unwrap();
            products.append(&mut new_products);

            input = remainder;




        //     // let (remainder, product) = Self::parse_mul(input).unwrap();
        //     match Self::parse_mul(input) {
        //         Ok((remainder, product)) => {
        //             dbg!(remainder, product);
        //             products.push(Product::new(product.0, product.1));
        //             input = remainder;
        //         },
        //         // Handle various parsing errors by skipping invalid input
        //         Err(nom::Err::Error(err)) => {
        //             match err.code {
        //                 ErrorKind::Char if input.len() > 1 => {
        //                     input = &input[1..];
        //                 }
        //                 ErrorKind::TakeUntil => {
        //                     // input = "";
        //                     break;
        //                 }
        //                 // _ => return Err(nom::Err::Error(err)),
        //                 e => panic!("Error: {:?}", e),
        //             }
        //         }
        //         Err(_) => break,
        //     }
        }

        dbg!(&products);

        Day3(products, PhantomData)
    }

    /// sums all products
    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        unimplemented!("Part 1")
    }

    /// sums all products
    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
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
        assert_eq!("161", Day3::<Part1>::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", Day3::<Part2>::parse(input).solve(Part::Two)?);
        Ok(())
    }

    // #[test]
    // fn test_part3() -> miette::Result<()> {
    //     let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    //     assert_eq!("96", Day3::parse(input).solve(Part::Two)?);
    //     Ok(())
    // }

    // #[test]
    // fn test_part4() -> miette::Result<()> {
    //     let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    //     assert_eq!("144", Day3::parse(input).solve(Part::Two)?);
    //     Ok(())
    // }
}