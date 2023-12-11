use std::fs;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum PointType {
    Galaxy,
    Empty,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn distance(&self, target: &Position) -> i64 {
        let (x1, y1) = (self.x, self.y);
        let (x2, y2) = (target.x, target.y);

        i64::abs(x2 - x1) + i64::abs(y2 - y1)
    }
}

// fn print(map: &Vec<Vec<PointType>>) {
//     map.iter().for_each(|row| {
//         row.iter().for_each(|char| {
//             let char = match char {
//                 PointType::Galaxy => "#",
//                 PointType::Empty => ".",
//             };
//
//             print!("{}", char);
//         });
//         println!();
//     })
// }

fn expand_space(map: &Vec<Vec<PointType>>, expand: &usize) -> Vec<Vec<PointType>> {
    let height = map.len();

    let vertical_expanded = map
        .iter()
        .map(|row| {
            let should_expand = row.iter().all(|point| point == &PointType::Empty);

            if should_expand {
                vec![row.clone(); *expand + 1]
            } else {
                vec![row.clone()]
            }
        })
        .flatten()
        .collect::<Vec<Vec<PointType>>>();

    let mut cloned_map = vertical_expanded.clone();

    vertical_expanded
        .iter()
        .enumerate()
        .for_each(|(row_index, row)| {
            let mut expanded = 0;

            row.iter().enumerate().for_each(|(col_index, item)| {
                let should_expand =
                    (0..height).all(|row_index| map[row_index][col_index] == PointType::Empty);

                if should_expand {
                    (0..*expand).into_iter().for_each(|_| {
                        cloned_map[row_index].insert(col_index + expanded, item.clone());
                        expanded += 1;
                    })
                }
            })
        });

    cloned_map
}

fn append_positions(map: Vec<Vec<PointType>>) -> Vec<Vec<(Position, PointType)>> {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, point)| {
                    (
                        Position {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        },
                        point.clone(),
                    )
                })
                .collect::<Vec<(Position, PointType)>>()
        })
        .collect()
}

fn distance(positions: &Vec<&Position>) -> Vec<i64> {
    positions
        .iter()
        .enumerate()
        .flat_map(|(index, a)| positions[(index + 1)..].iter().map(|b| a.distance(b)))
        .collect()
}

fn get_answer_with_expands(expand: usize, map: &Vec<Vec<PointType>>) -> i64 {
    let expanded = expand_space(&map, &expand);
    let appended = append_positions(expanded);

    let galaxies: Vec<&Position> = appended
        .iter()
        .flatten()
        .filter(|(_, point_type)| point_type == &PointType::Galaxy)
        .map(|(position, _)| position)
        .collect();

    distance(&galaxies).iter().sum()
}

pub fn main() {
    let input = fs::read_to_string("src/11/input.txt").expect("File not found");

    let map: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '#' => PointType::Galaxy,
                    '.' => PointType::Empty,
                    _ => panic!("Invalid char"),
                })
                .collect::<Vec<PointType>>()
        })
        .collect();

    let a = get_answer_with_expands(1, &map);

    let base = get_answer_with_expands(0, &map);
    let increase = get_answer_with_expands(1, &map) - base;

    let b = (1000000 - 1) * increase + base;

    println!("Answer a: {}", a);
    println!("Answer b: {}", b);
}
