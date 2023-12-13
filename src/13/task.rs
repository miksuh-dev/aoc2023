use std::fs;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum PointType {
    Ash,
    Rock,
}
#[derive(Debug, Hash, PartialEq, Clone, Eq)]
struct Point {
    position: (i32, i32),
    point_type: PointType,
}

#[derive(Debug)]
struct Board {
    pub vec: Vec<Point>,
    width: usize,
    height: usize,
}

impl Point {
    pub fn new(x: i32, y: i32, point_type: PointType) -> Point {
        Self {
            position: (x, y),
            point_type,
        }
    }
}

impl Board {
    pub fn new(initial_vec: Vec<char>, width: usize, height: usize) -> Self {
        let mut vec = Vec::new();

        for (index, value) in initial_vec.iter().enumerate() {
            let x = index % width;
            let y = index / width;

            let t = match value {
                '#' => PointType::Rock,
                '.' => PointType::Ash,
                value => panic!("Unknown type {}", value),
            };

            let point = Point::new(x as i32, y as i32, t);
            vec.push(point);
        }

        Self { vec, width, height }
    }

    fn index(&self, x: usize, y: usize) -> Option<&Point> {
        if (x >= self.width) || (y >= self.height) {
            return None;
        }

        Some(&self.vec[self.width * y + x])
    }

    fn _print(&self) {
        self.vec.iter().enumerate().for_each(|(index, point)| {
            let char = match point.point_type {
                PointType::Ash => ".",
                PointType::Rock => "#",
            };

            print!("{}", char);

            if index % self.width == 1 {
                println!("")
            }
        })
    }

    fn get_row(&self, y: usize) -> Vec<&Point> {
        (0..self.width)
            .into_iter()
            .filter_map(|x| self.index(x, y))
            .collect()
    }

    fn get_col(&self, x: usize) -> Vec<&Point> {
        (0..self.height)
            .into_iter()
            .filter_map(|y| self.index(x, y))
            .collect()
    }

    fn get_first_vertical_reflection_a(&self) -> usize {
        (1..self.width)
            .into_iter()
            .find(|index| {
                (0..*index)
                    .rev()
                    .zip(*index..self.width)
                    .all(|(left_index, right_index)| {
                        self.get_col(left_index)
                            .iter()
                            .zip(self.get_col(right_index))
                            .all(|(a, b)| a.point_type == b.point_type)
                    })
            })
            .unwrap_or(0)
    }

    fn get_first_horizontal_reflection_a(&self) -> usize {
        (1..self.height)
            .into_iter()
            .find(|index| {
                (0..*index)
                    .rev()
                    .zip(*index..self.height)
                    .all(|(left_index, right_index)| {
                        self.get_row(left_index)
                            .iter()
                            .zip(self.get_row(right_index))
                            .all(|(a, b)| a.point_type == b.point_type)
                    })
            })
            .unwrap_or(0)
    }

    fn get_first_vertical_reflection_b(&self) -> usize {
        (1..self.width)
            .into_iter()
            .find(|index| {
                let smudge_count = (0..*index)
                    .rev()
                    .zip(*index..self.width)
                    .map(|(left_index, right_index)| {
                        self.get_col(left_index)
                            .iter()
                            .zip(self.get_col(right_index))
                            .filter(|(a, b)| a.point_type != b.point_type)
                            .count()
                    })
                    .sum::<usize>();

                smudge_count == 1
            })
            .unwrap_or(0)
    }

    fn get_first_horizontal_reflection_b(&self) -> usize {
        (1..self.height)
            .into_iter()
            .find(|index| {
                let smudge_count = (0..*index)
                    .rev()
                    .zip(*index..self.height)
                    .map(|(left_index, right_index)| {
                        self.get_row(left_index)
                            .iter()
                            .zip(self.get_row(right_index))
                            .filter(|(a, b)| a.point_type != b.point_type)
                            .count()
                    })
                    .sum::<usize>();

                smudge_count == 1
            })
            .unwrap_or(0)
    }

    fn get_reflections_a(&self) -> usize {
        let vertical = self.get_first_vertical_reflection_a();
        let horizontal = self.get_first_horizontal_reflection_a();

        vertical + horizontal * 100
    }

    fn get_reflections_b(&self) -> usize {
        let vertical = self.get_first_vertical_reflection_b();
        let horizontal = self.get_first_horizontal_reflection_b();

        vertical + horizontal * 100
    }
}

pub fn main() {
    let input = fs::read_to_string("src/13/input.txt").expect("File not found");

    let boards: Vec<_> = input
        .split("\n\n")
        .map(|pattern| {
            let all_rows: Vec<&str> = pattern.split("\n").filter(|str| !str.is_empty()).collect();

            let width = all_rows.iter().nth(0).unwrap().len();
            let height = all_rows.len();

            let cells = all_rows
                .iter()
                .flat_map(|str| str.chars().collect::<Vec<char>>())
                .collect();

            Board::new(cells, width, height)
        })
        .collect();

    let a: usize = boards.iter().map(|board| board.get_reflections_a()).sum();
    let b: usize = boards.iter().map(|board| board.get_reflections_b()).sum();

    println!("Answer a: {}", a);
    println!("Answer b: {}", b);
}
