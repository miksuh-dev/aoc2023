use rayon::prelude::*;
use std::fs;

fn get_answer(seeds: Vec<i64>, groups: &Vec<Vec<(i64, i64, i64)>>) -> i64 {
    seeds
        .par_iter()
        .map(|&value| {
            let mut current_destination = value;
            for target_table in 1..groups.len() {
                current_destination =
                    match groups[target_table]
                        .iter()
                        .find(|&(_, source, range_length)| {
                            source + range_length > current_destination
                                && current_destination + 1 > *source
                        }) {
                        Some((destination, source, _)) => {
                            destination + current_destination - source
                        }
                        None => current_destination,
                    };
            }

            current_destination
        })
        .min()
        .unwrap()
}

pub fn main() {
    let input = fs::read_to_string("src/05/input.txt").expect("File not found");

    let mut groups: Vec<Vec<(i64, i64, i64)>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    let mut index = 0;
    input.lines().skip(2).for_each(|item| {
        if item == "" {
            index += 1;
        } else if char::is_numeric(item.chars().next().unwrap()) {
            let result: Vec<i64> = item
                .split(" ")
                .map(|number| number.parse::<i64>().unwrap())
                .take(3)
                .collect();

            let [destination, source, range_length] = [result[0], result[1], result[2]];

            groups[index + 1].push((destination, source, range_length))
        }
    });

    let a_seeds: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter_map(|number| number.parse::<i64>().ok())
        .collect();

    let b_seeds = a_seeds
        .chunks(2)
        .flat_map(|item| {
            let [seed, count] = [item[0], item[1]];

            seed..(seed + count)
        })
        .collect();

    let a = get_answer(a_seeds, &groups);
    let b = get_answer(b_seeds, &groups);

    println!("Answer a: {}", a);
    println!("Answer b: {}", b);
}
