use std::{collections::{HashMap, HashSet}, hash::Hash};
use tracing::{debug, instrument};

pub type Position = glam::IVec2;
pub type Velocity = glam::IVec2;

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
    pub fn to_offset(&self) -> Position {
        match self {
            Direction::Up => Position::NEG_Y,
            Direction::Down => Position::Y,
            Direction::Left => Position::NEG_X,
            Direction::Right => Position::X
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
pub const DIRECTIONS: [Position; 4] = [Position::NEG_Y, Position::X, Position::Y, Position::NEG_X];
/// Up, NE, Right, SE, Down, SW, Left, NW
pub const ALL_DIRECTIONS: [Position; 8] = [Position::NEG_Y, Position::ONE, Position::X, Position::new(1, -1), Position::Y,  Position::new(-1, -1), Position::NEG_X, Position::new(-1, 1)];

/// stores all chars, not recommended for NUMBERS (u8 vs char)
#[derive(Debug)]
pub struct Grid<T>(Vec<Vec<T>>);
// pub struct Grid<T>(Vec<Vec<T>>);


/// a Region or set of Positions
pub type UniquePositions = HashSet<Position>;

/// Visited positions and the direction in which they were travelled
pub type Backtracks = HashSet<(Position, Direction)>;

/// TODO: Day 10, height/rating
pub struct Something<T>(HashMap<Position, T>);
impl<T> std::ops::Deref for Something<T> {
    type Target = HashMap<Position, T>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: std::fmt::Debug + Copy + PartialEq> Grid<T> {
    pub fn get_width(&self) -> usize {
        self[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.len()
    }

    /// Walks the grid from top-left to bottom-right
    pub fn walk<F: FnMut(Position) -> O, O>(&self, mut see: F) {
        for row in 0..self.get_height() {
            for col in 0..self.get_width() {
                let pos = Position::new(col as i32, row as i32);

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
    pub fn go_straight(&self, start: Position, towards: Position, steps: usize) -> Option<Vec<T>> {
        let end_pos = start + (towards * steps as i32);
        if !self.in_bounds(end_pos) {
            debug!("{steps} steps from {start} to {towards} is out of bounds");
            return None;
        }

        (1..=steps)
            .map(|i| start + (towards * i as i32))
            .map(|pos| self.get_at(pos))
            .collect::<Option<Vec<_>>>()
    }

    pub fn get_at_unbounded(&self, pos: Position) -> T {
        self[pos.y as usize][pos.x as usize]
    }

    /// Bounded by the grid's dimensions
    pub fn get_at(&self, pos: Position) -> Option<T> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.get_width() as i32 || pos.y >= self.get_height() as i32 {
            return None;
        }

        Some(self[pos.y as usize][pos.x as usize])
        // Some(self.get_at_unbounded(pos))
    }

    pub fn to_position(&self, idx: usize) -> Position {
        let cols = self.get_width();
        let chars_per_row = cols + 1;
        let col = idx % chars_per_row;
        let row = idx / chars_per_row;
        Position::new(col as i32, row as i32)
    }

    /// Bounded by the grid's dimensions
    pub fn get_neighbor(&self, from: Position, at: Direction) -> Option<T> {
        let neighbor = from + at.to_offset();

        // self[neighbor.y as usize][neighbor.x as usize]
        self.get_at(neighbor)
    }

    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.get_width() as i32 && pos.y < self.get_height() as i32
    }

    pub fn get_orthogonal_neighbors(&self, pos: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        
        // Same deltas as the working version
        for delta in DIRECTIONS.iter() {
            let new_pos = pos + *delta;
            
            // Boundary check matching the working version
            if new_pos.x >= 0 && new_pos.x < self.get_width() as i32 && 
               new_pos.y >= 0 && new_pos.y < self.get_height() as i32 {
                neighbors.push(new_pos);
            }
        }
        
        neighbors
    }

    // pub fn get_orthogonal_neighbors(&self, from: Position) -> HashMap<Direction, (Position, T)> {
    //     let mut neighbors: HashMap<Direction, (Position, T)> = HashMap::new();

    //     // for direction in DIRECTIONS.iter() {
    //     //     let neighbor = from + *direction;
    //     //     let value = self.get_at(neighbor);

    //     //     if let Some(value) = value {
    //     //         neighbors.insert(
    //     //             match direction {
    //     //                 IVec2 { x: 0, y: 1 } => Direction::Down,
    //     //                 IVec2 { x: 0, y: -1 } => Direction::Up,
    //     //                 IVec2 { x: 1, y: 0 } => Direction::Right,
    //     //                 IVec2 { x: -1, y: 0 } => Direction::Left,
    //     //                 _ => unreachable!()
    //     //             },
    //     //             (neighbor, value)
    //     //         );
    //     //     }
    //     // }

    //     // Look up (decrease Y)
    //     if from.y > 0 {
    //         let neighbor = from + Direction::Up.to_offset();
    //         // neighbors.insert(Direction::Up, (neighbor, self.get_at(neighbor).unwrap()));
    //         neighbors.insert(Direction::Up, (neighbor, self.get_at_unbounded(neighbor)));
    //     }

    //     // Look down (increase Y)
    //     if from.y + 1 < self.get_height() as i32 {
    //         let neighbor = from + Direction::Down.to_offset();
    //         // neighbors.insert(Direction::Down, (neighbor, self.get_at(neighbor).unwrap()));
    //         neighbors.insert(Direction::Down, (neighbor, self.get_at_unbounded(neighbor)));
    //     }

    //     // Look left (decrease X)
    //     if from.x > 0 {
    //         let neighbor = from + Direction::Left.to_offset();
    //         // neighbors.insert(Direction::Left, (neighbor, self.get_at(neighbor).unwrap()));
    //         neighbors.insert(Direction::Left, (neighbor, self.get_at_unbounded(neighbor)));
    //     }

    //     // Look right (increase X)
    //     if from.x + 1 < self.get_width() as i32 {
    //         let neighbor = from + Direction::Right.to_offset();
    //         // neighbors.insert(Direction::Right, (neighbor, self.get_at(neighbor).unwrap()));
    //         neighbors.insert(Direction::Right, (neighbor, self.get_at_unbounded(neighbor)));
    //     }

    //     neighbors
    // }

    fn _get_diagonal_neghbors(&self, _from: Position) -> Vec<(Position, T)> {
        todo!()
    }

    fn _get_all_neighbors(&self, _from: Position) -> Vec<(Position, T)> {
        todo!()
    }

    pub fn flood_fill(&self, start: Position, visited: &mut HashSet<Position>) -> HashSet<Position> {
        let mut region = HashSet::new();
        let mut stack = vec![start];
        let target = self.get_at_unbounded(start);
        
        while let Some(pos) = stack.pop() {
            if !region.insert(pos) {
                continue;
            }
            visited.insert(pos);
            
            for neighbor in self.get_orthogonal_neighbors(pos) {
                // let (neighbor_pos, neighbor_char) = neighbor;
                // if *neighbor_char == target && !region.contains(neighbor_pos) {
                //     stack.push(*neighbor_pos);
                // }
                let neighbor_char = self.get_at_unbounded(pos);
                if neighbor_char == target && !region.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
        
        region
    }

    // pub fn flood_fill(&self, start: Position, visited: &mut HashSet<Position>) -> HashSet<Position> {
    //     let mut region = HashSet::new();
    //     let mut stack = vec![start];
    //     let target = self.get_at_unbounded(start);
        
    //     while let Some(pos) = stack.pop() {
    //         if !region.insert(pos) {
    //             continue;
    //         }
    //         visited.insert(pos);
            
    //         for neighbor in self.get_orthogonal_neighbors(pos) {
    //             // let neighbor_target = self.get_at_unbounded(neighbor);
    //             // let neighbor_target = neighbor.1;

    //             // if target == neighbor.1 && !region.contains(&neighbor.0) {
    //             //     stack.push(neighbor.0);
    //             // }
    //         }
    //     }
        
    //     region
    // }

    pub fn count_region_edges(&self, region: &UniquePositions) -> usize {
        let mut edges = 0;
    
        for pos in region {
            for neighbor in self.get_orthogonal_neighbors(*pos) {
                // Direct comparison now possible
                if !region.contains(&neighbor) {
                    edges += 1;
                }
            }
            
            // Border checks remain the same
            if pos.x == 0 || pos.x == (self.get_width() - 1) as i32 { edges += 1; }
            if pos.y == 0 || pos.y == (self.get_height() - 1) as i32 { edges += 1; }
        }
        
        edges
    }

    // /// assumes Set of Positions that are adjacent/touching...
    // pub fn count_region_edges(&self, region: &UniquePositions) -> usize {
    //     let mut edges = 0;

    //     for pos in region {
    //         for neighbor in self.get_orthogonal_neighbors(*pos) {                
    //             // If neighbor is outside region, it's an edge
    //             if !region.contains(&neighbor.1.0) {
    //                 edges += 1;
    //             }
    //         }
            
    //         // Count border edges
    //         if pos.x == 0 || pos.x == (self.get_width() - 1) as i32 { edges += 1; }
    //         if pos.y == 0 || pos.y == (self.get_height() - 1) as i32 { edges += 1; }
    //     }
        
    //     edges
    // }
}

impl<T> std::ops::Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
/// only stores the interesting positions and minmax bounds
pub struct PhantomGrid(pub UniquePositions, pub (Position, Position));

impl PhantomGrid {
    // Creates new grid with given dimensions
    pub fn new(width: i32, height: i32) -> Self {
        Self(
            HashSet::new(),
            (Position::ZERO, Position::new(width - 1, height - 1))
        )
    }

    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.y >= 0 
            && pos.x <= self.1.1.x 
            && pos.y <= self.1.1.y
    }
}

impl std::ops::Deref for PhantomGrid {
    type Target = UniquePositions;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for PhantomGrid {
    fn deref_mut(&mut self) -> &mut UniquePositions {
        &mut self.0
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