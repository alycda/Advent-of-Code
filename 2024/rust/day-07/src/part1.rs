use crate::custom_error::AocError;

enum Operator {
    Add,
    Multiply,
}

fn try_evaluate(numbers: &[usize], expected: usize) -> Option<usize> {
    let mut ops: Vec<Operator> = Vec::new();

    evaluate(numbers[0], &numbers[1..], expected, &mut ops)
}

fn evaluate(current: usize, nums: &[usize], target: usize, ops: &mut Vec<Operator>) -> Option<usize> {
    // dbg!(current, target);
    if current > target {
        return None;
    } else if current < 1 {
        panic!("if we are at 0")
    }
    
    // Base case - when we've used all numbers
    if nums.is_empty() {
        return if current == target { Some(current) } else { None }
    }

    let n = nums[0];
    let rest = &nums[1..];

    // Try multiplication first (faster growth)
    if let Some(result) = evaluate(current * n, rest, target, ops) {
        ops.push(Operator::Multiply);
        return Some(result);
    }

    // Try addition if multiplication failed
    if let Some(result) = evaluate(current + n, rest, target, ops) {
        ops.push(Operator::Add);
        return Some(result);
    }

    None
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output = input
        .lines()
        .filter_map(|line| {
            let(expected, operation) = line.split_once(": ").unwrap();

            // dbg!(expected, operation);

            let nums = operation.split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            // dbg!(expected, &nums); 

            try_evaluate(&nums, expected.parse::<usize>().unwrap())
        })
        .sum::<usize>();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // use rstest::rstest;

    // #[rstest]
    // #[case("", "")]
    // fn test_cases(#[case] input: &str, #[case] expected: &str) {
    //     assert_eq!(process(input).unwrap(), expected);
    // }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
