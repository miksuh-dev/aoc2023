use rayon::prelude::*;
use std::fs;

fn get_count(spring: String, records: &Vec<usize>) -> usize {
    let mut result = vec![spring.to_string()];

    let unknown_count = spring.chars().filter(|char| char == &'?').count();

    (0..unknown_count).into_iter().for_each(|_| {
        let new: Vec<String> = result
            .iter()
            .flat_map(|variation| {
                let a = variation.replacen("?", "#", 1);
                let b = variation.replacen("?", ".", 1);

                vec![a, b]
            })
            .filter(|variation| !result.contains(variation))
            .collect();

        result.extend(new);
    });

    result
        .into_par_iter()
        .filter(|spring| !spring.contains("?"))
        .map(|spring| group(spring))
        .filter(|variation| is_valid(variation, records))
        .count()
}

fn group(spring: String) -> Vec<Vec<char>> {
    let mut contiguous = true;

    let mut groups = vec![];
    let mut group = vec![];
    let mut first_damaged_found = false;

    spring.chars().for_each(|char| {
        if first_damaged_found == true {
            if char == '#' && contiguous == false {
                groups.push(group.clone());
                group.clear();
                contiguous = true
            } else if char == '.' && contiguous == true {
                contiguous = false
            }
        }

        if first_damaged_found == false && char == '#' {
            first_damaged_found = true
        }

        group.push(char);
    });

    if group.len() != 0 {
        groups.push(group.clone());
    }

    groups
}

fn is_valid(groups: &Vec<Vec<char>>, records: &Vec<usize>) -> bool {
    if groups.len() != records.len() {
        return false;
    }

    groups
        .iter()
        .zip(records.iter())
        .all(|(spring_group, expected_broken)| {
            spring_group.iter().filter(|&&spring| spring == '#').count() == *expected_broken
        })
}

pub fn main() {
    let input = fs::read_to_string("src/12/input.txt").expect("File not found");

    let a: usize = input
        .lines()
        .map(|line| {
            let (spring, record_str) = line.split_once(" ").unwrap();

            let record = record_str
                .split(",")
                .filter_map(|number| number.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            (spring.to_string(), record)
        })
        .par_bridge()
        .map(|(spring, records)| get_count(spring, &records))
        .sum();

    println!("Answer a: {}", a);
}
