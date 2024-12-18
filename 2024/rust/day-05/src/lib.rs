use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use ornaments::{AocError, Solution};

pub use crate::Day5 as Day;

pub struct Day5 {
    rules: HashMap<&'static str, HashSet<&'static str>>,
    updates: &'static str,
}

impl Day5 {
    fn is_valid_path(&self, segments: &[&str]) -> bool {
        segments.windows(2).all(|pair| {
            let (current, next) = (pair[0], pair[1]);
            self.rules.get(current)
                .map_or(false, |rules| rules.contains(next))
        })
    }

    fn process_updates<F>(&self, handle_valid: F) -> usize 
        where 
            F: Fn(bool, &str) -> Option<String>
        {
            self.updates.lines()
                .filter_map(|update| {
                    let segments: Vec<_> = update.split(",").collect();
                    let is_valid = self.is_valid_path(&segments);
                    handle_valid(is_valid, update)
                        .map(|result| process_update(&result))
                })
                .sum()
        }
}

impl Solution for Day5 {
    type Output = usize;
    // not sure if this is correct
    type Item = &'static str;

    fn parse(input: &str) -> Self {
        let input = input.to_string();
        // leak the string to make it static
        let input: &'static str = Box::leak(input.into_boxed_str());

        //                                     v `input` escapes the associated function body here; argument requires that `'1` must outlive `'static`
        let (rules_str, updates) = input.split("\n\n").collect_tuple().unwrap();

        let rules = rules_str.lines()
            .fold(HashMap::new(), | mut rules, rule | {
                let (key, value) = rule.split("|").collect_tuple().unwrap();

                rules.entry(key).or_insert(HashSet::new()).insert(value);

                rules
            });

        Self {
            rules,
            updates,
        }
    }

    fn part1(&mut self) -> Result<Self::Output, AocError> {
        let output = self.process_updates(|is_valid, update| {
            if is_valid { Some(update.to_string()) } else { None }
        });
        Ok(output)
    }

    fn part2(&mut self) -> Result<Self::Output, AocError> {
        let output = self.process_updates(|is_valid, update| {
            if !is_valid { Some(fix_order(update, &self.rules)) } else { None }
        });
        Ok(output)
    }
}

fn fix_order(input: &str, rules_map: &HashMap<&str, HashSet<&str>>) -> String {
    let items: Vec<&str> = input.split(',').collect();
    let ordered = valid_sequence(&items, rules_map);
    ordered.join(",")
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

pub fn process_update(input: &str) -> usize {
    input.lines()
        .map(|line| {
            let nums = line.split(',').collect::<Vec<&str>>();

            nums[nums.len()/2].parse::<usize>().unwrap() 
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
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
        assert_eq!("143", Day5::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
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
        assert_eq!("123", Day5::parse(input).solve(Part::Two)?);
        Ok(())
    }
}