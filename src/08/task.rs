use itertools::Itertools;
use num_integer::Integer;
use std::{collections::HashMap, fs};

enum Move {
    Left,
    Right,
}

fn get_path_length(
    start: String,
    is_end: fn(&String) -> bool,
    moves: &Vec<Move>,
    map: &HashMap<String, (String, String)>,
) -> i128 {
    let mut current_position = &start;

    let mut index = 0;
    loop {
        for m in moves.iter() {
            let (left, right) = map.get(current_position).unwrap();

            current_position = match m {
                Move::Left => left,
                Move::Right => right,
            };

            index += 1;

            if is_end(current_position) {
                return index;
            }
        }
    }
}

fn is_end_a(position: &String) -> bool {
    position == "ZZZ"
}

fn is_end_b(position: &String) -> bool {
    position.ends_with("Z")
}

pub fn main() {
    let input = fs::read_to_string("src/08/input.txt").expect("File not found");

    let moves: Vec<Move> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|char| match char {
            'L' => Move::Left,
            'R' => Move::Right,
            _ => panic!("Invalid move"),
        })
        .collect();

    let map: HashMap<String, (String, String)> =
        HashMap::from_iter(input.lines().skip(2).map(|line| {
            let (start, end_str) = line.split_once(" = ").unwrap();

            let (left, right) = end_str
                .split(", ")
                .map(|value| value.replace("(", "").replace(")", ""))
                .next_tuple()
                .unwrap();

            (start.to_string(), (left, right))
        }));

    let a: i128 = get_path_length("AAA".to_string(), is_end_a, &moves, &map);

    let b: i128 = input
        .lines()
        .skip(2)
        .map(|line| line.split(" = ").next().unwrap().to_string())
        .filter(|line| line.ends_with("A"))
        .map(|position| get_path_length(position, is_end_b, &moves, &map))
        .reduce(|acc, value| Integer::lcm(&acc, &value))
        .unwrap();

    println!("Answer a: {:?}", a);
    println!("Answer b: {:?}", b);
}
