//! Day 16: Reindeer Maze

use std::{cmp::Ordering, collections::BinaryHeap};

use ornaments::{AocError, Backtracks, Direction, PhantomGrid, Position, Solution, UniquePositions};

pub mod part2;

pub use crate::Day16 as Day;

pub struct Day16 {
    end: Position,
    maze: PhantomGrid,
    state: InternalState,
}

// error[E0277]: the trait bound `glam::i32::ivec2::IVec2: Ord` is not satisfied
//   --> day-16/src/lib.rs:18:5
//    |
// 15 | #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
//    |                                --- in this derive macro expansion
// ...
// 18 |     position: Position,
//    |     ^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `glam::i32::ivec2::IVec2`
//    |
//    = note: this error originates in the derive macro `Ord` (in Nightly builds, run with -Z macro-backtrace for more info)
// #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct InternalState {
    position: Position,
    direction: Direction,
    cost: usize,
    predecessor: Option<(Position, Direction)>,
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

impl Day {
    /// Generate next possible moves:
    fn _try_move() {            
        // // 1. Try moving forward
        // let next_pos = current.position + current.direction.to_position();
        // if !walls.contains(&next_pos) {
        //     queue.push(InternalState {
        //         cost: current.cost + 1,
        //         position: next_pos,
        //         direction: current.direction,
        //         predecessor: Some((current.position, current.direction)),
        //     });
        // }

        // // 2. Try turning right (cost 1000)
        // let right_dir = current.direction.turn_right();
        // queue.push(InternalState {
        //     cost: current.cost + 1000,
        //     position: current.position,
        //     direction: right_dir,
        //     predecessor: Some((current.position, current.direction)),
        // });

        // // 3. Try turning left (cost 1000)
        // let left_dir = current.direction.turn_left();
        // queue.push(InternalState {
        //     cost: current.cost + 1000,
        //     position: current.position,
        //     direction: left_dir,
        //     predecessor: Some((current.position, current.direction)),
        // });
    }
}

fn reconstruct_paths(
    state: &(Position, Direction),
    state_costs: &std::collections::HashMap<(Position, Direction), (usize, Vec<InternalState>)>,
    visited: &mut Backtracks,
    optimal_tiles: &mut UniquePositions
) {
    // If we've visited this state (position + direction), stop
    if !visited.insert(*state) {
        return;
    }
    
    // Add just the position to optimal tiles
    optimal_tiles.insert(state.0);
    
    // For each state that led to this one
    if let Some((_, states)) = state_costs.get(state) {
        for prev_state in states {
            if let Some(pred) = &prev_state.predecessor {
                reconstruct_paths(
                    pred,
                    state_costs,
                    visited,
                    optimal_tiles
                );
            }
        }
    }
}

impl Solution for Day {
    type Output = usize;
    type Item = ();

    fn parse(input: &str) -> Self {
        let grid = Day::to_grid(input);
        let start = grid.to_position(input.find("S").unwrap());
        let end = grid.to_position(input.find("E").unwrap());
        let maze = grid.to_maze('#');

        Self {
            end,
            maze,
            state: InternalState {
                position: start,
                direction: Direction::Right,
                cost: 0,
                predecessor: None,
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
            // self.try_move();
            
            // 1. Try moving forward
            let next_pos = current.position + current.direction.to_offset();
            if !self.maze.contains(&next_pos) {
                queue.push(InternalState {
                    cost: current.cost + 1,
                    position: next_pos,
                    direction: current.direction,
                    predecessor: None,
                });
            }

            // 2. Try turning right (cost 1000)
            let right_dir = current.direction.turn_right();
            queue.push(InternalState {
                cost: current.cost + 1000,
                position: current.position,
                direction: right_dir,
                predecessor: None,
            });

            // 3. Try turning left (cost 1000)
            let left_dir = current.direction.turn_left();
            queue.push(InternalState {
                cost: current.cost + 1000,
                position: current.position,
                direction: left_dir,
                predecessor: None,
            });
        }

        Ok(0)
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        // Create priority queue for states to explore
        let mut queue = BinaryHeap::new();
        
        // Track visited states (position + direction)
        let mut visited = Backtracks::new();

        let mut state_costs = Backtracks::new();

        queue.push(self.state);
        let mut min_cost = usize::MAX;
        let mut optimal_tiles = UniquePositions::new();
        let mut path_visited = UniquePositions::new();

        while let Some(current) = queue.pop() {
            // If cost exceeds minimum, skip this path
            if current.cost > min_cost {
                continue;
            }

            let state_key = (current.position, current.direction);

            // // Check if we've seen this state
            // match state_costs.get(&state_key) {
            //     Some((prev_cost, _)) if current.cost > *prev_cost => continue,
            //     Some((prev_cost, paths)) if current.cost == *prev_cost => {
            //         // Add this path to existing equal-cost paths
            //         let mut new_paths = paths.clone();
            //         new_paths.push(current.clone());
            //         state_costs.insert(state_key, (current.cost, new_paths));
            //     },
            //     _ => {
            //         // New best path to this state
            //         state_costs.insert(state_key, (current.cost, vec![current.clone()]));
            //     }
            // }

            // // If we reached the end, reconstruct the paths to return the cost
            // if current.position == end {
            //     if current.cost < min_cost {
            //         min_cost = current.cost;
            //         optimal_tiles.clear();
            //         path_visited.clear();
            //         reconstruct_paths(
            //             &(current.position, current.direction),
            //             &state_costs,
            //             &mut path_visited,
            //             &mut optimal_tiles
            //         );
            //     } else if current.cost == min_cost {
            //         reconstruct_paths(
            //             &(current.position, current.direction),
            //             &state_costs,
            //             &mut path_visited,
            //             &mut optimal_tiles
            //         );
            //     }
            // }

            // Skip if we've seen this state
            // let state_key = (current.position, current.direction);
            if !visited.insert(state_key) {
                continue;
            }

            // Generate next possible moves:
            // self.try_move();
            
            // 1. Try moving forward
            let next_pos = current.position + current.direction.to_offset();
            if !self.maze.contains(&next_pos) {
                queue.push(InternalState {
                    cost: current.cost + 1,
                    position: next_pos,
                    direction: current.direction,
                    predecessor: Some((current.position, current.direction)),
                });
            }

            // 2. Try turning right (cost 1000)
            let right_dir = current.direction.turn_right();
            queue.push(InternalState {
                cost: current.cost + 1000,
                position: current.position,
                direction: right_dir,
                predecessor: Some((current.position, current.direction)),
            });

            // 3. Try turning left (cost 1000)
            let left_dir = current.direction.turn_left();
            queue.push(InternalState {
                cost: current.cost + 1000,
                position: current.position,
                direction: left_dir,
                predecessor: Some((current.position, current.direction)),
            });
        }

        Ok(optimal_tiles.len())
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
        assert_eq!("45", Day::parse(input).solve(Part::Two)?);
        Ok(())
    }

    #[test]
    fn test_part2b() -> miette::Result<()> {
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
        assert_eq!("64", Day::parse(input).solve(Part::Two)?);
        Ok(())
    }
}