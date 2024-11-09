//! In the card game poker, a hand consists of five cards and are ranked, from
//! lowest to highest, in the following way:
//!
//! * **High Card**: Highest value card.
//! * **One Pair**: Two cards of the same value.
//! * **Two Pairs**: Two different pairs.
//! * **Three of a Kind**: Three cards of the same value.
//! * **Straight**: All cards are consecutive values.
//! * **Flush**: All cards of the same suit.
//! * **Full House**: Three of a kind and a pair.
//! * **Four of a Kind**: Four cards of the same value.
//! * **Straight Flush**: All cards are consecutive values of same suit.
//! * **Royal Flush**: Ten, Jack, Queen, King, Ace, in same suit.
//!
//! The cards are valued in the order: 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen,
//! King, Ace.
//!
//! If two players have the same ranked hands then the rank made up of the
//! highest value wins; for example, a pair of eights beats a pair of fives
//! (see example 1 below). But if two ranks tie, for example, both players have
//! a pair of queens, then highest cards in each hand are compared (see example
//! 4 below); if the highest cards tie then the next highest cards are compared,
//! and so on.
//!
//! Consider the following five hands dealt to two players:
//!
//! |**Hand**|   |                          **Player 1**                          |   |                          **Player 2**                           |   |**Winner**|
//! |--------|---|----------------------------------------------------------------|---|-----------------------------------------------------------------|---|----------|
//! | **1**  |   |            5H 5C 6S 7S KD  <br/><br/>Pair of Fives             |   |            2C 3S 8S 8D TD  <br/><br/>Pair of Eights             |   | Player 2 |
//! | **2**  |   |           5D 8C 9S JS AC  <br/><br/>Highest card Ace           |   |          2C 5C 7D 8S QH  <br/><br/>Highest card Queen           |   | Player 1 |
//! | **3**  |   |              2D 9C AS AH AC  <br/><br/>Three Aces              |   |          3D 6D 7D TD QD  <br/><br/>Flush with Diamonds          |   | Player 2 |
//! | **4**  |   |4D 6S 9H QH QC  <br/><br/>Pair of Queens  <br/>Highest card Nine|   |3D 6D 7H QD QS  <br/><br/>Pair of Queens  <br/>Highest card Seven|   | Player 1 |
//! | **5**  |   |  2H 2D 4C 4D 4S  <br/><br/>Full House  <br/>With Three Fours   |   |  3C 3D 3S 9S 9D  <br/><br/>Full House  <br/>with Three Threes   |   | Player 1 |
//!
//! The file, [poker.txt](assets/0054_poker.txt), contains one-thousand random
//! hands dealt to two players. Each line of the file contains ten cards
//! (separated by a single space): the first five are Player 1's cards and the
//! last five are Player 2's cards. You can assume that all hands are valid (no
//! invalid characters or repeated cards), each player's hand is in no specific
//! order, and in each hand there is a clear winner.
//!
//! How many hands does Player 1 win?

use itertools::Itertools;

use crate::utils::read_file;

