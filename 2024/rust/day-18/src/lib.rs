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

pub struct Part1;
pub struct Part2;

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

    // not quite full abuse of an iterator, but close. THIS IS NOT GREAT CODE, it was a learning experience (with scan)
    fn parse(input: &str) -> Self {
        let blocker = input.lines()
            .scan(UniquePositions::new(), |obstacles, line| {
                let parts = line.split(',').collect::<Vec<_>>();
                let pos = Position::new(
                    parts[0].parse::<i32>().unwrap(),
                    parts[1].parse::<i32>().unwrap()
                );

                obstacles.insert(pos);
                let grid = PhantomGrid(obstacles.clone(), (Position::ZERO, BOUNDS));
                Some((pos, grid.bfs() == u32::MAX))
            })
            .find(|(_, is_blocked)| *is_blocked)
            .map(|(pos, _)| pos);

        Self(PhantomGrid(UniquePositions::from([blocker.unwrap()]), (Position::ZERO, BOUNDS)), PhantomData)
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