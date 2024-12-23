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
// pub fn process(input: &str) -> miette::Result<String, AocError> {
//     let mut map = HashMap::new();

//     let output = input.lines()
//         .map(|line| {
//             let (a, b) = line.split_once("-").unwrap();

//             map.entry(a.to_string())
//                 .or_insert_with(Vec::new)
//                 .push(b.to_string());

//             // since it's undirected, also add the reverse
//             map.entry(b.to_string())
//                 .or_insert_with(Vec::new)
//                 .push(a.to_string());

//         })
//         .count();

//     // Convert to HashSet representation first
//     let connections: HashMap<String, HashSet<String>> = map
//         .into_iter()
//         .map(|(computer, vec)| (computer, vec.into_iter().collect()))
//         .collect();

//     // dbg!(&connections);

//     // // let mut largest_group = HashSet::new();

//     // // for (computer, connected_to) in &connections {
//     // //     // Start with this computer and all its connections
//     // //     let mut candidate_group = connected_to.clone();
//     // //     candidate_group.insert(computer.clone());

//     // //     // Remove computers that aren't connected to everyone else
//     // //     candidate_group = candidate_group
//     // //         .into_iter()
//     // //         .filter(|c1| {
//     // //             candidate_group.iter().all(|c2| {
//     // //                 c1 == c2 || connections.get(c1).unwrap().contains(c2)
//     // //             })
//     // //         })
//     // //         .collect();

//     // //     if candidate_group.len() > largest_group.len() {
//     // //         largest_group = candidate_group;
//     // //     }   
//     // // }

//     // // let mut largest_group = HashSet::new();

//     // // for (computer, connected_to) in &connections {
//     // //     // Start with this computer and all its connections
//     // //     let mut candidate_group = connected_to.clone();
//     // //     candidate_group.insert(computer.clone());
        
//     // //     // Get a vector of computers that aren't fully connected
//     // //     let to_remove: Vec<_> = candidate_group.iter()
//     // //         .filter(|c1| {
//     // //             !candidate_group.iter().all(|c2| {
//     // //                 *c1 == c2 || connections.get(*c1).unwrap().contains(c2)
//     // //             })
//     // //         })
//     // //         .cloned()
//     // //         .collect();

//     // //     // Remove the non-fully-connected computers
//     // //     for computer in to_remove {
//     // //         candidate_group.remove(&computer);
//     // //     }

//     // //     if candidate_group.len() > largest_group.len() {
//     // //         largest_group = candidate_group;
//     // //     }
//     // // }

//     // let mut largest_group = HashSet::new();
//     // let mut largest_size = 0;  // Track size for debugging

//     // // for (computer, connected_to) in &connections {
//     // //     let mut candidate_group = connected_to.clone();
//     // //     candidate_group.insert(computer.clone());
        
//     // //     println!("Starting with computer {} and group: {:?}", computer, candidate_group);
        
//     // //     let to_remove: Vec<_> = candidate_group.iter()
//     // //         .filter(|c1| {
//     // //             let all_connected = candidate_group.iter().all(|c2| {
//     // //                 let is_connected = *c1 == c2 || connections.get(*c1).unwrap().contains(c2);
//     // //                 println!("Checking {} -> {}: {}", c1, c2, is_connected);
//     // //                 is_connected
//     // //             });
//     // //             !all_connected
//     // //         })
//     // //         .cloned()
//     // //         .collect();

//     // //     println!("Removing: {:?}", to_remove);
        
//     // //     for computer in to_remove {
//     // //         candidate_group.remove(&computer);
//     // //     }

//     // //     println!("After removals: {:?}", candidate_group);

//     // //     if candidate_group.len() > largest_size {
//     // //         largest_size = candidate_group.len();
//     // //         largest_group = candidate_group;
//     // //         println!("New largest group found: {:?}", largest_group);
//     // //     }
//     // //     println!("-------------------");
//     // // }

//     // for (computer, connected_to) in &connections {
//     //     let mut candidate_group = connected_to.clone();
//     //     candidate_group.insert(computer.clone());
        
