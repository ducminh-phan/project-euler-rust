//! NOTE: This problem is a significantly more challenging version of Problem 81.
//!
//! In the `5` by `5` matrix below, the minimal path sum from the top left to
//! the bottom right, by moving left, right, up, and down, is indicated in bold
//! red and is equal to `2297`.
//!
//! ```text
//! *131*   673   *234*  *103* *018*
//! *201*  *096*  *342*   965  *150*
//!  630    803    746   *422* *111*
//!  537    699    497   *121*  956
//!  805    732    524   *037* *331*
//! ```
//!
//! Find the minimal path sum from the top left to the bottom right by moving
//! left, right, up, and down in `assets/0082_matrix.txt`, a 31K text file
//! containing an `80` by `80` matrix.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use itertools::Itertools;

use crate::utils::read_file;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve() -> crate::Answer {
    let matrix = read_file("assets/0083_matrix.txt", '\n')
        .iter()
        .map(|line| {
            line.split(',').flat_map(|w| w.parse::<u64>()).collect_vec()
        })
        .collect_vec();
    let n = matrix.len();

    shortest_path(&matrix, 0, n * n - 1).unwrap().into()
}

// Dijkstra's shortest path algorithm.
// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(
    matrix: &Vec<Vec<u64>>,
    start: usize,
    goal: usize,
) -> Option<u64> {
    let n = matrix.len();

    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = vec![u64::MAX; n * n];
    let mut heap = BinaryHeap::new();

    // We're at `start`
    dist[start] = matrix[start / n][start % n];
    heap.push(State {
        cost: dist[start],
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for (n_position, n_weight) in get_neighbors(matrix, position) {
            let next = State {
                cost: cost + n_weight,
                position: n_position,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

#[allow(clippy::ptr_arg)]
fn get_neighbors(matrix: &Vec<Vec<u64>>, idx: usize) -> Vec<(usize, u64)> {
    let n = matrix.len();
    let (r, c) = (idx / n, idx % n);

    let mut neighbors = vec![];
    if r > 0 {
        neighbors.push((r - 1, c));
    }
    if r < n - 1 {
        neighbors.push((r + 1, c))
    }
    if c > 0 {
        neighbors.push((r, c - 1));
    }
    if c < n - 1 {
        neighbors.push((r, c + 1));
    }

    neighbors
        .iter()
        .map(|(r, c)| (r * n + c, matrix[*r][*c]))
        .collect()
}
