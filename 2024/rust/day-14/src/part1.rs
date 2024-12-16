use ornaments::{PhantomGrid, Position};

use crate::{parse_robot, Day14, Robot};

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

    for i in 0..100 {
        grid.step(&mut robots);
        // println!("Next state:");
        // println!("{}", visualize_robots(&robots, dimensions.x, dimensions.y));
    }

    // println!("Final state:");
    // println!("{}", visualize_robots(&robots, dimensions.x, dimensions.y));

    let q0 = grid.get_quadrant_robots(&robots, 0).len(); // Top Left
    let q1 = grid.get_quadrant_robots(&robots, 1).len(); // Top Right  
    let q2 = grid.get_quadrant_robots(&robots, 2).len(); // Bottom Left
    let q3 = grid.get_quadrant_robots(&robots, 3).len(); // Bottom Right

    // The safety factor is the product of all quadrants
    let safety_factor = q0 * q1 * q2 * q3;

    // println!("Quadrant counts: {} {} {} {}", q0, q1, q2, q3);
    // println!("Safety factor: {}", safety_factor);

    Ok(safety_factor.to_string())
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

        assert_eq!("12", process(input, grid_size)?);
        Ok(())
    }
}