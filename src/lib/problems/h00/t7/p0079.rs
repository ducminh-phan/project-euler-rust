//! A common security method used for online banking is to ask the user for
//! three random characters from a passcode. For example, if the passcode was
//! 531278, they may ask for the 2nd, 3rd, and 5th characters; the expected
//! reply would be: 317.
//!
//! The text file, `assets/0079_keylog.txt`, contains fifty successful login
//! attempts.
//!
//! Given that the three characters are always asked for in order, analyse the
//! file so as to determine the shortest possible secret passcode of unknown
//! length.

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::read_file;

pub fn solve() {
    let mut outgoing: HashMap<i8, HashSet<i8>> = HashMap::new();
    let mut incoming: HashMap<i8, HashSet<i8>> = HashMap::new();

    read_file("assets/0079_keylog.txt", '\n')
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect_vec()
        })
        .for_each(|vs| {
            vs.iter().tuple_windows().for_each(|(u, v)| {
                outgoing
                    .entry(*u)
                    .and_modify(|vs| {
                        vs.insert(*v);
                    })
                    .or_insert(HashSet::from([*v]));
                incoming
                    .entry(*v)
                    .and_modify(|us| {
                        us.insert(*u);
                    })
                    .or_insert(HashSet::from([*u]));
            })
        });

    let all_nodes = incoming
        .keys()
        .chain(outgoing.keys())
        .copied()
        .collect::<HashSet<_>>();
    let mut nodes_without_incoming_edges = all_nodes
        .difference(&incoming.keys().copied().collect())
        .copied()
        .collect::<HashSet<_>>();

    // Kahn's_algorithm
    // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
    let mut topological_order = vec![];
    while !nodes_without_incoming_edges.is_empty() {
        let u = *nodes_without_incoming_edges.iter().next().unwrap();
        nodes_without_incoming_edges.remove(&u);

        topological_order.push(u);

        let vs = outgoing
            .get(&u)
            .into_iter()
            .flatten()
            .copied()
            .collect_vec();

        vs.into_iter().for_each(|v| {
            outgoing.entry(u).and_modify(|vs| {
                vs.remove(&v);
            });
            let incoming_nodes = incoming
                .entry(v)
                .and_modify(|us| {
                    us.remove(&u);
                })
                .or_default();

            if incoming_nodes.is_empty() {
                nodes_without_incoming_edges.insert(v);
            }
        })
    }

    let result = topological_order.iter().join("");
    println!("{result}");
}
