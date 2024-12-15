use std::{collections::{HashMap, HashSet}, hash::Hash};
use tracing::{debug, instrument};

use glam::IVec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// A, North
    Up,
    /// X, South
    Down,
    /// #, West
    Left,
    /// O, East
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

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

/// Up, Right, Down, Left
pub const DIRECTIONS: [IVec2; 4] = [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X];
/// Up, NE, Right, SE, Down, SW, Left, NW
pub const ALL_DIRECTIONS: [IVec2; 8] = [IVec2::NEG_Y, IVec2::ONE, IVec2::X, IVec2::new(1, -1), IVec2::Y,  IVec2::new(-1, -1), IVec2::NEG_X, IVec2::new(-1, 1)];

/// stores all chars, not recommended for NUMBERS (u8 vs char)
#[derive(Debug)]
pub struct Grid<T>(Vec<Vec<T>>);
// pub struct Grid<T>(Vec<Vec<T>>);

impl<T: std::fmt::Debug + Copy> Grid<T> {
    pub fn get_width(&self) -> usize {
        self[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.len()
    }

    /// Walks the grid from top-left to bottom-right
    pub fn walk<F: FnMut(IVec2) -> O, O>(&self, mut see: F) {
        for row in 0..self.get_height() {
            for col in 0..self.get_width() {
                let pos = IVec2::new(col as i32, row as i32);

                see(pos);
            }
        }
    }

    // fn go_straight_up(&self, start: IVec2, steps: usize) {
    //     self.go_straight(start, IVec2::NEG_Y, steps, None);
    //     todo!()
    // }

    // move in a straight line from the start direction the given number of steps
    // pub fn go_straight<F: Fn() -> bool>(&self, start: IVec2, direction: IVec2, steps: usize, _test: Option<F>) -> Option<Vec<char>> {
    #[instrument]
    pub fn go_straight(&self, start: IVec2, direction: IVec2, steps: usize) -> Option<Vec<T>> {
        let end_pos = start + (direction * steps as i32);
        if !self.in_bounds(end_pos) {
            debug!("{steps} steps from {start} in direction {direction} is out of bounds");
            return None;
        }

        (1..=steps)
            .map(|i| start + (direction * i as i32))
            .map(|pos| self.get_at(pos))
            .collect::<Option<Vec<_>>>()
    }

    pub fn get_at_unbounded(&self, pos: IVec2) -> T {
        self[pos.y as usize][pos.x as usize]
    }

    /// Bounded by the grid's dimensions
    pub fn get_at(&self, pos: IVec2) -> Option<T> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.get_width() as i32 || pos.y >= self.get_height() as i32 {
            return None;
        }

        Some(self[pos.y as usize][pos.x as usize])
        // Some(self.get_at_unbounded(pos))
    }

    pub fn to_position(&self, idx: usize) -> IVec2 {
        let cols = self.get_width();
        let chars_per_row = cols + 1;
        let col = idx % chars_per_row;
        let row = idx / chars_per_row;
        IVec2::new(col as i32, row as i32)
    }

    /// Bounded by the grid's dimensions
    pub fn get_neighbor(&self, from: IVec2, at: Direction) -> Option<T> {
        let neighbor = from + at.to_offset();

        // self[neighbor.y as usize][neighbor.x as usize]
        self.get_at(neighbor)
    }

    fn in_bounds(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.get_width() as i32 && pos.y < self.get_height() as i32
    }

    pub fn get_orthogonal_neighbors(&self, from: IVec2) -> HashMap<Direction, (IVec2, T)> {
        let mut neighbors: HashMap<Direction, (IVec2, T)> = HashMap::new();

        // for direction in DIRECTIONS.iter() {
        //     let neighbor = from + *direction;
        //     let value = self.get_at(neighbor);

        //     if let Some(value) = value {
        //         neighbors.insert(
        //             match direction {
        //                 IVec2 { x: 0, y: 1 } => Direction::Down,
        //                 IVec2 { x: 0, y: -1 } => Direction::Up,
        //                 IVec2 { x: 1, y: 0 } => Direction::Right,
        //                 IVec2 { x: -1, y: 0 } => Direction::Left,
        //                 _ => unreachable!()
        //             },
        //             (neighbor, value)
        //         );
        //     }
        // }

        // Look up (decrease Y)
        if from.y > 0 {
            let neighbor = from + Direction::Up.to_offset();
            // neighbors.insert(Direction::Up, (neighbor, self.get_at(neighbor).unwrap()));
            neighbors.insert(Direction::Up, (neighbor, self.get_at_unbounded(neighbor)));
        }

        // Look down (increase Y)
        if from.y + 1 < self.get_height() as i32 {
            let neighbor = from + Direction::Down.to_offset();
            // neighbors.insert(Direction::Down, (neighbor, self.get_at(neighbor).unwrap()));
            neighbors.insert(Direction::Down, (neighbor, self.get_at_unbounded(neighbor)));
        }

        // Look left (decrease X)
        if from.x > 0 {
            let neighbor = from + Direction::Left.to_offset();
            // neighbors.insert(Direction::Left, (neighbor, self.get_at(neighbor).unwrap()));
            neighbors.insert(Direction::Left, (neighbor, self.get_at_unbounded(neighbor)));
        }

        // Look right (increase X)
        if from.x + 1 < self.get_width() as i32 {
            let neighbor = from + Direction::Right.to_offset();
            // neighbors.insert(Direction::Right, (neighbor, self.get_at(neighbor).unwrap()));
            neighbors.insert(Direction::Right, (neighbor, self.get_at_unbounded(neighbor)));
        }

        neighbors
    }

    fn _get_diagonal_neghbors(&self, _from: IVec2) -> Vec<(IVec2, char)> {
        todo!()
    }

    fn _get_all_neighbors(&self, _from: IVec2) -> Vec<(IVec2, char)> {
        todo!()
    }
}

impl<T> std::ops::Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
/// only stores the interesting positions and minmax bounds
pub struct PhantomGrid(pub HashSet<IVec2>, pub (IVec2, IVec2));

impl PhantomGrid {
    pub fn in_bounds(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.y >= 0 
            && pos.x <= self.1.1.x 
            && pos.y <= self.1.1.y
    }
}

impl std::ops::Deref for PhantomGrid {
    type Target = HashSet<IVec2>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Solution {
    /// Ensures the output can be converted to a string
    type Output: std::fmt::Display;  
    /// used for the nom parser function
    type Item;
    // type Item = (); associated type defaults are unstable

    fn parse(input: &'_ str) -> Self;

    /// generally intended to parse a single line, not the full input
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

    fn to_grid(input: &str) -> Grid<char> {
        Grid(input.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>())
    }

    // fn to_grid<T, F>(input: &str, parser: Option<F>) -> Grid<T>
    // where
    //     F: Fn(char) -> Option<T>,
    // {
    //     Grid(input.lines()
    //         .map(|line| {
    //             line.chars()
    //                 .map(|c| {
    //                     parser.as_ref()
    //                         .map_or_else(|| c as T, |parse| parse(c).unwrap_or_else(|| c as T))
    //                 })
    //                 .collect::<Vec<_>>()
    //         })
    //         .collect::<Vec<Vec<_>>>())
    // }

    fn print(_input: &str) {
        todo!()
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