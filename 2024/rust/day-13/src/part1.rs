use glam::IVec2;

use crate::{parse_button, parse_prize, AocError, Button};

fn solve_button_presses(button_a: Button, button_b: Button, target: IVec2) -> Option<(i32, i32)> {
    let denominator = button_a.x * button_b.y - button_a.y * button_b.x;
    
    // Using Cramer's rule
    let a = (button_b.y * target.x - button_b.x * target.y) / denominator;
    let b = (-button_a.y * target.x + button_a.x * target.y) / denominator;
    
    // Check if we got integer solutions
    if a * denominator == (button_b.y * target.x - button_b.x * target.y) && 
       b * denominator == (-button_a.y * target.x + button_a.x * target.y) {
        Some((a, b))
    } else {
        None
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output: i32 = input.split("\n\n")
        .map(|mini_game| {
            // dbg!(mini_game);

            let mut game = mini_game.lines();

            // 3 tokens
            let (_, a) = parse_button(game.next().unwrap()).unwrap();
            let a = Button(a);

            // 1 token
            let (_, b) = parse_button(game.next().unwrap()).unwrap();
            let b = Button(b);

            // prize
            let (_, prize) = parse_prize(game.next().unwrap()).unwrap();

            (a, b, prize)
        })
        .filter_map(|(button_a, button_b, prize_location)| {
            dbg!(solve_button_presses(button_a, button_b, prize_location))
        }).map(|(a, b)| {
            (a * 3) + b
        }).sum();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_button_solver() {
    //     let button_a = Button(IVec2::new(94, 34));
    //     let button_b = Button(IVec2::new(22, 67));
    //     let target = IVec2::new(8400, 5400);
        
    //     let (a, b) = find_button_presses(&button_a, &button_b, target).unwrap();
    //     assert_eq!(*button_a * a + *button_b * b, target);
    // }

    // use rstest::rstest;

    // #[rstest]
    // #[case("Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400", "280")]
    // #[case("Button A: X+26, Y+66
// Button B: X+67, Y+21
// Prize: X=12748, Y=12176", "0")]
    // #[case("Button A: X+17, Y+86
// Button B: X+84, Y+37
// Prize: X=7870, Y=6450", "200")]
    // #[case("Button A: X+69, Y+23
// Button B: X+27, Y+71
// Prize: X=18641, Y=10279", "0")]
    // #[case("", "")]
    // #[case("", "")]
    // fn test_cases(#[case] input: &str, #[case] expected: &str) {
    //     assert_eq!(process(input).unwrap(), expected);
    // }

//     #[test]
//     fn test_process() -> miette::Result<()> {
//         let input = "Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400";
//         assert_eq!("280", process(input)?);
//         Ok(())
//     }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
