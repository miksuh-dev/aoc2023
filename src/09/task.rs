use std::fs;

fn create_diff_lines(line: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![line];

    loop {
        let last = &result.last().unwrap();

        if last.iter().all(|&value| value == 0) {
            return result;
        }

        let diff_line = last
            .windows(2)
            .flat_map(<&[i32; 2]>::try_from)
            .map(|[a, b]| b - a)
            .collect::<Vec<i32>>();

        result.push(diff_line)
    }
}

fn extrapolate_forward(data_with_diffs: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = data_with_diffs.clone();

    for index in (1..result.len()).rev() {
        let current_line_end = result[index].last().unwrap().clone();
        let upper_line_end = result[index - 1].last().unwrap().clone();

        result[index - 1].push(current_line_end + upper_line_end);
    }

    result
}

fn extrapolate_backwards(data_with_diffs: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = data_with_diffs.clone();

    for index in (1..result.len()).rev() {
        let current_line_start = result[index].first().unwrap().clone();
        let upper_line_start = result[index - 1].first().unwrap().clone();

        result[index - 1].insert(0, upper_line_start - current_line_start);
    }

    result
}

pub fn main() {
    let input = fs::read_to_string("src/09/input.txt").expect("File not found");

    let parsed_data: Vec<_> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|value| value.parse::<i32>().ok())
                .collect()
        })
        .map(create_diff_lines)
        .collect();

    let a: i32 = parsed_data
        .iter()
        .map(extrapolate_forward)
        .map(|line| line[0].last().unwrap().clone())
        .sum();

    let b: i32 = parsed_data
        .iter()
        .map(extrapolate_backwards)
        .map(|line| line[0].first().unwrap().clone())
        .sum();

    println!("Answer a: {}", a);
    println!("Answer b: {}", b);
}
