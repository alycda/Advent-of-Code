use nom::{
    IResult,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    sequence::{preceded, terminated, tuple},
};

pub mod custom_error;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Product(usize, usize);

impl Product {
    pub fn new(a: &str, b: &str) -> Self {
        Self(a.parse().unwrap(), b.parse().unwrap())
    }

    pub fn value(&self) -> usize {
        self.0 * self.1
    }
}

// TODO: parse numbers here
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
