use glam::IVec2;
use nom::{
    bytes::complete::tag, character::complete::{i32 as parse_i32, space0}, sequence::{preceded, separated_pair, tuple}, IResult
};

use crate::AocError;

#[derive(Debug, Clone)]
struct Position(IVec2);


#[derive(Debug, Clone)]
struct Velocity(IVec2);

#[derive(Debug, Clone)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

fn parse_position(input: &str) -> IResult<&str, Position> {
    let (input, coords) = preceded(
        tag("p="),
        separated_pair(parse_i32, tag(","), parse_i32)
    )(input)?;
    
    Ok((input, Position(IVec2::new(coords.0, coords.1))))
}

fn parse_velocity(input: &str) -> IResult<&str, Velocity> {
    let (input, coords) = preceded(
        tag("v="),
        separated_pair(parse_i32, tag(","), parse_i32)
    )(input)?;
    
    Ok((input, Velocity(IVec2::new(coords.0, coords.1))))
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, (position, _, velocity)) = tuple((
        parse_position,
        space0,  // This handles the space between position and velocity
        parse_velocity
    ))(input)?;

    Ok((input, Robot { position, velocity }))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let _ = input.lines().for_each(|line| {
        // let (remainder, pos) = parse_position(line).unwrap();
        // let (_, vel) = parse_velocity(remainder).unwrap();

        // dbg!(pos, remainder);

        let (_, rob) = parse_robot(line).unwrap();

        dbg!(rob);
    });

    Ok("0".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // use rstest::rstest;

    // #[rstest]
    // #[case("", "")]
    // fn test_cases(#[case] input: &str, #[case] expected: &str) {
    //     assert_eq!(process(input).unwrap(), expected);
    // }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process(input)?);
        Ok(())
    }
}

impl std::ops::Deref for Position {
    type Target = IVec2;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for Velocity {
    type Target = IVec2;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}