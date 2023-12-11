use rayon::prelude::*;
use std::fs;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum PipeType {
    Vertical,
    Horizontal,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum PointType {
    Start,
    Pipe(PipeType),
    Empty,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
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

impl Point {
    pub fn new(x: i32, y: i32, t: PointType) -> Point {
        Self {
            position: (x, y),
            t,
        }
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
                'S' => PointType::Start,
                '-' => PointType::Pipe(PipeType::Horizontal),
                '|' => PointType::Pipe(PipeType::Vertical),
                'L' => PointType::Pipe(PipeType::UpRight),
                'J' => PointType::Pipe(PipeType::UpLeft),
                '7' => PointType::Pipe(PipeType::DownLeft),
                'F' => PointType::Pipe(PipeType::DownRight),
                value => panic!("Unknown type {}", value),
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

    fn is_allowed_move(source: &PointType, target: &PointType, direction: Direction) -> bool {
        match direction {
            Direction::Up => match (source, target) {
                (
                    PointType::Start
                    | PointType::Pipe(PipeType::Vertical)
                    | PointType::Pipe(PipeType::UpLeft)
                    | PointType::Pipe(PipeType::UpRight),
                    PointType::Start
                    | PointType::Pipe(PipeType::Vertical)
                    | PointType::Pipe(PipeType::DownLeft)
                    | PointType::Pipe(PipeType::DownRight),
                ) => true,
                _ => false,
            },
            Direction::Right => match (source, target) {
                (
                    PointType::Start
                    | PointType::Pipe(PipeType::Horizontal)
                    | PointType::Pipe(PipeType::UpRight)
                    | PointType::Pipe(PipeType::DownRight),
                    PointType::Start
                    | PointType::Pipe(PipeType::Horizontal)
                    | PointType::Pipe(PipeType::UpLeft)
                    | PointType::Pipe(PipeType::DownLeft),
                ) => true,
                _ => false,
            },
            Direction::Down => match (source, target) {
                (
                    PointType::Start
                    | PointType::Pipe(PipeType::Vertical)
                    | PointType::Pipe(PipeType::DownLeft)
                    | PointType::Pipe(PipeType::DownRight),
                    PointType::Start
                    | PointType::Pipe(PipeType::Vertical)
                    | PointType::Pipe(PipeType::UpLeft)
                    | PointType::Pipe(PipeType::UpRight),
                ) => true,
                _ => false,
            },
            Direction::Left => match (source, target) {
                (
                    PointType::Start
                    | PointType::Pipe(PipeType::Horizontal)
                    | PointType::Pipe(PipeType::UpLeft)
                    | PointType::Pipe(PipeType::DownLeft),
                    PointType::Start
                    | PointType::Pipe(PipeType::Horizontal)
                    | PointType::Pipe(PipeType::UpRight)
                    | PointType::Pipe(PipeType::DownRight),
                ) => true,
                _ => false,
            },
        }
    }

    pub fn get_valid_moves(&self, point: &Point) -> Vec<Point> {
        let (x, y) = point.position;

        let mut valid_moves: Vec<Point> = vec![];

        if let Some(up) = self.index((y - 1) as usize, x as usize) {
            if Board::is_allowed_move(&point.t, &up.t, Direction::Up) {
                valid_moves.push(up.clone());
            }
        }

        if let Some(right) = self.index(y as usize, (x + 1) as usize) {
            if Board::is_allowed_move(&point.t, &right.t, Direction::Right) {
                valid_moves.push(right.clone());
            }
        }

        if let Some(down) = self.index((y + 1) as usize, x as usize) {
            if Board::is_allowed_move(&point.t, &down.t, Direction::Down) {
                valid_moves.push(down.clone());
            }
        }

        if let Some(left) = self.index(y as usize, (x - 1) as usize) {
            if Board::is_allowed_move(&point.t, &left.t, Direction::Left) {
                valid_moves.push(left.clone());
            }
        }

        valid_moves
    }

    fn travel(&self, point: &Point, mut path: Vec<Point>, length: usize) -> Vec<Point> {
        let valid_moves = self.get_valid_moves(&point);

        if path.len() > 1 {
            if let Some(end) = valid_moves.iter().find(|point| point.t == PointType::Start) {
                path.push(end.clone());

                return path;
            }
        }

        let move_point = valid_moves
            .into_iter()
            .find(|point| !path.contains(&point))
            .unwrap();

        path.push(move_point.clone());
        self.travel(&move_point, path, length)
    }

    fn is_inside_path(&self, point: &Point, path: &Vec<(i32, i32)>) -> bool {
        if path.contains(&point.position) {
            return false;
        }

        let (x, y) = point.position;

        let mut collisions = 0;
        for current_x in (0..x).rev() {
            let current_point = self.index(y as usize, current_x as usize).unwrap();

            if !path.contains(&current_point.position) {
                continue;
            }

            let does_collide = match current_point.t {
                PointType::Pipe(PipeType::Vertical)
                | PointType::Pipe(PipeType::UpLeft)
                | PointType::Pipe(PipeType::UpRight) => true,
                _ => false,
            };

            if does_collide {
                collisions += 1;
            }
        }

        collisions % 2 == 1
    }
}

pub fn main() {
    let input = fs::read_to_string("src/10/input.txt").expect("File not found");

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let cells = input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();

    let board = Board::new(cells, rows, cols);

    let start = board
        .vec
        .iter()
        .find(|point| point.t == PointType::Start)
        .unwrap();

    let path = board.travel(start, vec![], 0);

    let path_positions = path
        .iter()
        .map(|point| point.position)
        .collect::<Vec<(i32, i32)>>();

    let a = path.len() / 2;
    let b: usize = board
        .vec
        .par_iter()
        .filter(|point| board.is_inside_path(point, &path_positions))
        .count();

    println!("Answer a: {:?}", a);
    println!("Answer b: {:?}", b);
}
