pub trait Solution {
    /// Ensures the output can be converted to a string
    type Output: std::fmt::Display;  

    fn parse(input: &str) -> Self;

    // /// https://tfpk.github.io/nominomicon/introduction.html
    // fn parse_nom(_input: &str) -> Self where Self: Sized {
    //     todo!()
    // }
    
    fn nom_parser(_input: &str) -> nom::IResult<&str, Self, nom::error::Error<&str>> where Self: Sized {
        todo!()
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        todo!()
    }
    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        todo!()
    }
    
    fn solve(&mut self, part: Part) -> miette::Result<String, AocError> {
        Ok(match part {
            Part::One => self.part1()?.to_string(),
            Part::Two => self.part2()?.to_string()
        })
    } 

    fn get_mut(&mut self) -> &mut Self {
        self
    }

    // fn with_data<F, R>(&mut self, f: F) -> R 
    // where 
    //     F: for<'a> FnOnce(&'a mut Self) -> R 
    // {
    //     f(self)
    // }
}

#[derive(Debug, Clone, Copy)]
pub enum Part {
    One,
    Two
}

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
}