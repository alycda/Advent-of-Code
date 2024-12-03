use nom::{
    IResult,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    sequence::{preceded, terminated, tuple},
};

pub mod custom_error;

pub mod part1;
pub mod part2;

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
