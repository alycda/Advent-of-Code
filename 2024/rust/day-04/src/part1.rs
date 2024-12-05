// #![feature(iter_advance_by)]

use crate::custom_error::AocError;

fn count_horizontal(line: &str) -> usize {
    line.matches("XMAS")
        .inspect(|l| {dbg!(l);})
        .count() 
    + line.matches("SAMX")
        .inspect(|l| {dbg!(l);})
        .count()
}

fn count_vertical(input: &str) -> usize {
    let matrix = input.lines().map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<Vec<_>>>();

    let transposed = transpose(&matrix);
    transposed.iter().enumerate().map(|(_i, line)| {
        // dbg!((i, line));

        let line = line.iter().collect::<String>();

        // dbg!(i, line.iter().collect::<String>());
        // dbg!("col:", i, &line);

        count_horizontal(&line)
    }).sum::<usize>()
}

fn count_diagonal_ltr(input: &str) -> usize {
    // // let mut peekable = input.lines().peekable();

    // // let cols = peekable.peek().unwrap().chars().count();
    // // let rows = peekable.count();

    input.lines().enumerate()
    .map(|(i, original_line)| {
        // dbg!(i, original_line);

        input.lines()
            .skip(i)
            .take(4)
            .enumerate()
            // .map(|(idx, line)| {
            //     line.chars().nth(i + idx).unwrap()
            // })
            .filter_map(|(idx, diagonal_line)| {
                diagonal_line.chars().nth(i + idx)
            })
        // .collect::<Vec<_>>()
        .collect::<String>()

    })
    // .filter(|line| line.len() == 4)
    // .filter_map(|line| {
    //     if line.len() == 4 {
    //         Some(line.iter().collect::<String>())
    //     } else {
    //         None
    //     }
    // })
    .map(|line| {
        // dbg!(&line);
        // dbg!(count_horizontal(line));
        // let line = line.iter().collect::<String>();

        count_horizontal(&line)
    })
    // .count()
    // // .inspect(|idx, line|) {}
    // .map(|(row_num, line)| {
    //     // Start position shifts right by row number
    //     line.chars()
    //         .skip(row_num)     // Skip 'row_num' characters
    //         .take(4)   
    //         .inspect(|c| {

    //         })        // Take 4 characters after the skip
    //         .collect::<Vec<_>>()
    // })
    // .enumerate()
    // .map(|(diagonal_idx, line)| {
    // //     // peekable.next();

    // let line = line.iter().collect::<String>();
    //     // dbg!(diagonal_idx, &line);

    //     // dbg!(diagonal_idx, line.iter().collect::<String>());

    //     // dbg!(count_horizontal(&line.iter().collect::<String>()));

    // //     // dbg!(peekable.peek());

    //     count_vertical(&line)
    // })
    .sum::<usize>()
}

// fn extract_sliding_window(input: &[&str]) -> Vec<String> {
//     input.iter()
//         .enumerate()  // Gives us (index, line) pairs
//         .map(|(row_num, line)| {
//             // Start position shifts right by row number
//             line.chars()
//                 .skip(row_num)     // Skip 'row_num' characters
//                 .take(4)           // Take 4 characters after the skip
//                 .collect()
//         })
//         .collect()
// }

fn count_diagonal_rtl(input: &str) -> usize {
    let output = input.lines()
    // .inspect(|line| {
    //     dbg!(line);
    //     // dbg!(line.chars().collect::<String>());
    //     // dbg!(line.chars().rev().collect::<String>());
    // })
    .map(|line| {
        line.chars().rev().collect::<String>()
    })
    // .inspect(|line_reversed| {
    //     dbg!(line_reversed);
    // })
    .collect::<Vec<_>>().join("\n");

    count_diagonal_ltr(&output)
}

// fn process_window(input: &[Vec<char>]) -> Vec<Vec<char>> {
//     input.iter()
//         .enumerate()
//         .map(|(i, row)| {
//             let valid_chars: Vec<char> = row.iter()
//                 .skip(i)  // Skip i characters from the start
//                 .take(row.len() - i)  // Take remaining valid characters
//                 .cloned()
//                 .collect();
            
