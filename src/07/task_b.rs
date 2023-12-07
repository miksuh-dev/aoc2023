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
            'T' => 10,
            'J' => 1,
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
            .map(|c| (c, 1, if 1 == c.value { 1 } else { 0 }))
            .coalesce(|(card_1, n, joker_count_1), (card_2, m, joker_count_2)| {
                if card_1.value == card_2.value {
                    Ok((card_1, n + m, joker_count_1 + joker_count_2))
                } else {
                    Err(((card_1, n, joker_count_1), (card_2, m, joker_count_2)))
                }
            })
            .collect();

        dups.sort_by(|(a_card, count_1, _), (b_card, count_2, _)| {
            if count_1 == count_2 || a_card.value == 1 || b_card.value == 1 {
                b_card.value.cmp(&a_card.value)
            } else {
                count_2.cmp(count_1)
            }
        });

        match dups[..] {
            // Fives
            [(_, 5, _)] => 7,
            [(_, 4, _), .., (_, _, 1)] => 6,
            [(_, 3, _), .., (_, _, 2)] => 6,
            [(_, 2, _), .., (_, _, 3)] => 6,
            [(_, 1, _), .., (_, _, 4)] => 6,

            // Fours
            [(_, 4, _), .., (_, _, 0)] => 5,
            [(_, 3, _), .., (_, _, 1)] => 5,
            [(_, 2, _), .., (_, _, 2)] => 5,
            [(_, 1, _), .., (_, _, 3)] => 5,

            // Fullhouse
            [(_, 3, _), (_, 2, _)] => 4,
            [(_, 2, _), (_, 2, _), .., (_, _, 1)] => 4,

            // Three pairs
            [(_, 3, _), .., (_, _, 0)] => 3,
            [(_, 2, _), .., (_, _, 1)] => 3,
            [(_, 1, _), .., (_, _, 2)] => 3,

            // Two pair
            [(_, 2, _), (_, 2, _), .., (_, _, 0)] => 2,

            // One pair
            [(_, 2, _), .., (_, _, 0)] => 1,
            [(_, 1, _), .., (_, _, 1)] => 1,

            // Highcard
            _ => 0,
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

    let b: usize = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| -> usize { (index + 1) * hand.bet })
        .sum();

    println!("Answer b: {}", b);
}
