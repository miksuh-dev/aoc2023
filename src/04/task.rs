use itertools::Itertools;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("src/04/input.txt").expect("File not found");

    let match_count = input
        .lines()
        .map(|line| {
            let (winning_hand, own_hand) = line
                .split_once(":")
                .unwrap()
                .1
                .split("|")
                .map(|hand| {
                    hand.split(' ')
                        .filter_map(|number| number.parse::<i32>().ok())
                        .collect::<Vec<i32>>()
                })
                .next_tuple()
                .expect("Could not parse input");

            own_hand
                .iter()
                .filter(|item| winning_hand.contains(item))
                .count()
        })
        .collect::<Vec<_>>();

    let a: usize = match_count
        .iter()
        .map(|&count| if count > 0 { 1 << (count - 1) } else { 0 })
        .sum();

    let mut card_counts = vec![0; match_count.len()];
    for (card_index, cards) in match_count.iter().enumerate() {
        for target_index in (card_index + 1)..(card_index + cards + 1) {
            card_counts[target_index] += card_counts[card_index] + 1;
        }

        card_counts[card_index] += 1;
    }

    let b: usize = card_counts.iter().sum();

    println!("Answer a: {}", a);
    println!("Answer b: {}", b);
}
