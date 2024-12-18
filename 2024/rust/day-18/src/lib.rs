use ornaments::{AocError, PhantomGrid, Position, Solution, UniquePositions};

pub struct Day18(PhantomGrid);

impl std::ops::Deref for Day18 {
    type Target = PhantomGrid;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl std::ops::DerefMut for Day18 {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

impl Solution for Day18 {
    type Output = String;
    type Item = u32;

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

        #[cfg(debug_assertions)]
        let bounds = Position::new(6, 6);

        #[cfg(not(debug_assertions))]
        let bounds = Position::new(70, 70);

        Self(PhantomGrid(obstacles, (Position::ZERO, bounds)))
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        // let mut todo: VecDeque<(Position, u32)> = VecDeque::new();

        // Ok(ornaments::bfs(self.0, Position::ZERO, self.get_bounds().1,&mut todo).to_string())

        Ok(self.bfs().to_string())
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
        assert_eq!("22", Day18::parse(input).solve(Part::One)?);
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
        assert_eq!("6,1", Day18::parse(input).solve(Part::Two)?);
        Ok(())
    }
}