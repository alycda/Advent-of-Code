//! Day 13: Claw Contraption

use nom::{
    bytes::complete::tag, character::complete::{alpha1, char, digit1}, combinator::{map, map_res, opt}, sequence::{preceded, tuple}, IResult
};
use glam::IVec2;

use miette::Diagnostic;
use ornaments::{Position, Solution};
use thiserror::Error;

pub use crate::Day13 as Day;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
}

/// A, B, Prize
pub struct Day13(Vec<(Button, Button, Position)>);

impl std::ops::Deref for Day {
    type Target = Vec<(Button, Button, Position)>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl Day {
    fn solve_button_presses_part1(button_a: &Button, button_b: &Button, target: &Position) -> Option<(i32, i32)> {
        let denominator = button_a.x * button_b.y - button_a.y * button_b.x;
        
        // Using Cramer's rule
        let a = (button_b.y * target.x - button_b.x * target.y) / denominator;
        let b = (-button_a.y * target.x + button_a.x * target.y) / denominator;
        
        // Check if we got integer solutions
        if a * denominator == (button_b.y * target.x - button_b.x * target.y) && 
           b * denominator == (-button_a.y * target.x + button_a.x * target.y) {
            Some((a, b))
        } else {
            None
        }
    }

    fn solve_button_presses_part2(button_a: &Button, button_b: &Button, target: &Position, offset: i64) -> Option<(i64, i64)> {
        // Convert to i64 for large number calculations
        let a_x = button_a.x as i64;
        let a_y = button_a.y as i64;
        let b_x = button_b.x as i64;
        let b_y = button_b.y as i64;
        let t_x = target.x as i64 + offset;
        let t_y = target.y as i64 + offset;
        
        let denominator = a_x * b_y - a_y * b_x;
        
        // Using Cramer's rule with i64
        let a = (b_y * t_x - b_x * t_y) / denominator;
        let b = (-a_y * t_x + a_x * t_y) / denominator;
        
        // Check if we got integer solutions
        if a * denominator == (b_y * t_x - b_x * t_y) && 
           b * denominator == (-a_y * t_x + a_x * t_y) {
            Some((a, b))
        } else {
            None
        }
    }
// }

impl Solution for Day {
    type Output = i64;
    type Item = ();

    fn parse(input: &str) -> Self {
        let games = input.split("\n\n")
            .map(|mini_game| {
                let mut game = mini_game.lines();

                // 3 tokens
                let (_, a) = parse_button(game.next().unwrap()).unwrap();
                let a = Button(a);

                // 1 token
                let (_, b) = parse_button(game.next().unwrap()).unwrap();
                let b = Button(b);

                // prize
                let (_, prize) = parse_prize(game.next().unwrap()).unwrap();

                (a, b, prize)
            }).collect::<Vec<_>>();

        Self(games)
    }

    fn part1(&mut self) -> miette::Result<Self::Output, ornaments::AocError> {
        let total: i32 = self.iter()
            .filter_map(|(button_a, button_b, prize_location)| {
                solve_button_presses_part1(button_a, button_b, prize_location)
            }).map(|(a, b)| {
                (a * 3) + b
            }).sum();

        Ok(total as i64)
    }

    fn part2(&mut self) -> miette::Result<Self::Output, ornaments::AocError> {
        let total: i64 = self.iter()
            .filter_map(|(button_a, button_b, prize_location)| {
                solve_button_presses_part2(button_a, button_b, prize_location, 10_000_000_000_000)
            }).map(|(a, b)| {
                (a * 3) + b
            }).sum();

        Ok(total)
    }
}

#[derive(Debug)]
pub struct Button(Position);

impl std::ops::Deref for Button {
    type Target = Position;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(
        tuple((
            opt(char('+')),
            digit1
        )),
        |(_sign, num): (Option<char>, &str)| num.parse::<i32>()
    )(input)
}

pub fn parse_coordinates(input: &str) -> IResult<&str, IVec2> {
    map(
        tuple((
            preceded(tag("X"), parse_number),
            preceded(tag(", Y"), parse_number)
        )),
        |(x, y)| IVec2::new(x, y)
    )(input)
}

pub fn parse_button(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tuple((
            tag("Button "),
            alpha1,
            tag(": ")
        )),
        parse_coordinates
    )(input)
}

pub fn parse_prize_coordinates(input: &str) -> IResult<&str, IVec2> {
    map(
        tuple((
            preceded(tag("X="), parse_number),
            preceded(tag(", Y="), parse_number)
        )),
        |(x, y)| IVec2::new(x, y)
    )(input)
}

pub fn parse_prize(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tag("Prize: "),
        parse_prize_coordinates
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
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
        assert_eq!("480", Day::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
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
        assert_eq!("875318608908", Day::parse(input).solve(Part::Two)?);
        Ok(())
    }
}