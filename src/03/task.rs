use std::fs;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum PointType {
    Symbol,
    Number(i32),
    Empty,
}

#[derive(Debug, Hash, PartialEq, Clone, Copy, Eq)]
struct Point {
    position: (i32, i32),
    t: PointType,
}

struct Board {
    pub vec: Vec<Point>,
    row: usize,
    col: usize,
}

impl Point {
    pub fn new(x: i32, y: i32, t: PointType) -> Point {
        Self {
            position: (x, y),
            t,
        }
    }

    fn get_value(&self) -> i32 {
        match self.t {
            PointType::Number(v) => v,
            _ => panic!("None"),
        }
    }
}

impl Board {
    pub fn new(initial_vec: Vec<char>, row: usize, col: usize) -> Self {
        assert!(initial_vec.len() == row * col);

        let mut vec = Vec::new();

        for (index, value) in initial_vec.iter().enumerate() {
            let x = index % col;
            let y = index / col;

            let t = match value {
                '.' => PointType::Empty,
                value => {
                    if value.is_digit(10) {
                        PointType::Number(value.to_string().parse::<i32>().unwrap())
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

    pub fn get_number_points(&self, point: Point) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let (col, row) = point.position;

        let mut left_index = col - 1;
        while self.index(row as usize, left_index as usize).is_some() {
            if let Some(adjetant) = self.index(row as usize, left_index as usize) {
                match adjetant.t {
                    PointType::Number(_) => {
                        points.push(*adjetant);
                        left_index -= 1;
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }

        let mut right_index = col + 1;
        while self.index(row as usize, right_index as usize).is_some() {
            if let Some(adjetant) = self.index(row as usize, right_index as usize) {
                match adjetant.t {
                    PointType::Number(_) => {
                        points.push(*adjetant);
                        right_index += 1;
                        continue;
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }

        points.push(point);
        points.sort_by(|a, b| a.position.0.cmp(&b.position.0));

        points
    }

    pub fn get_number(&self, points: Vec<Point>) -> i32 {
        points
            .iter()
            .fold("".to_string(), |curr, &point| {
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
        ];

        let has_symbol = adjacent_cells
            .into_iter()
            .filter(|p| p.is_some())
            .map(|p| p.unwrap())
            .find(|p| match p.t {
                PointType::Symbol => true,
                _ => false,
            })
            .is_some();

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

    let mut points: Vec<i32> = board
        .vec
        .iter()
        .filter(|point| point.t != PointType::Symbol && point.t != PointType::Empty)
        .filter(|point| board.has_adjacent_symbol(*point))
        .map(|point| board.get_number(board.get_number_points(*point)))
        .collect();

    points.dedup();

    let a: i32 = points.iter().sum();

    println!("Answer a: {}", a);
}
