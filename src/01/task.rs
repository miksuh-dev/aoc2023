use std::fs;

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_number(own: &str) -> Option<i32> {
    match own {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

pub fn main() {
    let input = fs::read_to_string("src/01/input.txt").expect("File not found");

    let a: i32 = input
        .lines()
        .map(|row| {
            let first = row
                .chars()
                .into_iter()
                .find(|c| c.is_digit(10))
                .unwrap()
                .to_string();

            let last = row
                .chars()
                .into_iter()
                .rev()
                .find(|c| c.is_digit(10))
                .unwrap()
                .to_string();

            format!("{}{}", first, last).parse::<i32>().unwrap()
        })
        .sum();

    let b: i32 = input
        .lines()
        .map(|row| {
            let word_matches = WORDS
                .iter()
                .map(|&word| {
                    row.match_indices(word)
                        .map(|(index, value)| (get_number(value).unwrap(), index))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<(i32, usize)>>();

            let num_matches = row
                .chars()
                .into_iter()
                .enumerate()
                .filter_map(|(index, c)| {
                    let result = c.is_digit(10);

                    match result {
                        true => Some((c.to_string().parse::<i32>().unwrap(), index)),
                        false => None,
                    }
                })
                .collect::<Vec<(i32, usize)>>();

            let (first, _) = word_matches
                .iter()
                .chain(&num_matches)
                .min_by(|(_, index1), (_, index2)| index1.cmp(&index2))
                .unwrap();

            let (last, _) = word_matches
                .iter()
                .chain(&num_matches)
                .max_by(|(_, index1), (_, index2)| index1.cmp(&index2))
                .unwrap();

            format!("{}{}", first, last).parse::<i32>().unwrap()
        })
        .sum();

    println!("Answer a: {}", a);
    println!("Answer b: {}", b);
}
