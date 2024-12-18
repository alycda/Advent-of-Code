use nom::{
    bytes::complete::tag, character::complete::{alpha1, char, digit1}, combinator::{map, map_res, opt}, sequence::{preceded, tuple}, IResult
};
use glam::IVec2;

use miette::Diagnostic;
use thiserror::Error;

// pub use crate::Day13 as Day;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
}

#[derive(Debug)]
struct Button(IVec2);

impl std::ops::Deref for Button {
    type Target = IVec2;
    
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

pub mod part1;
pub mod part2;