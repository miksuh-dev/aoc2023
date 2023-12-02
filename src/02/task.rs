use std::fs;

pub struct Cube {
    color: String,
    count: i32,
}

impl Cube {
    pub fn new(color: String, count: i32) -> Self {
        Self { color, count }
    }
}

pub fn main() {
    let input = fs::read_to_string("src/02/input.txt").expect("File not found");

    let games: Vec<(i32, Vec<Vec<Cube>>)> = input
        .lines()
        .map(|row| {
            let (game_str, sets_str) = row.split_once(":").unwrap();
            let game_num = game_str.split(" ").nth(1).unwrap().parse::<i32>().unwrap();

            let sets = sets_str
                .split(";")
                .map(|set| {
                    set.split(',')
                        .map(|cube| {
                            let (count_str, color_str) = cube.trim().split_once(' ').unwrap();

                            let color = color_str.to_string();
                            let count = count_str.trim().parse::<i32>().unwrap();

                            Cube::new(color, count)
                        })
                        .collect()
                })
                .collect();

            (game_num, sets)
        })
        .collect();

    let a = games
        .iter()
        .filter(|(_, sets)| {
            !sets.iter().flatten().any(|cube| match cube.color.as_str() {
                "red" => cube.count > 12,
                "green" => cube.count > 13,
                "blue" => cube.count > 14,
                _ => panic!("Unknown color"),
            })
        })
        .map(|(game, _)| game)
        .sum::<i32>();

    let b = games
        .iter()
        .map(|(_, sets)| {
            let max_red = sets
                .iter()
                .flatten()
                .filter(|cube| cube.color == "red")
                .max_by_key(|cube| cube.count)
                .unwrap();

            let max_green = sets
                .iter()
                .flatten()
                .filter(|cube| cube.color == "green")
                .max_by_key(|cube| cube.count)
                .unwrap();

            let max_blue = sets
                .iter()
                .flatten()
                .filter(|cube| cube.color == "blue")
                .max_by_key(|cube| cube.count)
                .unwrap();

            max_red.count * max_green.count * max_blue.count
        })
        .sum::<i32>();

    println!("Answer a: {}", a);
    println!("Answer b: {}", b);
}
