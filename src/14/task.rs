use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum PointType {
    Cube,
    Rock,
    Empty,
}

#[derive(Debug, Hash, PartialEq, Clone, Eq)]
struct Point {
    position: (usize, usize),
    point_type: PointType,
}

#[derive(Debug)]
struct Board {
    pub vec: Vec<Point>,
    width: usize,
    height: usize,
}

impl Point {
    pub fn new(x: usize, y: usize, point_type: PointType) -> Point {
        Self {
            position: (x, y),
            point_type,
        }
    }

    fn is_allowed_move(&self, target: &Point) -> bool {
        self.point_type == PointType::Rock && target.point_type == PointType::Empty
    }
}

impl Board {
    pub fn new(initial_vec: &Vec<char>, width: usize, height: usize) -> Self {
        let mut vec = Vec::new();

        for (index, value) in initial_vec.iter().enumerate() {
            let x = index % width;
            let y = index / width;

            let t = match value {
                'O' => PointType::Rock,
                '#' => PointType::Cube,
                '.' => PointType::Empty,
                value => panic!("Unknown type {}", value),
            };

            let point = Point::new(x, y, t);
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

    fn set_index(&mut self, x: usize, y: usize, point: Point) {
        if (x >= self.width) || (y >= self.height) {
            panic!("Out of bounds index x {} y {}", x, y);
        }

        self.vec[self.width * y + x] = point
    }

    fn get_table_string(&self) -> String {
        String::from_iter(self.vec.iter().map(|char| match char.point_type {
            PointType::Cube => '#',
            PointType::Rock => 'O',
            PointType::Empty => '.',
        }))
    }

    fn _print(&self) {
        self.vec.iter().enumerate().for_each(|(index, point)| {
            let char = match point.point_type {
                PointType::Cube => "#",
                PointType::Rock => "O",
                PointType::Empty => ".",
            };

            if index % self.width == 0 {
                println!("")
            }
            print!("{}", char);
        });

        println!("");
    }

    fn get_load(&self) -> usize {
        self.vec
            .iter()
            .filter(|point| point.point_type == PointType::Rock)
            .map(|point| self.height - point.position.1)
            .sum()
    }

    fn move_cells(&mut self, direction: &Direction) {
        let mut moved;

        let bounds = match direction {
            Direction::North => (0, self.width, 1, self.height),
            Direction::East => (0, self.width - 1, 0, self.height),
            Direction::South => (0, self.width, 0, self.height - 1),
            Direction::West => (1, self.width, 0, self.height),
        };

        loop {
            moved = false;

            (bounds.0..bounds.1).into_iter().for_each(|x| {
                (bounds.2..bounds.3).into_iter().for_each(|y| {
                    let current_cell = self.index(x, y).unwrap();

                    let (target_x, target_y) = match direction {
                        Direction::North => (x, y - 1),
                        Direction::East => (x + 1, y),
                        Direction::South => (x, y + 1),
                        Direction::West => (x - 1, y),
                    };
                    // let (target_x, target_y) = (x - 1, y - target_x_offset);
                    let target_cell = self.index(target_x, target_y).unwrap();

                    if current_cell.is_allowed_move(target_cell) {
                        self.set_index(
                            current_cell.position.0,
                            current_cell.position.1,
                            Point {
                                position: (current_cell.position.0, current_cell.position.1),
                                point_type: PointType::Empty,
                            },
                        );
                        self.set_index(
                            target_x,
                            target_y,
                            Point {
                                position: (target_x, target_y),
                                point_type: PointType::Rock,
                            },
                        );

                        moved = true;
                    }
                })
            });
            //
            if !moved {
                break;
            }
        }
    }
}

fn part_a(mut board: Board) -> usize {
    board.move_cells(&Direction::North);
    board.get_load()
}

fn part_b(mut board: Board) -> usize {
    let mut map: HashSet<String> = HashSet::new();
    let mut looping_items: Vec<usize> = vec![];

    loop {
        map.insert(board.get_table_string());

        for direction in vec![
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .iter()
        {
            board.move_cells(&direction);
        }

        if map.get(&(board.get_table_string())).is_some() {
            let load = board.get_load();

            if looping_items.iter().any(|item| load == *item) {
                break;
            }

            looping_items.push(load);
        }
    }

    board.get_load()
}

pub fn main() {
    let input = fs::read_to_string("src/14/input.txt").expect("File not found");

    let cells: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    println!("Answer a: {}", part_a(Board::new(&cells, width, height)));
    println!("Answer b: {}", part_b(Board::new(&cells, width, height)));
}
