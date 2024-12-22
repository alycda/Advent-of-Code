use crate::AocError;


fn mult(a: usize) -> usize {
    a * 64
}

/// To mix a value into the secret number, calculate the bitwise XOR of 
/// the given value and the secret number. Then, the secret number becomes 
/// the result of that operation.
/// 
/// luckily, XOR is commutative, so the order of the operands doesn't matter.
fn mix(secret: usize, result: usize) -> usize {
    result ^ secret
}

/// To prune the secret number, calculate the value of the secret number modulo 16777216. 
/// Then, the secret number becomes the result of that operation.
fn prune(secret: usize) -> usize {
    secret % 16777216
}


// // secret * 64
// // secret / 32
// // secret * 2048
// fn secret(input: usize) -> usize {
//     // let a = input * 64;
//     let a = mult(input);
//     // let b = input ^ a;
//     let b = mix(input, a);
//     // let c = b / 
//     let c = prune(b);

//     dbg!(a, b, c);

//     c
// }

fn calculate_next_secret(mut secret: usize) -> usize {
    // First operation
    let result = secret * 64;
    secret = mix(secret, result);
    secret = prune(secret);

    // Second operation
    let result = secret / 32;
    secret = mix(secret, result);
    secret = prune(secret);

    // Third operation
    let result = secret * 2048;
    secret = mix(secret, result);
    secret = prune(secret);

    secret
}


fn repeat(input: usize, count: usize) -> usize {
    if count == 0 {
        return input;
    }

    repeat(calculate_next_secret(input), count - 1)

    // for _ in 0..count {
    //     // let a = mult(secret);
    //     // let b = mix(secret, a);
    //     // let c = prune(b);

    //     // secret = c;

    //     secret(secret)
    // }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output: usize = input.lines()
        .map(|line| {
            let number: usize = line.parse().unwrap();

            // calculate_next_secret(number)
            repeat(number, 2000)
        })
        .sum();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case((42, 15), 37)]
    #[case((15, 42), 37)]
    fn test_mix(#[case] input: (usize, usize), #[case] expected: usize) {
        assert_eq!(mix(input.0, input.1), expected);
    }

    #[rstest]
    #[case(100000000, 16113920)]
    fn test_prune(#[case] input: usize, #[case] expected: usize) {
        assert_eq!(prune(input), expected);
    }

    #[rstest]
    #[case((123, 1), 15887950)]
    #[case((123, 2), 16495136)]
    #[case((123, 3), 527345)]
    #[case((123, 4), 704524)]
    #[case((123, 5), 1553684)]
    #[case((123, 6), 12683156)]
    #[case((123, 7), 11100544)]
    #[case((123, 8), 12249484)]
    #[case((123, 9), 7753432)]
    #[case((123, 10), 5908254)]
    fn test_cases(#[case] input: (usize, usize), #[case] expected: usize) {
        assert_eq!(repeat(input.0, input.1), expected);
    }

    // #[rstest]
    // #[case(("1", 2000), "8685429")]
    // #[case(("10", 2000), "4700978")]
    // #[case(("100", 2000), "15273692")]
    // #[case(("2024", 2000), "8667524")]

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1
10
100
2024";
        assert_eq!("37327623", process(input)?);
        Ok(())
    }
}
