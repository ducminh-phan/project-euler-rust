//! In the game, **Monopoly**, the standard board is set up in the following
//! way:
//!
//! ![0084_monopoly_board.png](https://projecteuler.net/resources/images/0084_monopoly_board.png?1678992052)
//!
//! A player starts on the GO square and adds the scores on two 6-sided dice to
//! determine the number of squares they advance in a clockwise direction.
//! Without any further rules we would expect to visit each square with equal
//! probability: 2.5%. However, landing on G2J (Go To Jail), CC (community
//! chest), and CH (chance) changes this distribution.
//!
//! In addition to G2J, and one card from each of CC and CH, that orders the
//! player to go directly to jail, if a player rolls three consecutive doubles,
//! they do not advance the result of their 3rd roll. Instead, they proceed
//! directly to jail.
//!
//! At the beginning of the game, the CC and CH cards are shuffled. When a
//! player lands on CC or CH they take a card from the top of the respective
//! pile and, after following the instructions, it is returned to the bottom
//! of the pile. There are sixteen cards in each pile, but for the purpose of
//! this problem we are only concerned with cards that order a movement; any
//! instruction not concerned with movement will be ignored and the player will
//! remain on the CC/CH square.
//!
//! * Community Chest (2/16 cards):
//!   1. Advance to GO
//!   2. Go to JAIL
//!
//! * Chance (10/16 cards):
//!   1. Advance to GO
//!   2. Go to JAIL
//!   3. Go to C1
//!   4. Go to E3
//!   5. Go to H2
//!   6. Go to R1
//!   7. Go to next R (railway company)
//!   8. Go to next R
//!   9. Go to next U (utility company)
//!   10. Go back 3 squares.
//!
//! The heart of this problem concerns the likelihood of visiting a particular
//! square. That is, the probability of finishing at that square after a roll.
//! For this reason it should be clear that, with the exception of G2J for which
//! the probability of finishing on it is zero, the CH squares will have the
//! lowest probabilities, as 5/8 request a movement to another square, and it is
//! the final square that the player finishes at on each roll that we are
//! interested in. We shall make no distinction between "Just Visiting" and
//! being sent to JAIL, and we shall also ignore the rule about requiring a
//! double to "get out of jail", assuming that they pay to get out on their next
//! turn.
//!
//! By starting at GO and numbering the squares sequentially from 00 to 39 we
//! can concatenate these two-digit numbers to produce strings that correspond
//! with sets of squares.
//!
//! Statistically it can be shown that the three most popular squares, in order,
//! are JAIL (6.24%) = Square 10, E3 (3.18%) = Square 24,
//! and GO (3.09%) = Square 00. So these three most popular squares can be
//! listed with the six-digit modal string: 102400.
//!
//! If, instead of using two 6-sided dice, two 4-sided dice are used, find the
//! six-digit modal string.

use std::collections::HashMap;
use std::sync::LazyLock;

use itertools::{iproduct, Itertools};
use nalgebra::{OMatrix, U40};
use num::Complex;

use crate::utils::parse_env;

pub fn solve() {
    let dice_size: usize = parse_env("DICE_SIZE", 4);

    let markov_process = MarkovProcess::new(dice_size);
    let result = compute(&markov_process.compute_transition_matrix());

    println!("{result}");
}

#[allow(dead_code, clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Labels {
    GO = 0,
    A1,
    CC1,
    A2,
    T1,
    R1,
    B1,
    CH1,
    B2,
    B3,
    JAIL,
    C1,
    U1,
    C2,
    C3,
    R2,
    D1,
    CC2,
    D2,
    D3,
    FP,
    E1,
    CH2,
    E2,
    E3,
    R3,
    F1,
    F2,
    U2,
    F3,
    G2J,
    G1,
    G2,
    CC3,
    G3,
    R4,
    CH3,
    H1,
    T2,
    H2,

    // Special value for the number of labels
    SIZE,
}

