//! Day 23: LAN Party 

use std::{collections::{HashMap, HashSet}, marker::PhantomData};

use ornaments::{AocError, Solution};

pub struct Part1;
pub struct Part2;

pub use crate::Day23 as Day;

pub struct Day23<T, V>{
    // HashSet<String>
    data: HashMap<String, V>,
    part: PhantomData<T>,
}

// impl<T, V> Solution for Day<T, V> {

// }

impl Solution for Day<Part1, Vec<String>> {
    type Output = usize;
    type Item = ();

    fn parse(input: &str) -> Self {
        let data = input.lines()
            .fold(HashMap::new(), |mut map, line| {
                let (a, b) = line.split_once("-").unwrap();

                map.entry(a.to_string())
                .or_insert_with(Vec::new)
                .push(b.to_string());

                // since it's undirected, also add the reverse
                map.entry(b.to_string())
                    .or_insert_with(Vec::new)
                    .push(a.to_string());

                map
            });

        Self { data, part: PhantomData }
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let map = &self.data;
        let mut trios = Vec::new();
    
        for c1 in map.keys() {
            // Get c1's connections
            for c2 in map.get(c1).unwrap() {
                // Get c2's connections, excluding c1
                if let Some(c2_connections) = map.get(c2) {
                    for c3 in c2_connections {
                        if c3 != c1 && c3 != c2 {
                            // Check if c3 connects back to c1
                            if map.get(c3).unwrap().contains(c1) {
                                let mut trio = vec![c1.clone(), c2.clone(), c3.clone()];
                                trio.sort();
                                trios.push(trio);
                            }
                        }
                    }
                }
            }
        }

        // Deduplicate
        trios.sort();
        trios.dedup();

        // Filter for 't'
        let count = trios.iter()
            .filter(|trio| {
                trio.iter().any(|computer| computer.starts_with('t'))
            })
            .count();

        // todo!("refactoring");

        Ok(count)
    }

    fn part2(&mut self) -> Result<Self::Output, AocError> {
        unimplemented!("Part 2 is not implemented for Day<Part1, V>")
    }
}

impl Solution for Day<Part2, HashSet<String>> {
    type Output = String;
    type Item = ();

    fn parse(input: &str) -> Self {
        // connections
        let mut data: HashMap<String, HashSet<String>> = HashMap::new();

        // Build the graph
        for line in input.lines() {
            let (a, b) = line.split_once("-").unwrap();
            data.entry(a.to_string())
                .or_insert_with(HashSet::new)
                .insert(b.to_string());
            data.entry(b.to_string())
                .or_insert_with(HashSet::new)
                .insert(a.to_string());
        }

        Self { data, part: PhantomData }
    }

    fn part1(&mut self) -> Result<Self::Output, AocError> {
        unimplemented!("Part 1 is not implemented for Day<Part2, V>")
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        let connections = &self.data;
        let mut max_clique = HashSet::new();
    
        // Start with each vertex
        for start in connections.keys() {
            let mut clique = HashSet::new();
            clique.insert(start.clone());
            
            // Get all neighbors
            let mut candidates: HashSet<_> = connections[start].clone();
            
            // Try to add each candidate to our clique
            while !candidates.is_empty() {
                let candidate = candidates.iter().next().unwrap().clone();
                candidates.remove(&candidate);
                
                // Check if candidate is connected to all clique members
                let is_connected_to_all = clique.iter().all(|member| 
                    connections[&candidate].contains(member)
                );
                
                if is_connected_to_all {
                    clique.insert(candidate);
                }
            }
            
            if clique.len() > max_clique.len() {
                max_clique = clique;
            }
        }

        // Convert to sorted Vec and join with commas
        let mut result: Vec<_> = max_clique.into_iter().collect();
        result.sort();
        Ok(result.join(","))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!("7", Day::<Part1, Vec<String>>::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!("co,de,ka,ta", Day::parse(input).solve(Part::Two)?);
        Ok(())
    }
}