use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum MirrorType {
    Vertical,
    Horizontal,
    TopRight,
    BottomRight,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum PointType {
    Empty,
    Mirror(MirrorType),
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

    // fn is_allowed_move(&self, target: &Point) -> bool {
    //     self.point_type == PointType::Rock && target.point_type == PointType::Empty
    // }
}

impl Board {
    pub fn new(initial_vec: &Vec<char>, width: usize, height: usize) -> Self {
        let mut vec = Vec::new();

        for (index, value) in initial_vec.iter().enumerate() {
            let x = index % width;
            let y = index / width;

            let t = match value {
                '|' => PointType::Mirror(MirrorType::Vertical),
                '-' => PointType::Mirror(MirrorType::Horizontal),
                '\\' => PointType::Mirror(MirrorType::TopRight),
                '/' => PointType::Mirror(MirrorType::BottomRight),
                '.' => PointType::Empty,
                value => panic!("Unknown type {}", value),
            };

            let point = Point::new(x as i32, y as i32, t);
            vec.push(point);
        }

        Self { vec, width, height }
    }

    fn index(&self, x: i32, y: i32) -> Option<&Point> {
        let x = usize::try_from(x).ok();
        let y = usize::try_from(y).ok();

        if let (Some(x), Some(y)) = (x, y) {
            if (x >= self.width) || (y >= self.height) {
                return None;
            }

            Some(&self.vec[self.width * y + x])
        } else {
            None
        }
    }

    fn index_mut(&mut self, x: usize, y: usize) -> Option<&mut Point> {
        if (x >= self.width) || (y >= self.height) {
            return None;
        }

        Some(&mut self.vec[self.width * y + x])
    }

    fn get_row(&self, y: usize) -> Vec<&Point> {
        (0..self.width)
            .into_iter()
            .filter_map(|x| self.index(x as i32, y as i32))
            .collect()
    }

    fn get_col(&self, x: usize) -> Vec<&Point> {
        (0..self.height)
            .into_iter()
            .filter_map(|y| self.index(x as i32, y as i32))
            .collect()
    }

    fn set_index(&mut self, x: usize, y: usize, point: Point) {
        if (x >= self.width) || (y >= self.height) {
            panic!("Out of bounds index x {} y {}", x, y);
        }

        self.vec[self.width * y + x] = point
    }

    fn print(&self, visited: &Vec<(i32, i32)>) {
        self.vec.iter().enumerate().for_each(|(index, point)| {
            let char = if visited.contains(&point.position) {
                '#'
            } else {
                match point.point_type {
                    PointType::Mirror(MirrorType::Vertical) => '|',
                    PointType::Mirror(MirrorType::Horizontal) => '-',
                    PointType::Mirror(MirrorType::TopRight) => '\\',
                    PointType::Mirror(MirrorType::BottomRight) => '/',
                    PointType::Empty => '.',
                }
            };

            if index % self.width == 0 {
                println!("")
            }
            print!("{}", char);
        });

        println!("");
    }

    pub fn get_next_moves(
        &self,
        point: &Point,
        direction: &Direction,
    ) -> Vec<(Option<&Point>, &Direction)> {
        let (current_x, current_y) = point.position;

        if let Some(target_point) = match direction {
            Direction::Up => self.index(current_x, current_y - 1),
            Direction::Right => self.index(current_x + 1, current_y),
            Direction::Down => self.index(current_x, current_y + 1),
            Direction::Left => self.index(current_x - 1, current_y),
        } {
            let (target_x, target_y) = target_point.position;

            let valid_moves = match direction {
                Direction::Up => match target_point.point_type {
                    PointType::Mirror(MirrorType::Vertical) => {
                        vec![(self.index(target_x, target_y), &Direction::Up)]
                    }
                    PointType::Mirror(MirrorType::Horizontal) => {
                        vec![
                            (self.index(target_x, target_y), &Direction::Left),
                            (self.index(target_x, target_y), &Direction::Right),
                        ]
                    }
                    PointType::Mirror(MirrorType::TopRight) => {
                        vec![(self.index(target_x, target_y), &Direction::Left)]
                    }
                    PointType::Mirror(MirrorType::BottomRight) => {
                        vec![(self.index(target_x, target_y), &Direction::Right)]
                    }
                    PointType::Empty => {
                        vec![(self.index(target_x, target_y), &Direction::Up)]
                    }
                },
                Direction::Right => match target_point.point_type {
                    PointType::Mirror(MirrorType::Vertical) => {
                        vec![
                            (self.index(target_x, target_y), &Direction::Up),
                            (self.index(target_x, target_y), &Direction::Down),
                        ]
                    }
                    PointType::Mirror(MirrorType::Horizontal) => {
                        vec![(self.index(target_x, target_y), &Direction::Right)]
                    }
                    PointType::Mirror(MirrorType::TopRight) => {
                        vec![(self.index(target_x, target_y), &Direction::Down)]
                    }
                    PointType::Mirror(MirrorType::BottomRight) => {
                        vec![(self.index(target_x, target_y), &Direction::Up)]
                    }
                    PointType::Empty => {
                        vec![(self.index(target_x, target_y), &Direction::Right)]
                    }
                },
                Direction::Down => match target_point.point_type {
                    PointType::Mirror(MirrorType::Vertical) => {
                        vec![(self.index(target_x, target_y), &Direction::Down)]
                    }
                    PointType::Mirror(MirrorType::Horizontal) => {
                        vec![
                            (self.index(target_x, target_y), &Direction::Left),
                            (self.index(target_x, target_y), &Direction::Right),
                        ]
                    }
                    PointType::Mirror(MirrorType::TopRight) => {
                        vec![(self.index(target_x, target_y), &Direction::Right)]
                    }
                    PointType::Mirror(MirrorType::BottomRight) => {
                        vec![(self.index(target_x, target_y), &Direction::Left)]
                    }
                    PointType::Empty => {
                        vec![(self.index(target_x, target_y), &Direction::Down)]
                    }
                },
                Direction::Left => match target_point.point_type {
                    PointType::Mirror(MirrorType::Vertical) => {
                        vec![
                            (self.index(target_x, target_y), &Direction::Up),
                            (self.index(target_x, target_y), &Direction::Down),
                        ]
                    }
                    PointType::Mirror(MirrorType::Horizontal) => {
                        vec![(self.index(target_x, target_y), &Direction::Left)]
                    }
                    PointType::Mirror(MirrorType::TopRight) => {
                        vec![(self.index(target_x, target_y), &Direction::Up)]
                    }
                    PointType::Mirror(MirrorType::BottomRight) => {
                        vec![(self.index(target_x, target_y), &Direction::Down)]
                    }
                    PointType::Empty => {
                        vec![(self.index(target_x, target_y), &Direction::Left)]
                    }
                },
            };

            valid_moves
        } else {
            vec![]
        }
    }

    fn travel(&self, point: &Point, direction: &Direction, path: &mut Vec<(Point, Direction)>) {
        if !path
            .iter()
            .find(|(existing_point, existing_direction)| {
                existing_point == point && existing_direction == direction
            })
            .is_some()
        {
            path.push((point.clone(), direction.clone()));

            for next_move in self.get_next_moves(point, &direction).iter_mut() {
                if let (Some(point), direction) = next_move {
                    self.travel(point, direction, path)
                }
                // }
            }
        }
    }

    fn travel_from(&self, position: (i32, i32), direction: &Direction) -> Vec<(Point, Direction)> {
        let path: &mut Vec<(Point, Direction)> = &mut vec![];

        let (x, y) = position;
        let start = self.index(x, y).unwrap().clone();

        let first_directions = match direction {
            Direction::Up => match start.point_type {
                PointType::Mirror(MirrorType::Vertical) => {
                    vec![direction.clone()]
                }
                PointType::Mirror(MirrorType::Horizontal) => {
                    vec![Direction::Left, Direction::Right]
                }
                PointType::Mirror(MirrorType::TopRight) => {
                    vec![Direction::Left]
                }
                PointType::Mirror(MirrorType::BottomRight) => {
                    vec![Direction::Right]
                }
                PointType::Empty => {
                    vec![direction.clone()]
                }
            },
            Direction::Right => match start.point_type {
                PointType::Mirror(MirrorType::Vertical) => {
                    vec![Direction::Up, Direction::Down]
                }
                PointType::Mirror(MirrorType::Horizontal) => {
                    vec![direction.clone()]
                }
                PointType::Mirror(MirrorType::TopRight) => {
                    vec![Direction::Down]
                }
                PointType::Mirror(MirrorType::BottomRight) => {
                    vec![Direction::Up]
                }
                PointType::Empty => {
                    vec![direction.clone()]
                }
            },
            Direction::Down => match start.point_type {
                PointType::Mirror(MirrorType::Vertical) => {
                    vec![direction.clone()]
                }
                PointType::Mirror(MirrorType::Horizontal) => {
                    vec![Direction::Left, Direction::Right]
                }
                PointType::Mirror(MirrorType::TopRight) => {
                    vec![Direction::Right]
                }
                PointType::Mirror(MirrorType::BottomRight) => {
                    vec![Direction::Left]
                }
                PointType::Empty => {
                    vec![direction.clone()]
                }
            },
            Direction::Left => match start.point_type {
                PointType::Mirror(MirrorType::Vertical) => {
                    vec![Direction::Up, Direction::Down]
                }
                PointType::Mirror(MirrorType::Horizontal) => {
                    vec![direction.clone()]
                }
                PointType::Mirror(MirrorType::TopRight) => {
                    vec![Direction::Up]
                }
                PointType::Mirror(MirrorType::BottomRight) => {
                    vec![Direction::Down]
                }
                PointType::Empty => {
                    vec![direction.clone()]
                }
            },
        };

        first_directions.iter().for_each(|direction| {
            self.travel(&start, &Direction::Down, path);
        });

        path.to_vec()
    }
}

pub fn main() {
    let input = fs::read_to_string("src/16/input.txt").expect("File not found");

    let cells: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let board = Board::new(&cells, width, height);

    let path_a = board.travel_from((0, 0), &Direction::Right);

    let positions_a: Vec<(i32, i32)> = path_a.iter().map(|(point, _)| point.position).collect();

    let a = board
        .vec
        .iter()
        .filter(|point| positions_a.contains(&point.position))
        .count();

    let b = board
        .get_row(height)
        .iter()
        .map(|point| (point, Direction::Up))
        .chain(
            board
                .get_col(width)
                .iter()
                .map(|point| (point, Direction::Right)),
        )
        .chain(
            board
                .get_row(0)
                .iter()
                .map(|point| (point, Direction::Down)),
        )
        .chain(
            board
                .get_col(0)
                .iter()
                .map(|point| (point, Direction::Left)),
        )
        .enumerate()
        .map(|(index, (point, direction))| {
            println!("index {} ", index);
            let path_b = board.travel_from(point.position, &direction);

            let positions_b: Vec<(i32, i32)> =
                path_b.iter().map(|(point, _)| point.position).collect();
            board
                .vec
                .iter()
                .filter(|point| positions_b.contains(&point.position))
                .count()
        })
        .max()
        .unwrap();

    println!("a (7608) {}", a);
    println!("b (8221) {}", b);
}
