use itertools::Itertools;
use std::{cmp::Ordering, fs};

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    hand_type: usize,
    bet: usize,
}

#[derive(Debug, Clone)]
struct Card {
    value: usize,
}

impl Card {
    pub fn new(char: char) -> Self {
        let value = match char {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            value => value.to_string().parse::<usize>().unwrap(),
        };

        Self { value }
    }
}

impl Hand {
    pub fn new(cards_str: &str, bet_str: &str) -> Self {
        let cards = cards_str
            .chars()
            .map(|char| Card::new(char))
            .collect::<Vec<Card>>();

        let bet = bet_str.parse::<usize>().unwrap();

        Self {
            hand_type: Hand::get_hand_type(&cards),
            cards,
            bet,
        }
    }

    pub fn get_hand_type(cards: &Vec<Card>) -> usize {
        let mut cards_copy = cards.clone();

        cards_copy.sort_by(|a, b| b.value.cmp(&a.value));

        let mut dups: Vec<_> = cards_copy
            .iter()
            .map(|c| (c, 1))
            .coalesce(|(card_1, n), (card_2, m)| {
                if card_1.value == card_2.value {
                    Ok((card_1, n + m))
                } else {
                    Err(((card_1, n), (card_2, m)))
                }
            })
            .collect();

        dups.sort_by(|(a_card, count_1), (b_card, count_2)| {
            if count_1 != count_2 {
                count_2.cmp(count_1)
            } else {
                b_card.value.cmp(&a_card.value)
            }
        });

        match dups[..] {
            [(_, 5), ..] => 6,         // Five
            [(_, 4), ..] => 5,         // Four
            [(_, 3), (_, 2), ..] => 4, // Fullhouse
            [(_, 3), ..] => 3,         // Three pair
            [(_, 2), (_, 2), ..] => 2, // Two pair
            [(_, 2), ..] => 1,         // One pair
            _ => 0,                    // Highcard
        }
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            other.hand_type.cmp(&self.hand_type)
        } else {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .map(|(a, b)| b.value.cmp(&a.value))
                .find(|&ordering| ordering != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        }
    }
}

pub fn main() {
    let input = fs::read_to_string("src/07/input.txt").expect("File not found");

    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(cards, bet)| Hand::new(cards, bet))
        .collect();

    hands.sort_by(|a, b| b.compare(a));

    let a: usize = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| -> usize { (index + 1) * hand.bet })
        .sum();

    println!("Answer a: {}", a);
}
