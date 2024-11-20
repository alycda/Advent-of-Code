use crate::custom_error::EcError;

use std::collections::HashMap;

// WORDS:LOR,LL,SI,OR,EN,ON,UM

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, EcError> {
    let mut rune_count = HashMap::new();

    let peek = input.chars().collect::<Vec<_>>();
    let mut peek = peek.windows(2).peekable();

    // peek.clone()
    //     .collect::<Vec<_>>()
    //     .windows(2)
    //     .map(|chars| {
    //         let s = dbg!(chars.iter().collect::<String>());

    //         match s.as_str() {
    //             "LL" | "SI" | "OR" | "EN" | "ON" | "UM" => {
    //                 *rune_count.entry(s).or_insert(0) += 1;
    //             }
    //             "LO" => {
    //                 dbg!(peek.next());
    //             }
    //             _ => {}
    //         }

    //         0
    //     })
    //     .sum::<u32>();

    while let Some(c) = peek.next() {
        let s = dbg!(c.iter().collect::<String>());

        match s.as_str() {
            "LL" | "SI" | "OR" | "EN" | "ON" | "UM" => {
                *rune_count.entry(s).or_insert(0) += 1;
            }
            "LO" => {
                // dbg!(peek.peek());
                if let Some(p) = peek.peek() {
                    dbg!(p[1]);

                    if p[1] == 'R' {
                        *rune_count.entry("LOR".to_string()).or_insert(0) += 1;
                    }
                }
            }
            _ => {}
        }

        // if let Some(p) = peek.peek() {
        //     dbg!(p[1]);

        // let s = dbg!(format!("{c}{p}"));

        // match s.as_str() {
        //     "LL" | "SI" | "OR" | "EN" | "ON" | "UM" => {
        //         *rune_count.entry(s).or_insert(0) += 1;
        //     }
        //     "LO" => {
        //         dbg!(peek.peek());
        //     }
        //     _ => {}
        // }
        // }
    }

    dbg!(&rune_count);

    Ok(rune_count.values().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    // #[test]
    // fn test_process() -> miette::Result<()> {
    //     let input = "AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE";
    //     assert_eq!("4", process(input)?);
    //     Ok(())
    // }

    #[rstest]
    // #[case("THE FLAME SHIELDED THE HEART OF THE KINGS", 3)]
    // #[case("POWE PO WER P OWE R", 2)]
    // #[case("THERE IS THE END", 3)]
    #[case("LOREM", 2)]
    fn multi_test(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, process(input).unwrap().parse::<u32>().unwrap());
    }
}
