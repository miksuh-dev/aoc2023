use std::fs;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum PointType {
    Symbol,
    Number { value: i32, points: Vec<(i32, i32)> },
    Empty,
}

#[derive(Debug, Hash, PartialEq, Clone, Eq)]
struct Point {
    position: (i32, i32),
    t: PointType,
}

struct Board {
    pub vec: Vec<Point>,
    row: usize,
    col: usize,
}

impl PointType {
    fn get_value(&self) -> i32 {
        match self {
            PointType::Number { value, .. } => *value,
            _ => panic!("None"),
        }
    }

    fn get_points(&self) -> &Vec<(i32, i32)> {
        match self {
            PointType::Number { points, .. } => points,
            _ => panic!("None"),
        }
    }
}

impl Point {
    pub fn new(x: i32, y: i32, t: PointType) -> Point {
        Self {
            position: (x, y),
            t,
        }
    }

    fn get_value(&self) -> i32 {
        self.t.get_value()
    }
}

impl Board {
    pub fn new(initial_vec: Vec<char>, row: usize, col: usize) -> Self {
        let mut vec = Vec::new();

        for (index, value) in initial_vec.iter().enumerate() {
            let x = index % col;
            let y = index / col;

            let t = match value {
                '.' => PointType::Empty,
                value => {
                    if value.is_digit(10) {
                        PointType::Number {
                            value: value.to_string().parse::<i32>().unwrap(),
                            points: vec![((x as i32, y as i32))],
                        }
                    } else {
                        PointType::Symbol
                    }
                }
            };

            let point = Point::new(x as i32, y as i32, t);
            vec.push(point);
        }

        Self { vec, row, col }
    }

    fn index(&self, row: usize, col: usize) -> Option<&Point> {
        if (row >= self.row) || (col >= self.col) {
            return None;
        }

        let i = self.col * row;

        Some(&self.vec[i + col])
    }

    pub fn get_number_points(&self, point: &Point) -> PointType {
        let mut points: Vec<(i32, i32)> = vec![];

        let (col, row) = point.position;

        let mut left_index = col - 1;
        while self.index(row as usize, left_index as usize).is_some() {
            if let Some(adjetant) = self.index(row as usize, left_index as usize) {
                match adjetant.t {
                    PointType::Number { .. } => {
                        points.push(adjetant.position);
                        left_index -= 1;
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }

        let mut right_index = col;
        while self.index(row as usize, right_index as usize).is_some() {
            if let Some(adjetant) = self.index(row as usize, right_index as usize) {
                match adjetant.t {
                    PointType::Number { .. } => {
                        points.push(adjetant.position);
                        right_index += 1;
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }

        PointType::Number {
            value: self.get_number(&points),
            points,
        }
    }

    pub fn get_number(&self, points: &Vec<(i32, i32)>) -> i32 {
        self.vec
            .iter()
            .filter(|point| points.contains(&point.position))
            .fold("".to_string(), |curr, point| {
                curr + &point.get_value().to_string()
            })
            .parse::<i32>()
            .unwrap()
    }

    pub fn has_adjacent_symbol(&self, point: &Point) -> bool {
        let (x, y) = point.position;

        let adjacent_cells = vec![
            // up
            self.index((y - 1) as usize, x as usize),
            // top-right
            self.index((y - 1) as usize, (x + 1) as usize),
            //right
            self.index(y as usize, (x + 1) as usize),
            // bottom-right
            self.index((y + 1) as usize, (x + 1) as usize),
            // down
            self.index((y + 1) as usize, x as usize),
            // bottom-down
            self.index((y + 1) as usize, (x - 1) as usize),
            // left
            self.index(y as usize, (x - 1) as usize),
            // top-left
            self.index((y - 1) as usize, (x - 1) as usize),
        ];

        let has_symbol = adjacent_cells
            .iter()
            .filter(|p| p.is_some())
            .map(|p| p.unwrap())
            .any(|p| match p.t {
                PointType::Symbol => true,
                _ => false,
            });

        has_symbol
    }
}

pub fn main() {
    let input = fs::read_to_string("src/03/input.txt").expect("File not found");

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let cells = input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();

    let board = Board::new(cells, rows, cols);

    let points: Vec<PointType> = board
        .vec
        .iter()
        .filter(|&point| point.t != PointType::Symbol && point.t != PointType::Empty)
        .filter(|&point| board.has_adjacent_symbol(point))
        .map(|point| board.get_number_points(point))
        .collect();

    let a: i32 = points
        .iter()
        .enumerate()
        .filter(|(index, point)| {
            !points.iter().enumerate().any(|(index2, point2)| {
                if index >= &index2 {
                    false
                } else {
                    point
                        .get_points()
                        .iter()
                        .find(|p| point2.get_points().contains(p))
                        .is_some()
                }
            })
        })
        .map(|(_, point)| point.get_value())
        .sum();

    println!("Answer a: {}", a);
}
