use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}};

use ornaments::{AocError, Backtracks, Direction, PhantomGrid, Position, Solution, UniquePositions};

pub mod part2;

pub use crate::Day16 as Day;

pub struct Day16 {
    start: Position,
    end: Position,
    // maze: UniquePositions,
    maze: PhantomGrid,
    state: InternalState,
}

// #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct InternalState {
    position: Position,
    direction: Direction,
    cost: usize,
}

impl Ord for InternalState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for InternalState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day {}

impl Solution for Day {
    type Output = usize;
    type Item = ();

    fn parse(input: &str) -> Self {
        let grid = Day::to_grid(input);
        let start = grid.to_position(input.find("S").unwrap());
        let end = grid.to_position(input.find("E").unwrap());
        let maze = grid.to_maze('#');

        // let mut walls = UniquePositions::new();
        // grid.walk(|pos| {
        //     if grid.get_at(pos).unwrap() == '#' {
        //         walls.insert(pos);
        //     }
        // });

        // let maze = PhantomGrid::new()

        Self {
            start,
            end,
            maze,
            state: InternalState {
                position: start,
                direction: Direction::Right,
                cost: 0,
            },
        }
    }

    fn part1(&mut self) -> Result<Self::Output, AocError> {

        // Create priority queue for states to explore
        let mut queue = BinaryHeap::new();
        
        // Track visited states (position + direction)
        let mut visited = Backtracks::new();

        queue.push(self.state);

        while let Some(current) = queue.pop() {
            // If we reached the end, return the cost
            if current.position == self.end {
                return Ok(current.cost);
            }

            // Skip if we've seen this state
            let state_key = (current.position, current.direction);
            if !visited.insert(state_key) {
                continue;
            }

            // Generate next possible moves:
            
            // 1. Try moving forward
            let next_pos = current.position + current.direction.to_offset();
            if !self.maze.contains(&next_pos) {
                queue.push(InternalState {
                    cost: current.cost + 1,
                    position: next_pos,
                    direction: current.direction,
                });
            }

            // 2. Try turning right (cost 1000)
            let right_dir = current.direction.turn_right();
            queue.push(InternalState {
                cost: current.cost + 1000,
                position: current.position,
                direction: right_dir,
            });

            // 3. Try turning left (cost 1000)
            let left_dir = current.direction.turn_left();
            queue.push(InternalState {
                cost: current.cost + 1000,
                position: current.position,
                direction: left_dir,
            });
        }

        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1a() -> miette::Result<()> {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!("7036", Day::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part1b() -> miette::Result<()> {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!("11048", Day::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2a() -> miette::Result<()> {
        let input = "";
        assert_eq!("0", Day::parse(input).solve(Part::Two)?);
        Ok(())
    }

    #[test]
    fn test_part2b() -> miette::Result<()> {
        let input = "";
        assert_eq!("0", Day::parse(input).solve(Part::Two)?);
        Ok(())
    }
}