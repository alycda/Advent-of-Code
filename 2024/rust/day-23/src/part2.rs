use std::collections::{HashMap, HashSet};

use crate::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let connections = input.lines()
        .fold(HashMap::new(), |mut map, line| {
            let (a, b) = line.split_once("-").unwrap();

            map.entry(a.to_string())
                .or_insert_with(HashSet::new)
                .insert(b.to_string());
            map.entry(b.to_string())
                .or_insert_with(HashSet::new)
                .insert(a.to_string());

            map
        });

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
        assert_eq!("co,de,ka,ta", process(input)?);
        Ok(())
    }
}
