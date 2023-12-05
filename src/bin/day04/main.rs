struct Scratchcard {
    pub winning_numbers: Vec<u32>,
    pub your_numbers: Vec<u32>,
}

impl Scratchcard {
    fn matches(&self) -> u32 {
        let mut matches = 0;
        for n in &self.your_numbers {
            if self.winning_numbers.contains(n) {
                matches += 1;
            }
        }
        matches
    }

    fn points(&self) -> u64 {
        let matches = self.matches();
        if matches > 0 {
            2_u64.pow(matches - 1)
        } else {
            0
        }
    }
}

struct Book {
    // tuple of card and number of copies of card
    pub cards: Vec<(u32, Scratchcard)>,
}

impl Book {
    fn new(cards: Vec<Scratchcard>) -> Self {
        Self {
            cards: cards.into_iter().map(|s| (1, s)).collect(),
        }
    }

    fn total_cards(&mut self) -> u64 {
        let len = self.cards.len();
        for i in 0..len {
            let copies = self.cards[i].0;
            let matches = self.cards[i].1.matches() as usize;
            for j in 1..=matches {
                self.cards[i + j].0 += copies;
            }
        }
        self.cards.iter().fold(0, |acc, c| acc + c.0 as u64)
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let scratchcards: Vec<Scratchcard> = input
        .lines()
        .map(|line| {
            let start = line.find(':').unwrap();
            let mid = line.find('|').unwrap();
            let winning_numbers: Vec<u32> = line[start + 1..mid]
                .split(' ')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let your_numbers: Vec<u32> = line[mid + 1..]
                .split(' ')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            Scratchcard {
                winning_numbers,
                your_numbers,
            }
        })
        .collect();

    let part_1_total = scratchcards.iter().fold(0, |acc, s| acc + s.points());
    println!("Part 1: {part_1_total}");

    let mut book = Book::new(scratchcards);
    println!("Part 2: {}", book.total_cards());
}
