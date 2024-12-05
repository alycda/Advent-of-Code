use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (rules, updates) = input.split("\n\n").collect_tuple().unwrap();

    let rules_map = rules.lines()
        // .inspect(|rule| {
        //     dbg!(rule);
        // })
        .fold(HashMap::new(), | mut rules, rule | {
            let (key, value) = rule.split("|").collect_tuple().unwrap();

            rules.entry(key).or_insert(HashSet::new()).insert(value);

            rules
        });

    // dbg!(&rules_map);

    let output = updates.lines()
        .enumerate()
        .filter_map(|(row, update)| {
            let segments: Vec<_> = update.split(",").collect();
            // Check if path is valid by looking at consecutive pairs
            let is_valid = segments.windows(2).all(|pair| {
                let (current, next) = (pair[0], pair[1]);
                rules_map.get(current)
                    .map_or(false, |rules| rules.contains(next))
            });
            
            if is_valid {
                Some((row, update))
            } else {
                None
            }
        })
        .map(|(_, x)| process_update(x).unwrap().parse::<usize>().unwrap())
        .sum::<usize>();

    // let output = updates.lines()
    //     .enumerate()
    //     .filter_map(|(row, update)| -> Option<(usize, &str)> {
    //         let mut peekable = update.split(",").peekable();
    //         let value = rules_map.clone();
            
    //         update.split(",")
    //             .enumerate()
    //             // .try_fold(0, |acc, (idx, page)| {
    //             .map(|(idx, page)| {
    //                 peekable.next();
                    
    //                 if let Some(next) = peekable.peek() {
    //                     if let Some(rules) = value.get(page) {
    //                         if rules.contains(&*next) {
    //                             return Some((row, update));
    //                         } else {
    //                             None
    //                         }
    //                     } else {
    //                         None
    //                     }
    //                 } else {
    //                     return Some((row, update));
    //                 }
    //             })
            
    //         // None::<(_, &str)>
    //     })
    //     // .filter_map(|update| {
    //     //     // dbg!(update);

    //     //     let mut peekable = update.split(",").peekable();

    //     //     let value = rules_map.clone();
    //     //     update.split(",")
    //     //         .enumerate()
    //     //         // .take(peekable.len() - 1)
    //     //         .map({
    //     //         move |(idx, page)| {
    //     //             peekable.next();
    //     //             dbg!(idx, page, peekable.peek());
    //     //             // dbg!(&rules_map.get(page));

    //     //             if let Some(_) = peekable.peek() {
    //     //                 if let Some(rules) = &value.get(page) {
    //     //                     if let Some(next) = peekable.peek() {
    //     //                         dbg!(rules.contains(&*next));
    //     //                     } else {
    //     //                         // are we at the end?
    //     //                         dbg!(idx);
    //     //                     }
    //     //                     // dbg!(rules);
    //     //                     // dbg!(rules.contains(&*peekable.peek().unwrap()));
    //     //                 } else {
    //     //                     dbg!("No rules found for page {page} with index {idx}");
    //     //                     return None
    //     //                 }
    //     //             } else {
    //     //                 // are we at the end?
    //     //                 dbg!(idx);
    //     //             }

    //     //             Some("update")

    //     //         }
    //     //         })
    //     //         // .count();
    //     // })
    //     // .count();
    //     // .inspect(|(row, x)| {
    //     //     dbg!(row, x);
    //     // })
    //     .map(|(_, x): (_, &str)| process_update(x).unwrap().parse::<usize>().unwrap() )
    //     .sum::<usize>();

    Ok(output.to_string())
}

pub fn process_update(input: &str) -> miette::Result<String, AocError> {
    let output = input.lines()
        .map(|line| {
            let nums = line.split(',').collect::<Vec<&str>>();

            nums[nums.len()/2].parse::<usize>().unwrap() 
        })
        .inspect(|middle_number| {
            dbg!(middle_number);
        })
        .sum::<usize>();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("75,47,61,53,29
97,61,53,29,13
75,29,13", "143")]
    #[case("75,47,61,53,29
97,61,53,29,13
75,29,13", "143")]
    fn test_cases(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_update(input).unwrap(), expected);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