//     //     // Keep only computers that are connected to ALL others
//     //     let to_remove: Vec<_> = candidate_group.iter()
//     //         .filter(|c1| {
//     //             !candidate_group.iter().all(|c2| {
//     //                 if *c1 == c2 { 
//     //                     return true; 
//     //                 }
//     //                 // Check both directions of connection
//     //                 connections.get(*c1).unwrap().contains(c2) &&
//     //                 connections.get(c2).unwrap().contains(*c1)
//     //             })
//     //         })
//     //         .cloned()
//     //         .collect();
    
//     //     for computer in to_remove {
//     //         candidate_group.remove(&computer);
//     //     }
    
//     //     if candidate_group.len() > largest_group.len() {
//     //         largest_group = candidate_group;
//     //     }
//     // }

//     // let mut possibilities: HashSet<HashSet<String>> = HashSet::new();

//     // // First find all fully connected groups of 3
//     // for (c1, connected_to) in &connections {
//     //     for c2 in connected_to {
//     //         // Get c2's connections, excluding c1
//     //         if let Some(c2_connections) = connections.get(c2) {
//     //             for c3 in c2_connections {
//     //                 if c3 != c1 && c3 != c2 {
//     //                     // Check if c3 connects back to c1
//     //                     if connections.get(c3).unwrap().contains(c1) {
//     //                         let mut group = HashSet::new();
//     //                         group.insert(c1.clone());
//     //                         group.insert(c2.clone());
//     //                         group.insert(c3.clone());
//     //                         possibilities.insert(group);
//     //                     }
//     //                 }
//     //             }
//     //         }
//     //     }
//     // }

//     // // Now try to expand each group
//     // let mut largest_group = HashSet::new();
//     // for group in &possibilities {
//     //     let mut expanded_group = group.clone();
        
//     //     // Try adding each computer that's connected to all current members
//     //     for (computer, connections) in &connections {
//     //         if !expanded_group.contains(computer) {
//     //             if expanded_group.iter().all(|member| 
//     //                 connections.contains(member) && 
//     //                 connections.get(member).unwrap().contains(computer)
//     //             ) {
//     //                 expanded_group.insert(computer.clone());
//     //             }
//     //         }
//     //     }
        
//     //     if expanded_group.len() > largest_group.len() {
//     //         largest_group = expanded_group;
//     //     }
//     // }

//     let mut possibilities: Vec<HashSet<String>> = Vec::new();

// // First find all fully connected groups of 3
// for (c1, connected_to) in &connections {
//     for c2 in connected_to {
//         // Get c2's connections, excluding c1
//         if let Some(c2_connections) = connections.get(c2) {
//             for c3 in c2_connections {
//                 if c3 != c1 && c3 != c2 {
//                     // Check if c3 connects back to c1
//                     if connections.get(c3).unwrap().contains(c1) {
//                         let mut group = HashSet::new();
//                         group.insert(c1.clone());
//                         group.insert(c2.clone());
//                         group.insert(c3.clone());
//                         possibilities.push(group);
//                     }
//                 }
//             }
//         }
//     }
// }

//     // // Now try to expand each group
//     // let mut largest_group = HashSet::new();
//     // for group in &possibilities {
//     //     let mut expanded_group = group.clone();
        
//     //     // Try adding each computer that's connected to all current members
//     //     for (computer, connections) in &connections {
//     //         if !expanded_group.contains(computer) {
//     //             if expanded_group.iter().all(|member| 
//     //                 connections.contains(member) && 
//     //                 connections.get(member).unwrap().contains(computer)
//     //             ) {
//     //                 expanded_group.insert(computer.clone());
//     //             }
//     //         }
//     //     }
        
//     //     if expanded_group.len() > largest_group.len() {
//     //         largest_group = expanded_group;
//     //     }
//     // }

// //     let mut largest_group = HashSet::new();

// // for (computer, connections) in &connections {
// //     let mut candidate_group = HashSet::new();
// //     candidate_group.insert(computer.clone());
    
