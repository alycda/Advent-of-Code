use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{custom_error::AocError, process_update};

fn invert_dependencies<'a>(rules: &'a HashMap<&'a str, HashSet<&'a str>>) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut inverted = HashMap::new();
    
    for (&key, values) in rules {
        // Each key becomes a dependency for each of its values
        for &value in values {
            inverted.entry(value)
                .or_insert_with(HashSet::new)
                .insert(key);
        }
    }
    
    inverted
}

fn valid_sequence<'a>(items: &[&'a str], rules_map: &HashMap<&str, HashSet<&str>>) -> Vec<&'a str> {
    let deps = invert_dependencies(rules_map);
    let mut result = Vec::new();
    let mut remaining: HashSet<&str> = items.iter().copied().collect();
    
    while !remaining.is_empty() {
        // break the borrow: error[E0502]: cannot borrow `remaining` as mutable because it is also borrowed as immutable
        let &next = remaining.iter()
            .find(|&&item| {
                deps.get(item)
                    .map_or(true, |deps| 
                        deps.iter().all(|dep| !remaining.contains(dep)))
            })
            .expect("Should find an item with satisfied deps");
            
        result.push(next);
        remaining.remove(next);
    }
    
    result
}

fn fix_order(input: &str, rules_map: &HashMap<&str, HashSet<&str>>) -> String {
    let items: Vec<&str> = input.split(',').collect();
    let ordered = valid_sequence(&items, rules_map);
    ordered.join(",")
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (rules, updates) = input.split("\n\n").collect_tuple().unwrap();

    let rules_map = rules.lines()
        .fold(HashMap::new(), | mut rules, rule | {
            let (key, value) = rule.split("|").collect_tuple().unwrap();

            rules.entry(key).or_insert(HashSet::new()).insert(value);

            rules
        });

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
            
            match is_valid {
                false => Some((row, fix_order(update, &rules_map))),
                true => None,
            }
        })
        .map(|(_, x)| process_update(&x))
        .sum::<usize>();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

//     use rstest::rstest;

//     #[rstest]
//     #[case("75,97,47,61,53
// 61,13,29
// 97,13,75,29,47", 123)]
//     fn test_cases(#[case] input: &str, #[case] expected: usize) {
//         assert_eq!(process_update(fix_order(input)), expected);
//     }

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
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
