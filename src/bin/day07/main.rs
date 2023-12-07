use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;

lazy_static! {
    static ref CARD_VALUES: HashMap<u8, u8> = HashMap::from([
        (b'2', 1),
        (b'3', 2),
        (b'4', 3),
        (b'5', 4),
        (b'6', 5),
        (b'7', 6),
        (b'8', 7),
        (b'9', 8),
        (b'T', 9),
        (b'J', 10),
        (b'Q', 11),
        (b'K', 12),
        (b'A', 13),
        (b'*', 0), //joker
    ]);
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [u8; 5],
    bid: u64,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: &[u8], bid: u64, joker_rule: bool) -> Self {
        let mut cards: [u8; 5] = cards.try_into().unwrap();
        if joker_rule {
            for c in &mut cards {
                if *c == b'J' {
                    *c = b'*';
                }
            }
        }

        Self {
            cards,
            bid,
            hand_type: Self::hand_type(&cards),
        }
    }

    fn hand_type(cards: &[u8; 5]) -> HandType {
        use HandType::*;
        if cards.iter().all(|&c| c == cards[0]) {
            return FiveOfAKind;
        }

        let mut card_map = HashMap::new();
        for c in cards {
            *card_map.entry(c).or_default() += 1;
        }
        let jokers = card_map.remove(&b'*').unwrap_or_default();
        let card_counts: Vec<u8> = card_map.into_values().collect();

        if card_counts.iter().any(|&c| c == 4) {
            if jokers == 1 {
                return FiveOfAKind;
            }
            return FourOfAKind;
        }
        if card_counts.iter().any(|&c| c == 3) {
            if jokers == 2 {
                return FiveOfAKind;
            } else if jokers == 1 {
                return FourOfAKind;
            }
            if card_counts.iter().any(|&c| c == 2) {
                return FullHouse;
            }
            return ThreeOfAKind;
        }
        let pairs = card_counts.iter().filter(|&&c| c == 2).count();
        if pairs == 2 {
            if jokers == 1 {
                return FullHouse;
            }
            return TwoPair;
        } else if pairs == 1 {
            if jokers == 3 {
                return FiveOfAKind;
            } else if jokers == 2 {
                return FourOfAKind;
            } else if jokers == 1 {
                return ThreeOfAKind;
            }
            return Pair;
        }

        match jokers {
            4 => FiveOfAKind,
            3 => FourOfAKind,
            2 => ThreeOfAKind,
            1 => Pair,
            _ => HighCard,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_ordering = self.hand_type.cmp(&other.hand_type);
        if type_ordering != Ordering::Equal {
            return type_ordering;
        }
        for (card, other_card) in zip(self.cards.iter(), other.cards.iter()) {
            let card_ordering = CARD_VALUES[card].cmp(&CARD_VALUES[other_card]);
            if card_ordering != Ordering::Equal {
                return card_ordering;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            Hand::new(
                split.next().unwrap().as_bytes(),
                split.next().unwrap().parse::<u64>().unwrap(),
                false,
            )
        })
        .collect();
    hands.sort();
    println!(
        "part 1: {}",
        hands
            .iter()
            .enumerate()
            .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1) as u64)
    );

    let mut hands: Vec<Hand> = hands
        .into_iter()
        .map(|h| Hand::new(&h.cards, h.bid, true))
        .collect();
    hands.sort();
    println!(
        "part 2: {}",
        hands
            .iter()
            .enumerate()
            .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1) as u64)
    );
}