//             // Pad with dots to maintain row length
//             let mut processed = valid_chars;
//             processed.extend(vec!['.'; i]);
//             processed
//         })
//         .collect()
// }

// fn process_text(input: &[Vec<char>], window_size: usize) -> Vec<Vec<char>> {
//     let mut result = Vec::new();
//     let mut start = 0;
    
//     while start + window_size <= input.len() {
//         let window = &input[start..start + window_size];
//         let processed = process_window(window);
//         result.extend(processed);
//         start += 1;
//     }
    
//     result
// }

fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    
    // Pre-allocate the transposed matrix
    let mut transposed = vec![vec![matrix[0][0].clone(); rows]; cols];
    
    // Fill the transposed matrix
    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j].clone();
        }
    }

    transposed
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let horizontal_count = input
        .lines()
            .map(|line| {
            count_horizontal(line)
        })
        .sum::<usize>();

    // for (i, line) in input.lines().enumerate() {
    //     dbg!((i, line));

    //     for (j, c) in line.chars().enumerate() {
    //         dbg!((j, c));
    //     }
    // }
    
    let vertical_count = count_vertical(input);

    let diagonal_ltr_count = count_diagonal_ltr(input);
    let diagonal_rtl_count = count_diagonal_rtl(input);

    dbg!(horizontal_count, vertical_count, diagonal_ltr_count, diagonal_rtl_count);

    let output = horizontal_count + vertical_count + diagonal_ltr_count + diagonal_rtl_count;

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("..X...
.SAMX.
.A..A.
XMAS.S
.X....", "4")]
// #[case("MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX", "18")]
// #[case("....XXMAS.
// .SAMXMS...
// ...S..A...
// ..A.A.MS.X
// XMASAMX.MM
// X.....XA.A
// S.S.S.S.SS
// .A.A.A.A.A
// ..M.M.M.MM
// .X.X.XMASX", "18")]
#[case("..XXMAS", "1")]
#[case("SAMXMS.", "1")]
#[case("MS.X", "0")]
#[case("XMASAMX.MM", "2")]
#[case("XMAS", "1")]
#[case("SAMX.MM", "1")]
#[case("...XA.A", "0")]
#[case(".X.X.XMASX", "1")]
#[case(".X.XMAS", "1")]
#[case("..XXMAS
AMXMS..
.S..A..
A.A.MS.", "1")]
#[case("...XXMA
SAMXMS.
..S..A.
.A.A.MS", "2")]
#[case("X
M
A
S
A
M
X", "2")]
    fn test_cases(#[case] input: &'static str, #[case] expected: &str) {
        assert_eq!(process(input).unwrap(), expected);
    }


    #[rstest]
    #[case("..X...
.SAMX.
.A..A.
XMAS.S
.X....", 2)]
#[case("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX", 5)]
#[case("....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX", 5)]
    fn test_horizontal(#[case] input: &'static str, #[case] expected: usize) {
        assert_eq!(count_horizontal(input), expected);
    }

    #[rstest]
    #[case("..X...
.SAMX.
.A..A.
XMAS.S
.X....", 1)]
#[case("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX", 3)]
#[case("....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX", 3)]
    fn test_vertical(#[case] input: &'static str, #[case] expected: usize) {
        assert_eq!(count_vertical(input), expected);
    }

    #[rstest]
    #[case("..X...
.SAMX.
.A..A.
XMAS.S
.X....", 1)]
#[case("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX", 5)]
#[case("....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX", 5)]
    fn test_diagonal_ltr(#[case] input: &'static str, #[case] expected: usize) {
        assert_eq!(count_diagonal_ltr(input), expected);
    }

//     #[test]
//     fn test_diagonals() {
//         let input = "ABCDEFG
// XABCDEF
// YXABCDE
// ZYXABCD";

//         dbg!(count_diagonal_ltr(input));
//         panic!("halt");
//     }

}
