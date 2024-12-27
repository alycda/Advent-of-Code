use std::collections::HashSet;

use ornaments::{PhantomGrid, Position};

use crate::{parse_robot, Day14, Robot};

fn has_no_overlaps(robots: &[Robot]) -> bool {
    let mut positions = HashSet::new();
    // If we can't insert a position, it means there's already a robot there
    robots.iter().all(|robot| positions.insert(robot.position))
}

#[tracing::instrument]
pub fn process(input: &str, dimensions: Position) -> miette::Result<String, ornaments::AocError> {
    let mut robots = input.lines()
        .map(|line| {
            let (_, robot) = parse_robot(line).unwrap();

            // dbg!(robot)
            robot
        })
        .collect::<Vec<_>>();
    assert!(robots.len() > 0);

    // dbg!(robots);
    // println!("Initial state:");
    // println!("{}", visualize_robots(&robots, dimensions.x, dimensions.y));

    let mut grid = Day14(PhantomGrid::new(dimensions.x, dimensions.y));

    let mut steps = 0;

    for step in 0.. {
        grid.step(&mut robots);
        // println!("Next state:");
        // println!("{}", visualize_robots(&robots, dimensions.x, dimensions.y));

        if has_no_overlaps(&robots) {
            // println!("Found no overlaps at step: {}", step + 1);  // +1 because we start at 0
            steps = step + 1;
            break;
        }
    }

    // println!("Final state:");
    // println!("{}", Day14::visualize_robots(&robots, dimensions.x, dimensions.y));

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let grid_size = Position::new(11, 7);

        assert_eq!("1", process(input, grid_size)?);
        Ok(())
    }
}