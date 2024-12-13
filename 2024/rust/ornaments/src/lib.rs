use glam::IVec2;

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn to_offset(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::NEG_Y,
            Direction::Down => IVec2::Y,
            Direction::Left => IVec2::NEG_X,
            Direction::Right => IVec2::X
        }
    }
}

/// Down, Up, Right, Left
pub const DIRECTIONS: [IVec2; 4] = [IVec2::Y, IVec2::NEG_Y, IVec2::X, IVec2::NEG_X];

pub struct Grid(Vec<Vec<char>>);

impl Grid {
    fn get_width(&self) -> usize {
        self[0].len()
    }

    fn get_height(&self) -> usize {
        self.len()
    }

    pub fn walk<F: FnMut(IVec2) -> O, O>(&self, mut see: F) {
        for row in 0..self.get_height() {
            for col in 0..self.get_width() {
                let pos = IVec2::new(col as i32, row as i32);

                see(pos);
            }
        }
    }

    pub fn get_at(&self, pos: IVec2) -> char {
        self[pos.y as usize][pos.x as usize]
    }

    fn get_neighbor(&self, from: IVec2, at: Direction) -> char {
        let neighbor = from + at.to_offset();

        // self[neighbor.y as usize][neighbor.x as usize]
        self.get_at(neighbor)
    }

    fn get_orthogonal_neighbors(&self, from: IVec2) -> Vec<(IVec2, char)> {
        todo!()
    }
}

impl std::ops::Deref for Grid {
    type Target = Vec<Vec<char>>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Solution {
    /// Ensures the output can be converted to a string
    type Output: std::fmt::Display;  
    type Item;

    fn parse(input: &'static str) -> Self;

    /// https://tfpk.github.io/nominomicon/introduction.html
    fn nom_parser(_input: &str) -> nom::IResult<&str, Self::Item, nom::error::Error<&str>> where Self: Sized {
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

    fn to_grid(input: &str) -> Grid {
        Grid(input.lines()
            .map(|line| line.chars().collect())
            .collect())
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
    
    #[error("Failed to parse input: {msg}")]
    #[diagnostic(code(aoc::parse_error))]
    ParseError {
        msg: String,
        #[source_code]
        input: String,
        #[label("error occurred here")]
        span: Option<(usize, usize)>,
    }
}