static NEXT_RAILWAY: LazyLock<HashMap<usize, usize>> = LazyLock::new(|| {
    HashMap::from([
        (Labels::CH1 as usize, Labels::R2 as usize),
        (Labels::CH2 as usize, Labels::R3 as usize),
        (Labels::CH3 as usize, Labels::R1 as usize),
    ])
});
static NEXT_UTILITY: LazyLock<HashMap<usize, usize>> = LazyLock::new(|| {
    HashMap::from([
        (Labels::CH1 as usize, Labels::U1 as usize),
        (Labels::CH2 as usize, Labels::U2 as usize),
        (Labels::CH3 as usize, Labels::U1 as usize),
    ])
});

type Distribution = Vec<f64>;
type TransitionMatrix = Vec<Distribution>;

struct MarkovProcess {
    dice_size: usize,
}

impl MarkovProcess {
    fn new(dice_size: usize) -> Self {
        Self { dice_size }
    }

    fn cross_sum(&self) -> Vec<usize> {
        iproduct!(1..=self.dice_size, 1..=self.dice_size)
            .map(|(x, y)| x + y)
            .collect()
    }

    fn two_dice_distribution(&self) -> Distribution {
        let mut distribution = vec![0f64; Labels::SIZE as usize];
        self.cross_sum().into_iter().counts().into_iter().for_each(
            |(outcome, frequency)| {
                distribution[outcome] =
                    (frequency as f64) / (self.dice_size.pow(2) as f64);
            },
        );

        distribution
    }

    fn compute_transition_matrix(&self) -> TransitionMatrix {
        let distribution = self.two_dice_distribution();
        (0..Labels::SIZE as usize)
            .map(|idx| {
                let mut dist = distribution.clone();
                dist.rotate_right(idx);
                Self::apply_community_chest(&mut dist);
                Self::apply_chance(&mut dist);
                Self::apply_g2j(&mut dist);

                dist
            })
            .collect()
    }

    fn apply_community_chest(dist: &mut Distribution) {
        [Labels::CC1, Labels::CC2, Labels::CC3]
            .iter()
            .for_each(|&cc| {
                let cc = cc as usize;
                let p = dist[cc];

                dist[cc] *= 14. / 16.;
                dist[Labels::GO as usize] += p / 16.;
                dist[Labels::JAIL as usize] += p / 16.;
            });
    }

    fn apply_chance(dist: &mut Distribution) {
        [Labels::CH1, Labels::CH2, Labels::CH3]
            .iter()
            .for_each(|&ch| {
                let ch = ch as usize;
                let p = dist[ch];

                dist[ch] *= 3. / 8.;

                [
                    Labels::GO as usize,
                    Labels::JAIL as usize,
                    Labels::C1 as usize,
                    Labels::E3 as usize,
                    Labels::H2 as usize,
                    Labels::R1 as usize,
                    ch - 3,
                    NEXT_RAILWAY[&ch],
                    NEXT_RAILWAY[&ch],
                    NEXT_UTILITY[&ch],
                ]
                .iter()
                .for_each(|&card| dist[card] += p / 16.);
            });
    }

    fn apply_g2j(dist: &mut Distribution) {
        dist[Labels::JAIL as usize] += dist[Labels::G2J as usize];
        dist[Labels::G2J as usize] = 0.;
    }
}

fn compute(matrix: &[Vec<f64>]) -> String {
    // from_vec fills the matrix column-by-column, which means we already
    // have the transpose of the transition matrix
    let matrix = OMatrix::<Complex<f64>, U40, U40>::from_vec(
        matrix
            .iter()
            .flatten()
            .copied()
            .map(|re| Complex { re, im: 0. })
            .collect(),
    );

    let schur = matrix.schur();
    let eigenvector = schur.unpack().0.column(0).into_owned();
    let normalized_eigenvector = (eigenvector / eigenvector.sum())
        .into_iter()
        .map(|e| e.re)
        .collect_vec();

    normalized_eigenvector
        .iter()
        .enumerate()
        .k_largest_by(3, |(_, a), (_, b)| a.total_cmp(b))
        .map(|(i, _)| format!("{:02}", i))
        .join("")
}
