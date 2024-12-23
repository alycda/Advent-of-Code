use std::collections::HashMap;
use itertools::Itertools;

use crate::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut map = HashMap::new();

    let output = input.lines()
        .map(|line| {
            let (a, b) = line.split_once("-").unwrap();

            map.entry(a.to_string())
                .or_insert_with(Vec::new)
                .push(b.to_string());

            // since it's undirected, also add the reverse
            map.entry(b.to_string())
                .or_insert_with(Vec::new)
                .push(a.to_string());

        })
        .count();

    // dbg!(&map);

    // Ok("".to_string())

    // // find the connected trios
    // let mut connected_trios = Vec::new();

    // for (computer, connections) in &map {
    //     if connections.len() >= 2 {
    //         for combo in connections.iter().combinations(2) {
    //             if map.get(combo[0]).unwrap().contains(combo[1]) {
    //                 let mut trio = vec![
    //                     computer.clone(),
    //                     combo[0].clone(),
    //                     combo[1].clone()
    //                 ];
    //                 trio.sort();  // Sort to help with deduplication
    //                 connected_trios.push(trio);
    //             }
    //         }
    //     }
    // }

    // // Deduplicate trios (since A-B-C is same as B-C-A)
    // connected_trios.sort();
    // connected_trios.dedup();

    // // Filter for trios containing 't' or 'T'
    // let t_trios: Vec<_> = connected_trios.into_iter()
    //     .filter(|trio| {
    //         trio.iter().any(|computer| computer.to_lowercase().contains('t'))
    //     })
    //     .collect();

    // Ok(t_trios.len().to_string())

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

    Ok(count.to_string())
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
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
