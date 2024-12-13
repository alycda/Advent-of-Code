use nom::{
    bytes::complete::tag, character::complete::{alpha1, char, digit1, space0, space1}, combinator::{map, map_res, opt}, sequence::{preceded, tuple}, IResult
};
use glam::IVec2;

use crate::AocError;

// A = 3 tokens
// B = 1 token

#[derive(Debug)]
struct Button(IVec2);

impl std::ops::Deref for Button {
    type Target = IVec2;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(
        tuple((
            opt(char('+')),
            digit1
        )),
        |(sign, num): (Option<char>, &str)| num.parse::<i32>()
    )(input)
}

fn parse_coordinates(input: &str) -> IResult<&str, IVec2> {
    map(
        tuple((
            preceded(tag("X"), parse_number),
            preceded(tag(", Y"), parse_number)
        )),
        |(x, y)| IVec2::new(x, y)
    )(input)
}

fn parse_button(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tuple((
            tag("Button "),
            alpha1,
            tag(": ")
        )),
        parse_coordinates
    )(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    input.split("\n\n").for_each(|mini_game| {
        // dbg!(mini_game);

        let mut game = mini_game.lines();

        let (_, a) = parse_button(game.next().unwrap()).unwrap();
        let a = Button(a);
        dbg!(a);
        
        let (_, b) = parse_button(game.next().unwrap()).unwrap();
        let b = Button(b);
        dbg!(b);

    });

    // let a = Button(IVec2::new(94, 34));
    // let b: = Button(IVec2::new(22, 67));

    Ok("0".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // use rstest::rstest;

    // #[rstest]
    // #[case("Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400", "280")]
    // #[case("Button A: X+26, Y+66
// Button B: X+67, Y+21
// Prize: X=12748, Y=12176", "0")]
    // #[case("Button A: X+17, Y+86
// Button B: X+84, Y+37
// Prize: X=7870, Y=6450", "200")]
    // #[case("Button A: X+69, Y+23
// Button B: X+27, Y+71
// Prize: X=18641, Y=10279", "0")]
    // #[case("", "")]
    // #[case("", "")]
    // fn test_cases(#[case] input: &str, #[case] expected: &str) {
    //     assert_eq!(process(input).unwrap(), expected);
    // }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
