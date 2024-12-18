use std::marker::PhantomData;

use ornaments::{AocError, PhantomGrid, Position, Solution, UniquePositions};

#[cfg(debug_assertions)]
const BOUNDS: Position = Position::new(6, 6);

#[cfg(not(debug_assertions))]
const BOUNDS: Position = Position::new(70, 70);

pub struct Day18<P>(PhantomGrid, PhantomData<P>);

impl<P> std::ops::Deref for Day18<P> {
    type Target = PhantomGrid;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Part1 { solution: u32 }
pub struct Part2 { solution: Position }

impl Solution for Day18<Part1> {
    type Output = u32;
    type Item = Position;

    fn parse(input: &str) -> Self {
        #[cfg(debug_assertions)]
        let take_how_many = 12;

        #[cfg(not(debug_assertions))]
        let take_how_many = 1024;

        let obstacles: UniquePositions = input.lines()
            .take(take_how_many)
            .map(|line| {
                let parts = line.split(',').collect::<Vec<_>>();
                Position::new(
                    parts[0].parse::<i32>().unwrap(),
                    parts[1].parse::<i32>().unwrap()
                )
            })
            .collect();

        Self(PhantomGrid(obstacles, (Position::ZERO, BOUNDS)), PhantomData)
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        Ok(self.bfs())
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        unreachable!("Part 2 is not implemented for Day 18/Part1")
    }
}

impl Solution for Day18<Part2> {
    type Output = String;
    type Item = Position;

    /// this is PURE ABUSE of an iterator for the sake of the challenge/ learning why NOT to do this (and understanding what each iterator fn is really meant for)
    fn parse(input: &str) -> Self {
        // State mutation outside iterator, should probably use .scan() instead
        let mut obstacles = UniquePositions::new();

        // let obstacles: UniquePositions = input.lines()
        //     // .map_while(|line| {
        //     .skip_while(|line| {
        //         let parts = line.split(',').collect::<Vec<_>>();
        //         let pos = Position::new(
        //             parts[0].parse::<i32>().unwrap(),
        //             parts[1].parse::<i32>().unwrap()
        //         );

        //         // Add this obstacle
        //         obstacles.insert(pos);
        //         let grid = PhantomGrid(obstacles.clone(), (Position::ZERO, BOUNDS));
        //         let path_length = grid.bfs();

        //         // if path_length == u32::MAX {
        //         //     None
        //         // } else {
        //         //     Some(pos)
        //         // }

        //         path_length == u32::MAX
        //     }).collect();

        let mut blocker = None; // More external state mutation
        let obstacles_remaining: UniquePositions = input.lines()
            // Abuse #1: Using skip_while for side effects
            .skip_while(|line| {
                let parts = line.split(',').collect::<Vec<_>>();
                let pos = Position::new(
                    parts[0].parse::<i32>().unwrap(),
                    parts[1].parse::<i32>().unwrap()
                );
                
                // Abuse #2: Mutating external state in predicate
                obstacles.insert(pos);
                let grid = PhantomGrid(obstacles.clone(), (Position::ZERO, BOUNDS));
                
                if grid.bfs() == u32::MAX {
                    // Abuse #3: Using predicate for side effect storage
                    blocker = Some(pos);

                    // Abuse #4: Using boolean return for flow control rather than filtering
                    false  // Stop skipping here
                } else {
                    true   // Keep skipping
                }
            })
            // Abuse #5: Processing remaining items we don't actually need (because the original data structure is a HashSet and I didn't want to break apart Day18 "just because")
            .map(|line| {
                let parts = line.split(',').collect::<Vec<_>>();
                Position::new(
                    parts[0].parse::<i32>().unwrap(),
                    parts[1].parse::<i32>().unwrap()
                )
            })
            // Abuse #6: Collecting items we immediately discard (so that the iterator will run)
            .collect();

        if let Some(pos) = blocker {
            // Abuse #7: Clearing state we just built up
            obstacles.clear();
            // We have our blocking position
            obstacles.insert(dbg!(pos));
        }

        Self(PhantomGrid(obstacles, (Position::ZERO, BOUNDS)), PhantomData)
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        unreachable!("Part 1 is not implemented for Day 18/Part2")
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        Ok(self.iter().take(1).fold(String::new(), |_acc, pos| {
            format!("{},{}", pos.x, pos.y)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!("22", Day18::<Part1>::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!("6,1", Day18::<Part2>::parse(input).solve(Part::Two)?);
        Ok(())
    }
}