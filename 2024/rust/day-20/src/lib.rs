use std::collections::{HashMap, HashSet};

use miette::Diagnostic;
use thiserror::Error;

pub type Position = glam::IVec2;

/// a Region or set of Positions
pub type UniquePositions = HashSet<Position>;

/// Up, Right, Down, Left
pub const DIRECTIONS: [Position; 4] = [Position::NEG_Y, Position::X, Position::Y, Position::NEG_X];

/// stores all chars, not recommended for NUMBERS (u8 vs char)
#[derive(Debug)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> std::ops::Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    
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

    pub fn to_position(&self, idx: usize) -> Position {
        let cols = self.get_width();
        let chars_per_row = cols + 1;
        let col = idx % chars_per_row;
        let row = idx / chars_per_row;
        Position::new(col as i32, row as i32)
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

    /// Bounded by the grid's dimensions
    pub fn get_at(&self, pos: Position) -> Option<T> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.get_width() as i32 || pos.y >= self.get_height() as i32 {
            return None;
        }

        Some(self[pos.y as usize][pos.x as usize])
        // Some(self.get_at_unbounded(pos))
    }

    pub fn get_at_unbounded(&self, pos: Position) -> T {
        self[pos.y as usize][pos.x as usize]
    }

    pub fn to_maze(&self, to_match: T) -> PhantomGrid {
        let mut walls = UniquePositions::new();

        self.walk(|pos| {
            if self.get_at_unbounded(pos) == to_match {
                walls.insert(pos);
            }
        });

        PhantomGrid(walls, (Position::ZERO, Position::new(self.get_width() as i32 - 1, self.get_height() as i32 - 1)))
    }
}

#[derive(Debug, Clone)]
/// only stores the interesting positions and minmax bounds
pub struct PhantomGrid(pub UniquePositions, pub (Position, Position));

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

/// TODO: Day 10, height/rating & Day 20
pub struct Something<T>(HashMap<Position, T>);

impl<T> std::ops::Deref for Something<T> {
    type Target = HashMap<Position, T>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Something<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
}

pub mod part1;
pub mod part2;