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
        (b'A', 13)
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
    fn new(cards: &[u8], bid: u64) -> Self {
        let cards: [u8; 5] = cards.try_into().unwrap();

        Self {
            cards,
            bid,
            hand_type: Self::hand_type(&cards),
        }
    }

    fn hand_type(cards: &[u8; 5]) -> HandType {
        if cards.iter().all(|&c| c == cards[0]) {
            return HandType::FiveOfAKind;
        }

        let mut card_map = HashMap::new();
        for c in cards {
            *card_map.entry(c).or_default() += 1;
        }
        let card_counts: Vec<u8> = card_map.into_values().collect();

        if card_counts.iter().any(|&c| c == 4) {
            return HandType::FourOfAKind;
        }
        if card_counts.iter().any(|&c| c == 3) {
            if card_counts.iter().any(|&c| c == 2) {
                return HandType::FullHouse;
            }
            return HandType::ThreeOfAKind;
        }
        let pairs = card_counts.iter().filter(|&&c| c == 2).count();
        if pairs == 2 {
            return HandType::TwoPair;
        } else if pairs == 1 {
            return HandType::Pair;
        }

        HandType::HighCard
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
            )
        })
        .collect();
    hands.sort();
    println!(
        "part 1: {}",
        hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1) as u64)
    );
}
