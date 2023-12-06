use itertools::Itertools;
use std::fs;
use std::iter::zip;

struct Game {
    distance: i64,
    time: i64,
}

impl Game {
    fn get_possible_wins_count(&self) -> i64 {
        (0..self.time + 1)
            .into_iter()
            .map(|press_time| {
                let speed = press_time;
                let distance = self.time - press_time;

                speed * distance
            })
            .filter(|travel_distance| travel_distance > &self.distance)
            .count()
            .try_into()
            .unwrap()
    }
}

pub fn main() {
    let input = fs::read_to_string("src/06/input.txt").expect("File not found");

    let a: i64 = input
        .lines()
        .map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .split(" ")
                .filter(|char| !char.is_empty())
                .filter_map(|number| number.parse::<i64>().ok())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
        .windows(2)
        .flat_map(|window| {
            zip(window[0].clone(), window[1].clone())
                .map(|(time, distance)| Game { time, distance })
        })
        .map(|game| game.get_possible_wins_count())
        .product();

    let (time, distance) = input
        .lines()
        .flat_map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .replace(" ", "")
                .parse::<i64>()
                .ok()
        })
        .collect_tuple()
        .unwrap();

    let b = Game { distance, time }.get_possible_wins_count();

    println!("Answer a: {}", a);
    println!("Answer b: {}", b);
}