// //     // Start with all connections as candidates
// //     let mut potential_members: HashSet<_> = connections.clone();
    
// //     // Keep only members that connect to everyone else
// //     let members: HashSet<_> = potential_members
// //         .into_iter()
// //         .filter(|member| {
// //             // Check if this member connects to all other candidates
// //             let member_connections = connections.get(member).unwrap();
// //             potential_members.iter().all(|other| {
// //                 member == other || 
// //                 member_connections.contains(other)
// //             }) && member_connections.contains(computer)
// //         })
// //         .collect();
    
// //     if !members.is_empty() {
// //         candidate_group.extend(members);
// //         if candidate_group.len() > largest_group.len() {
// //             largest_group = candidate_group;
// //         }
// //     }
// // }

// let mut largest_group = HashSet::new();

// for (computer, connections) in &connections {
//     let mut candidate_group = HashSet::new();
//     candidate_group.insert(computer.clone());
    
//     // Convert connections to Vec for iteration
//     let potential_members: Vec<_> = connections.iter().cloned().collect();
    
//     // Keep only members that connect to everyone else
//     let members: HashSet<_> = potential_members
//         .iter()
//         .filter(|member| {
//             // Check if this member connects to all other potential members
//             if let Some(member_connections) = connections.get(*member) {
//                 potential_members.iter().all(|other| {
//                     *member == other || 
//                     member_connections.contains(other)
//                 }) && member_connections.contains(computer)
//             } else {
//                 false
//             }
//         })
//         .cloned()
//         .collect();
    
//     if !members.is_empty() {
//         candidate_group.extend(members);
//         if candidate_group.len() > largest_group.len() {
//             largest_group = candidate_group;
//         }
//     }
// }

//     dbg!(&largest_group);

//     panic!("halt");

//     // // dbg!(&map);

//     // // Ok("".to_string())

//     // // // find the connected trios
//     // // let mut connected_trios = Vec::new();

//     // // for (computer, connections) in &map {
//     // //     if connections.len() >= 2 {
//     // //         for combo in connections.iter().combinations(2) {
//     // //             if map.get(combo[0]).unwrap().contains(combo[1]) {
//     // //                 let mut trio = vec![
//     // //                     computer.clone(),
//     // //                     combo[0].clone(),
//     // //                     combo[1].clone()
//     // //                 ];
//     // //                 trio.sort();  // Sort to help with deduplication
//     // //                 connected_trios.push(trio);
//     // //             }
//     // //         }
//     // //     }
//     // // }

//     // // // Deduplicate trios (since A-B-C is same as B-C-A)
//     // // connected_trios.sort();
//     // // connected_trios.dedup();

//     // // // Filter for trios containing 't' or 'T'
//     // // let t_trios: Vec<_> = connected_trios.into_iter()
//     // //     .filter(|trio| {
//     // //         trio.iter().any(|computer| computer.to_lowercase().contains('t'))
//     // //     })
//     // //     .collect();

//     // // Ok(t_trios.len().to_string())

//     // let mut trios = Vec::new();
    
//     // for c1 in map.keys() {
//     //     // Get c1's connections
//     //     for c2 in map.get(c1).unwrap() {
//     //         // Get c2's connections, excluding c1
//     //         if let Some(c2_connections) = map.get(c2) {
//     //             for c3 in c2_connections {
//     //                 if c3 != c1 && c3 != c2 {
//     //                     // Check if c3 connects back to c1
//     //                     if map.get(c3).unwrap().contains(c1) {
//     //                         let mut trio = vec![c1.clone(), c2.clone(), c3.clone()];
//     //                         trio.sort();
//     //                         trios.push(trio);
//     //                     }
//     //                 }
//     //             }
//     //         }
//     //     }
//     // }

//     // // Deduplicate
//     // trios.sort();
//     // trios.dedup();

//     // // Filter for 't'
//     // let count = trios.iter()
//     //     .filter(|trio| {
//     //         trio.iter().any(|computer| computer.starts_with('t'))
//     //     })
//     //     .count();

//     // Ok(count.to_string())
// }

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
