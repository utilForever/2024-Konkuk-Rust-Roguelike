use std::{cmp::Ordering, collections::HashSet};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum CardSuit {
    CLOVER,
    HEART,
    DIAMOND,
    SPADE,
}

impl From<&str> for CardSuit {
    fn from(value: &str) -> Self {
        match value {
            "C" => CardSuit::CLOVER,
            "H" => CardSuit::HEART,
            "D" => CardSuit::DIAMOND,
            "S" => CardSuit::SPADE,
            _ => panic!("Invalid suit value."),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
enum CardRank {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}

impl From<&str> for CardRank {
    fn from(value: &str) -> Self {
        match value {
            "2" => CardRank::TWO,
            "3" => CardRank::THREE,
            "4" => CardRank::FOUR,
            "5" => CardRank::FIVE,
            "6" => CardRank::SIX,
            "7" => CardRank::SEVEN,
            "8" => CardRank::EIGHT,
            "9" => CardRank::NINE,
            "10" => CardRank::TEN,
            "J" => CardRank::JACK,
            "Q" => CardRank::QUEEN,
            "K" => CardRank::KING,
            "A" => CardRank::ACE,
            _ => panic!("Invalid rank value."),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Card {
    suit: CardSuit,
    rank: CardRank,
}

impl Card {
    fn new(s: &str) -> Card {
        let mut s_iter = s.chars();
        let suit: &str = &s_iter.next_back().unwrap().to_string();
        let rank: &str = s_iter.as_str();
        Card {
            suit: CardSuit::from(suit),
            rank: CardRank::from(rank),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.rank.partial_cmp(&other.rank).unwrap() {
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => Some(self.suit.cmp(&other.suit)),
        }
    }
}

#[derive(PartialEq, Eq)]
enum HandRanking {
    HighCard([CardRank; 5]),
    OnePair([CardRank; 4]),
    TwoPair([CardRank; 3]),
    ThreeOfAKind([CardRank; 3]),
    Straight(CardRank),
    Flush([CardRank; 5]),
    FullHouse([CardRank; 2]),
    FourOfAKind([CardRank; 2]),
    StraightFlush(CardRank),
}

impl HandRanking {
    fn rank(&self) -> i32 {
        match *self {
            HandRanking::HighCard(_) => 0,
            HandRanking::OnePair(_) => 1,
            HandRanking::TwoPair(_) => 2,
            HandRanking::ThreeOfAKind(_) => 3,
            HandRanking::Straight(_) => 4,
            HandRanking::Flush(_) => 5,
            HandRanking::FullHouse(_) => 6,
            HandRanking::FourOfAKind(_) => 7,
            HandRanking::StraightFlush(_) => 8,
        }
    }
}

fn cmp_ranks<const N: usize>(&a: &[CardRank; N], &b: &[CardRank; N]) -> Ordering {
    for idx in 0..N {
        match a[idx].partial_cmp(&b[idx]).unwrap() {
            Ordering::Greater => {
                return Ordering::Greater;
            }
            Ordering::Less => {
                return Ordering::Less;
            }
            Ordering::Equal => (),
        };
    }
    Ordering::Equal
}

impl PartialOrd for HandRanking {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&HandRanking::HighCard(self_numbers), &HandRanking::HighCard(other_numbers)) => {
                Some(cmp_ranks(&self_numbers, &other_numbers))
            }
            (&HandRanking::OnePair(self_numbers), &HandRanking::OnePair(other_numbers)) => {
                Some(cmp_ranks(&self_numbers, &other_numbers))
            }
            (&HandRanking::TwoPair(self_numbers), &HandRanking::TwoPair(other_numbers)) => {
                Some(cmp_ranks(&self_numbers, &other_numbers))
            }
            (
                &HandRanking::ThreeOfAKind(self_numbers),
                &HandRanking::ThreeOfAKind(other_numbers),
            ) => Some(cmp_ranks(&self_numbers, &other_numbers)),
            (&HandRanking::Straight(self_number), &HandRanking::Straight(other_number)) => {
                self_number.partial_cmp(&other_number)
            }
            (&HandRanking::Flush(self_numbers), &HandRanking::Flush(other_numbers)) => {
                Some(cmp_ranks(&self_numbers, &other_numbers))
            }
            (&HandRanking::FullHouse(self_numbers), &HandRanking::FullHouse(other_numbers)) => {
                Some(cmp_ranks(&self_numbers, &other_numbers))
            }
            (&HandRanking::FourOfAKind(self_numbers), &HandRanking::FourOfAKind(other_numbers)) => {
                Some(cmp_ranks(&self_numbers, &other_numbers))
            }
            (
                &HandRanking::StraightFlush(self_number),
                &HandRanking::StraightFlush(other_number),
            ) => self_number.partial_cmp(&other_number),
            _ => Some(self.rank().cmp(&other.rank())),
        }
    }
}

#[derive(Debug)]
struct Hand<'a> {
    orig: &'a str,
    cards: [Card; 5],
}

impl<'a> Hand<'a> {
    fn new(cards: &'a str) -> Hand {
        let card_collection: Vec<Card> = cards
            .split_ascii_whitespace()
            .map(|s| Card::new(s))
            .take(5)
            .collect();
        Hand {
            orig: cards,
            cards: card_collection.try_into().unwrap(),
        }
    }

    fn get_hand_rank(&self) -> HandRanking {
        let mut single_counted_set: HashSet<CardRank> = HashSet::new();
        let mut double_counted_set: HashSet<CardRank> = HashSet::new();
        let mut triple_counted_set: HashSet<CardRank> = HashSet::new();
        let mut quadra_counted_set: HashSet<CardRank> = HashSet::new();
        let mut suit_count = vec![0, 0, 0, 0];
        let mut rank_vec: Vec<CardRank> = Vec::new();
        for card in &self.cards {
            suit_count[card.suit as usize] += 1;
            rank_vec.push(card.rank);
            if triple_counted_set.take(&card.rank).is_some() {
                quadra_counted_set.insert(card.rank);
            } else if double_counted_set.take(&card.rank).is_some() {
                triple_counted_set.insert(card.rank);
            } else if single_counted_set.take(&card.rank).is_some() {
                double_counted_set.insert(card.rank);
            } else {
                single_counted_set.insert(card.rank);
            }
        }
        let mut single_counted_vec: Vec<CardRank> = Vec::from_iter(single_counted_set);
        single_counted_vec.sort();
        if suit_count.iter().position(|&v| v == 5).is_some() {
            match single_counted_vec[..] {
                [CardRank::TEN, CardRank::JACK, CardRank::QUEEN, CardRank::KING, CardRank::ACE] => {
                    HandRanking::StraightFlush(CardRank::ACE)
                }
                [CardRank::NINE, CardRank::TEN, CardRank::JACK, CardRank::QUEEN, CardRank::KING] => {
                    HandRanking::StraightFlush(CardRank::KING)
                }
                [CardRank::EIGHT, CardRank::NINE, CardRank::TEN, CardRank::JACK, CardRank::QUEEN] => {
                    HandRanking::StraightFlush(CardRank::QUEEN)
                }
                [CardRank::SEVEN, CardRank::EIGHT, CardRank::NINE, CardRank::TEN, CardRank::JACK] => {
                    HandRanking::StraightFlush(CardRank::JACK)
                }
                [CardRank::SIX, CardRank::SEVEN, CardRank::EIGHT, CardRank::NINE, CardRank::TEN] => {
                    HandRanking::StraightFlush(CardRank::TEN)
                }
                [CardRank::FIVE, CardRank::SIX, CardRank::SEVEN, CardRank::EIGHT, CardRank::NINE] => {
                    HandRanking::StraightFlush(CardRank::NINE)
                }
                [CardRank::FOUR, CardRank::FIVE, CardRank::SIX, CardRank::SEVEN, CardRank::EIGHT] => {
                    HandRanking::StraightFlush(CardRank::EIGHT)
                }
                [CardRank::THREE, CardRank::FOUR, CardRank::FIVE, CardRank::SIX, CardRank::SEVEN] => {
                    HandRanking::StraightFlush(CardRank::SEVEN)
                }
                [CardRank::TWO, CardRank::THREE, CardRank::FOUR, CardRank::FIVE, CardRank::SIX] => {
                    HandRanking::StraightFlush(CardRank::SIX)
                }
                [CardRank::TWO, CardRank::THREE, CardRank::FOUR, CardRank::FIVE, CardRank::ACE] => {
                    HandRanking::StraightFlush(CardRank::FIVE)
                }
                _ => {
                    single_counted_vec.reverse();
                    HandRanking::Flush(single_counted_vec[..].try_into().unwrap())
                }
            }
        } else if !quadra_counted_set.is_empty() {
            HandRanking::FourOfAKind([
                *quadra_counted_set.iter().next().unwrap(),
                single_counted_vec[0],
            ])
        } else if !triple_counted_set.is_empty() {
            let triple_number: CardRank = *triple_counted_set.iter().next().unwrap();
            if !double_counted_set.is_empty() {
                HandRanking::FullHouse([triple_number, *double_counted_set.iter().next().unwrap()])
            } else {
                HandRanking::ThreeOfAKind([
                    triple_number,
                    single_counted_vec[1],
                    single_counted_vec[0],
                ])
            }
        } else if !double_counted_set.is_empty() {
            if double_counted_set.len() == 2 {
                let mut paired_vec: Vec<CardRank> = Vec::from_iter(double_counted_set);
                paired_vec.sort();
                HandRanking::TwoPair([paired_vec[1], paired_vec[0], single_counted_vec[0]])
            } else {
                single_counted_vec.push(*double_counted_set.iter().next().unwrap());
                single_counted_vec.reverse();
                HandRanking::OnePair(single_counted_vec[..].try_into().unwrap())
            }
        } else {
            match single_counted_vec[..] {
                [CardRank::TEN, CardRank::JACK, CardRank::QUEEN, CardRank::KING, CardRank::ACE] => {
                    HandRanking::Straight(CardRank::ACE)
                }
                [CardRank::NINE, CardRank::TEN, CardRank::JACK, CardRank::QUEEN, CardRank::KING] => {
                    HandRanking::Straight(CardRank::KING)
                }
                [CardRank::EIGHT, CardRank::NINE, CardRank::TEN, CardRank::JACK, CardRank::QUEEN] => {
                    HandRanking::Straight(CardRank::QUEEN)
                }
                [CardRank::SEVEN, CardRank::EIGHT, CardRank::NINE, CardRank::TEN, CardRank::JACK] => {
                    HandRanking::Straight(CardRank::JACK)
                }
                [CardRank::SIX, CardRank::SEVEN, CardRank::EIGHT, CardRank::NINE, CardRank::TEN] => {
                    HandRanking::Straight(CardRank::TEN)
                }
                [CardRank::FIVE, CardRank::SIX, CardRank::SEVEN, CardRank::EIGHT, CardRank::NINE] => {
                    HandRanking::Straight(CardRank::NINE)
                }
                [CardRank::FOUR, CardRank::FIVE, CardRank::SIX, CardRank::SEVEN, CardRank::EIGHT] => {
                    HandRanking::Straight(CardRank::EIGHT)
                }
                [CardRank::THREE, CardRank::FOUR, CardRank::FIVE, CardRank::SIX, CardRank::SEVEN] => {
                    HandRanking::Straight(CardRank::SEVEN)
                }
                [CardRank::TWO, CardRank::THREE, CardRank::FOUR, CardRank::FIVE, CardRank::SIX] => {
                    HandRanking::Straight(CardRank::SIX)
                }
                [CardRank::TWO, CardRank::THREE, CardRank::FOUR, CardRank::FIVE, CardRank::ACE] => {
                    HandRanking::Straight(CardRank::FIVE)
                }
                _ => {
                    rank_vec.sort();
                    rank_vec.reverse();
                    HandRanking::HighCard(rank_vec[..].try_into().unwrap())
                }
            }
        }
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.get_hand_rank() == other.get_hand_rank()
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_hand_rank().partial_cmp(&other.get_hand_rank())
    }
}

// Given a list of poker hands, return a list of those hands which win.
//
// Note the type signature: this function should return _the same_ reference to
// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands_vec: Vec<Hand> = hands.into_iter().map(|&s| Hand::new(s)).collect();
    hands_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let highest_hand_rank = hands_vec.last().unwrap().get_hand_rank();
    let mut ret_vec: Vec<&'a str> = Vec::new();
    for idx in 0..hands_vec.len() {
        if hands_vec[idx].get_hand_rank() == highest_hand_rank {
            ret_vec.push(&hands_vec[idx].orig);
        }
    }
    ret_vec
}

fn hs_from<'a>(input: &[&'a str]) -> HashSet<&'a str> {
    let mut hs = HashSet::new();

    for item in input.iter() {
        hs.insert(*item);
    }

    hs
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that the expected output is produced from the given input
    // using the `winning_hands` function.
    //
    // Note that the output can be in any order. Here, we use a HashSet to
    // abstract away the order of outputs.
    fn test<'a, 'b>(input: &[&'a str], expected: &[&'b str]) {
        assert_eq!(hs_from(&winning_hands(input)), hs_from(expected))
    }

    #[test]
    fn test_single_hand_always_wins() {
        test(&["4S 5S 7H 8D JC"], &["4S 5S 7H 8D JC"])
    }

    #[test]
    fn test_duplicate_hands_always_tie() {
        let input = &["3S 4S 5D 6H JH", "3S 4S 5D 6H JH", "3S 4S 5D 6H JH"];
        assert_eq!(&winning_hands(input), input)
    }

    #[test]
    fn test_highest_card_of_all_hands_wins() {
        test(
            &["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"],
            &["3S 4S 5D 6H JH"],
        )
    }

    #[test]
    fn test_a_tie_has_multiple_winners() {
        test(
            &[
                "4D 5S 6S 8D 3C",
                "2S 4C 7S 9H 10H",
                "3S 4S 5D 6H JH",
                "3H 4H 5C 6C JD",
            ],
            &["3S 4S 5D 6H JH", "3H 4H 5C 6C JD"],
        )
    }

    #[test]
    fn test_high_card_can_be_low_card_in_an_otherwise_tie() {
        // multiple hands with the same high cards, tie compares next highest ranked,
        // down to last card
        test(&["3S 5H 6S 8D 7H", "2S 5D 6D 8C 7S"], &["3S 5H 6S 8D 7H"])
    }

    #[test]
    fn test_one_pair_beats_high_card() {
        test(&["4S 5H 6C 8D KH", "2S 4H 6S 4D JH"], &["2S 4H 6S 4D JH"])
    }

    #[test]
    fn test_highest_pair_wins() {
        test(&["4S 2H 6S 2D JH", "2S 4H 6C 4D JD"], &["2S 4H 6C 4D JD"])
    }

    #[test]
    fn test_two_pairs_beats_one_pair() {
        test(&["2S 8H 6S 8D JH", "4S 5H 4C 8C 5C"], &["4S 5H 4C 8C 5C"])
    }

    #[test]
    fn test_two_pair_ranks() {
        // both hands have two pairs, highest ranked pair wins
        test(&["2S 8H 2D 8D 3H", "4S 5H 4C 8S 5D"], &["2S 8H 2D 8D 3H"])
    }

    #[test]
    fn test_two_pairs_second_pair_cascade() {
        // both hands have two pairs, with the same highest ranked pair,
        // tie goes to low pair
        test(&["2S QS 2C QD JH", "JD QH JS 8D QC"], &["JD QH JS 8D QC"])
    }

    #[test]
    fn test_two_pairs_last_card_cascade() {
        // both hands have two identically ranked pairs,
        // tie goes to remaining card (kicker)
        test(&["JD QH JS 8D QC", "JS QS JC 2D QD"], &["JD QH JS 8D QC"])
    }

    #[test]
    fn test_three_of_a_kind_beats_two_pair() {
        test(&["2S 8H 2H 8D JH", "4S 5H 4C 8S 4H"], &["4S 5H 4C 8S 4H"])
    }

    #[test]
    fn test_three_of_a_kind_ranks() {
        //both hands have three of a kind, tie goes to highest ranked triplet
        test(&["2S 2H 2C 8D JH", "4S AH AS 8C AD"], &["4S AH AS 8C AD"])
    }

    #[test]
    fn test_low_three_of_a_kind_beats_high_two_pair() {
        test(&["2H 2D 2C 8H 5H", "AS AC KS KC 6S"], &["2H 2D 2C 8H 5H"])
    }

    #[test]
    fn test_three_of_a_kind_cascade_ranks() {
        // with multiple decks, two players can have same three of a kind,
        // ties go to highest remaining cards
        test(&["4S AH AS 7C AD", "4S AH AS 8C AD"], &["4S AH AS 8C AD"])
    }

    #[test]
    fn test_straight_beats_three_of_a_kind() {
        test(&["4S 5H 4C 8D 4H", "3S 4D 2S 6D 5C"], &["3S 4D 2S 6D 5C"])
    }

    #[test]
    fn test_aces_can_end_a_straight_high() {
        // aces can end a straight (10 J Q K A)
        test(&["4S 5H 4C 8D 4H", "10D JH QS KD AC"], &["10D JH QS KD AC"])
    }

    #[test]
    fn test_aces_can_end_a_straight_low() {
        // aces can start a straight (A 2 3 4 5)
        test(&["4S 5H 4C 8D 4H", "4D AH 3S 2D 5C"], &["4D AH 3S 2D 5C"])
    }

    #[test]
    fn test_straight_cascade() {
        // both hands with a straight, tie goes to highest ranked card
        test(&["4S 6C 7S 8D 5H", "5S 7H 8S 9D 6H"], &["5S 7H 8S 9D 6H"])
    }

    #[test]
    fn test_straight_scoring() {
        // even though an ace is usually high, a 5-high straight is the lowest-scoring straight
        test(&["2H 3C 4D 5D 6H", "4S AH 3S 2D 5H"], &["2H 3C 4D 5D 6H"])
    }

    #[test]
    fn test_flush_beats_a_straight() {
        test(&["4C 6H 7D 8D 5H", "2S 4S 5S 6S 7S"], &["2S 4S 5S 6S 7S"])
    }

    #[test]
    fn test_flush_cascade() {
        // both hands have a flush, tie goes to high card, down to the last one if necessary
        test(&["4H 7H 8H 9H 6H", "2S 4S 5S 6S 7S"], &["4H 7H 8H 9H 6H"])
    }

    #[test]
    fn test_full_house_beats_a_flush() {
        test(&["3H 6H 7H 8H 5H", "4S 5C 4C 5D 4H"], &["4S 5C 4C 5D 4H"])
    }

    #[test]
    fn test_full_house_ranks() {
        // both hands have a full house, tie goes to highest-ranked triplet
        test(&["4H 4S 4D 9S 9D", "5H 5S 5D 8S 8D"], &["5H 5S 5D 8S 8D"])
    }

    #[test]
    fn test_full_house_cascade() {
        // with multiple decks, both hands have a full house with the same triplet, tie goes to the pair
        test(&["5H 5S 5D 9S 9D", "5H 5S 5D 8S 8D"], &["5H 5S 5D 9S 9D"])
    }

    #[test]
    fn test_four_of_a_kind_beats_full_house() {
        test(&["4S 5H 4D 5D 4H", "3S 3H 2S 3D 3C"], &["3S 3H 2S 3D 3C"])
    }

    #[test]
    fn test_four_of_a_kind_ranks() {
        // both hands have four of a kind, tie goes to high quad
        test(&["2S 2H 2C 8D 2D", "4S 5H 5S 5D 5C"], &["4S 5H 5S 5D 5C"])
    }

    #[test]
    fn test_four_of_a_kind_cascade() {
        // with multiple decks, both hands with identical four of a kind, tie determined by kicker
        test(&["3S 3H 2S 3D 3C", "3S 3H 4S 3D 3C"], &["3S 3H 4S 3D 3C"])
    }

    #[test]
    fn test_straight_flush_beats_four_of_a_kind() {
        test(&["4S 5H 5S 5D 5C", "7S 8S 9S 6S 10S"], &["7S 8S 9S 6S 10S"])
    }

    #[test]
    fn test_straight_flush_ranks() {
        // both hands have straight flush, tie goes to highest-ranked card
        test(&["4H 6H 7H 8H 5H", "5S 7S 8S 9S 6S"], &["5S 7S 8S 9S 6S"])
    }
}