pub fn solve() {
    let result = read_file("assets/0054_poker.txt", '\n')
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| (parse_hand(&line[..14]), parse_hand(&line[15..])))
        .map(|(h1, h2)| (eval_hand(h1), eval_hand(h2)))
        .filter(|(s1, s2)| s1 > s2)
        .count();
    dbg!(result);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl From<char> for Suit {
    fn from(value: char) -> Self {
        match value {
            'H' => Self::Heart,
            'D' => Self::Diamond,
            'C' => Self::Club,
            'S' => Self::Spade,
            _ => panic!("{}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Card {
    rank: u8,
    suit: Suit,
}

type Hand = [Card; 5];

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        if value.len() != 2 {
            panic!("{}", value);
        }

        let c = value.chars().nth(0).unwrap();
        let rank = match c {
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("{}", c),
        };
        let suit = Suit::from(value.chars().nth(1).unwrap());

        Self { rank, suit }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct RankCount {
    // count has higher priority when comparing
    count: u8,
    rank: u8,
}

struct RankCounts {
    ranks: [u8; 5],
    counts: [u8; 5],
}

impl RankCounts {
    fn from_ranks(ranks: &[u8; 5]) -> Self {
        let counts_map = ranks.iter().counts();
        let mut rank_counts = ranks.map(|r| RankCount {
            rank: r,
            count: counts_map[&r] as u8,
        });
        rank_counts.sort_by(|a, b| b.cmp(a));

        let ranks = rank_counts.map(|rc| rc.rank);
        let counts = rank_counts.map(|rc| rc.count);

        Self { ranks, counts }
    }
}

fn parse_hand(s: &str) -> Hand {
    assert_eq!(s.len(), 14);
    let mut hand: [Option<Card>; 5] = [None; 5];
    for (i, s) in s.split(' ').enumerate() {
        let c = Card::from(s);
        hand[i] = Some(c);
    }

    hand.map(|card| card.unwrap())
}

/// Evaluate hand, the first element in the result is the score with
/// Royal Flush has the score of 9 and High Card has the score of 0. The
/// remaining items are card ranks sorted descending.
fn eval_hand(hand: Hand) -> [u8; 6] {
    let mut ranks = hand.map(|c| c.rank);
    // Reverse sorting
    ranks.sort_by(|a, b| b.cmp(a));

    // Count the ranks and sorted reversely
    let RankCounts { ranks, counts, .. } = RankCounts::from_ranks(&ranks);

    let is_straight = counts.eq(&vec![1; 5][..]) && ranks[0] - ranks[4] == 4;

    let suits = hand.map(|c| c.suit);
    let is_flush = suits.iter().unique().count() == 1;

    let score = if is_straight && is_flush {
        if ranks[0] == 14 {
            // Royal Flush
            9
        } else {
            // Straight Flush
            8
        }
    }
    // Four of a Kind
    else if counts[0] == 4 {
        7
    }
    // Full House
    else if counts[0] == 3 && counts[3] == 2 {
        6
    }
    // Flush
    else if is_flush {
        5
    }
    // Straight
    else if is_straight {
        4
    }
    // Three of a Kind
    else if counts == [3, 3, 3, 1, 1] {
        3
    }
    // Two Pairs
    else if counts == [2, 2, 2, 2, 1] {
        2
    }
    // One Pair
    else if counts == [2, 2, 1, 1, 1] {
        1
    } else {
        0
    };

    let mut evaluation = [0; 6];
    evaluation[0] = score;
    evaluation[1..].copy_from_slice(&ranks);
    evaluation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let cards = "4D 6S 9H QH QC KH AS"
            .split(' ')
            .map(Card::from)
            .collect::<Vec<_>>();

        assert_eq!(
            cards,
            vec![
                Card {
                    rank: 4,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: 6,
                    suit: Suit::Spade,
                },
                Card {
                    rank: 9,
                    suit: Suit::Heart,
                },
                Card {
                    rank: 12,
                    suit: Suit::Heart,
                },
                Card {
                    rank: 12,
                    suit: Suit::Club,
                },
                Card {
                    rank: 13,
                    suit: Suit::Heart,
                },
                Card {
                    rank: 14,
                    suit: Suit::Spade,
                },
            ]
        )
    }

    #[test]
    fn test_parse_hand() {
        let cards = "9H QH QC KH AS"
            .split(' ')
            .map(Card::from)
            .collect::<Vec<_>>();

        assert_eq!(
            cards,
            [
                Card {
                    rank: 9,
                    suit: Suit::Heart,
                },
                Card {
                    rank: 12,
                    suit: Suit::Heart,
                },
                Card {
                    rank: 12,
                    suit: Suit::Club,
                },
                Card {
                    rank: 13,
                    suit: Suit::Heart,
                },
                Card {
                    rank: 14,
                    suit: Suit::Spade,
                },
            ]
        )
    }

    #[test]
    fn test_eval_hand() {
        assert_eq!(
            eval_hand(parse_hand("TH JH QH KH AH")),
            [9, 14, 13, 12, 11, 10]
        );

        assert_eq!(
            eval_hand(parse_hand("TH JH QH KH 9H")),
            [8, 13, 12, 11, 10, 9]
        );

        // Four of a Kind
        assert_eq!(eval_hand(parse_hand("9H 9D 9C 9S 5H")), [7, 9, 9, 9, 9, 5]);
        assert_eq!(
            eval_hand(parse_hand("9H 9D 9C 9S AH")),
            [7, 9, 9, 9, 9, 14]
        );

        // Full House
        assert_eq!(eval_hand(parse_hand("9H 9D 9C 8S 8H")), [6, 9, 9, 9, 8, 8]);
        assert_eq!(
            eval_hand(parse_hand("9H 9D 9C TS TH")),
            [6, 9, 9, 9, 10, 10]
        );

        // Flush
        assert_eq!(
            eval_hand(parse_hand("2H 3H 9H KH AH")),
            [5, 14, 13, 9, 3, 2]
        );

        // Straight
        assert_eq!(eval_hand(parse_hand("2H 3C 4H 5S 6D")), [4, 6, 5, 4, 3, 2]);

        // Three of a Kind
        assert_eq!(
            eval_hand(parse_hand("9H 9D 9C TS AH")),
            [3, 9, 9, 9, 14, 10]
        );

        // Two Pairs
        assert_eq!(
            eval_hand(parse_hand("9H 9D 8C 8S AH")),
            [2, 9, 9, 8, 8, 14]
        );

        // One Pair
        assert_eq!(
            eval_hand(parse_hand("9H 9D 8C 7S AH")),
            [1, 9, 9, 14, 8, 7]
        );
    }
}
