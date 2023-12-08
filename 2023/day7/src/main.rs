use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483\
";

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => unreachable!(), // only these cards are present
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    fn from(cards: [Card; 5]) -> Self {
        let mut map: HashMap<Card, u8> = HashMap::new();
        for card in cards {
            let count = map.entry(card).or_insert(0);
            *count += 1;
        }
        let mut counts: Vec<u8> = map.into_values().collect();
        counts.sort();
        match counts.as_slice() {
            [1, 1, 1, 1, 1] => HandKind::HighCard,
            [1, 1, 1, 2] => HandKind::OnePair,
            [1, 2, 2] => HandKind::TwoPair,
            [1, 1, 3] => HandKind::ThreeOfAKind,
            [2, 3] => HandKind::FullHouse,
            [1, 4] => HandKind::FourOfAKind,
            [5] => HandKind::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Hand {
    bid: u64,
    cards: [Card; 5],
    kind: HandKind,
}

impl Hand {
    fn new(cards: [Card; 5], bid: u64) -> Self {
        Self {
            cards,
            bid,
            kind: HandKind::from(cards),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.cards.eq(&other.cards)
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            std::cmp::Ordering::Equal => match self.cards.cmp(&other.cards) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
                ord => ord,
            },
            ord => ord,
        }
    }
}

fn main() {
    let solve = part_1(INPUT);
    println!("{solve}");
}

fn part_1(input: &str) -> u64 {
    let start = Instant::now();
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        let all_cards = split.next().unwrap();
        let cards: [Card; 5] = all_cards
            .chars()
            .map(Card::from)
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();

        let bid = split.next().unwrap().parse().unwrap();
        let hand = Hand::new(cards, bid);
        hands.push(hand);
    }

    let end = Instant::now();
    println!("Parsing time: {:?}", end.duration_since(start));

    // println!("{:?}", hands);

    let start = Instant::now();

    hands.sort();
    let solve = hands
        .iter()
        .enumerate()
        .map(|(idx, &hand)| hand.bid * (idx as u64 + 1))
        .fold(0, |a, c| a + c);

    let end = Instant::now();

    println!("Processing time: {:?}", end.duration_since(start));

    solve
}
