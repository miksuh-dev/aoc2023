use std::fs;

fn get_hash(sequence: &str) -> usize {
    let mut current_value: usize = 0;

    sequence
        .chars()
        .filter(|&char| !char.is_whitespace())
        .for_each(|char| {
            current_value += char as usize;
            current_value *= 17;
            current_value %= 256;
        });

    current_value
}

pub fn main() {
    let input = fs::read_to_string("src/15/input.txt").expect("File not found");
    let sequences: Vec<&str> = input.lines().next().unwrap().split(",").collect();

    let a: usize = sequences.iter().map(|sequence| get_hash(sequence)).sum();

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    sequences.iter().for_each(|sequence| {
        if sequence.ends_with("-") {
            let (label, _) = sequence.split_once("-").unwrap();
            let hash = get_hash(label);

            boxes[hash].retain(|(label_2, _)| label != *label_2)
        } else {
            let (label, focal_str) = sequence.split_once("=").unwrap();
            let new_lens = (label, focal_str.parse::<usize>().unwrap());
            let hash = get_hash(label);

            if let Some(existing) = boxes[hash]
                .iter()
                .position(|(label_2, _)| &label == label_2)
            {
                boxes[hash][existing] = new_lens;
            } else {
                boxes[hash].push(new_lens);
            }
        }
    });

    let b: usize = boxes
        .iter()
        .enumerate()
        .filter(|(_, content)| content.len() > 0)
        .map(|(box_index, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(index, (_, focal))| (box_index + 1) * (index + 1) * focal)
                .sum::<usize>()
        })
        .sum();

    println!("Answer a: {}", a);
    println!("Answer b: {}", b);
}
