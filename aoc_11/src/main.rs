mod intcode;

use intcode::{read_input, Computer, ComputerStatus, Intcode};
use std::collections::HashMap;
use std::fs::File;

type Color = Intcode;

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn right(&self) -> Self {
        use Direction::*;

        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn left(&self) -> Self {
        use Direction::*;

        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point(isize, isize);

impl Point {
    fn forward(&self, direction: &Direction) -> Self {
        use Direction::*;
        let (x, y) = match direction {
            Up => (0, 1),
            Left => (-1, 0),
            Down => (0, -1),
            Right => (1, 0),
        };

        Self(self.0 + x, self.1 + y)
    }
}

struct Robot {
    current: Point,
    board: HashMap<Point, Color>,
    direction: Direction,
}

impl Robot {
    fn new() -> Self {
        Self {
            current: Point(0, 0),
            direction: Direction::Up,
            board: HashMap::new(),
        }
    }

    fn get(&self) -> Color {
        self.board.get(&self.current).copied().unwrap_or(0)
    }

    fn set(&mut self, c: Color) {
        self.board.insert(self.current, c);
    }

    fn turn_left(&mut self) {
        let new = self.direction.left();
        self.direction = new;
    }

    fn turn_right(&mut self) {
        let new = self.direction.right();
        self.direction = new;
    }

    fn forward(&mut self) {
        let new = self.current.forward(&self.direction);
        self.current = new;
    }

    fn run_robot(&mut self, v: &[Intcode]) {
        let mut c = Computer::new(v);

        'main: loop {
            let color = loop {
                match c.run() {
                    ComputerStatus::Halt => break 'main,
                    ComputerStatus::WaitingForInput => c.input_add(self.get()),
                    ComputerStatus::ReturnedValue => {
                        break c.output_get();
                    }
                }
            };

            let turn = loop {
                match c.run() {
                    ComputerStatus::Halt => break 'main,
                    ComputerStatus::WaitingForInput => c.input_add(self.get()),
                    ComputerStatus::ReturnedValue => {
                        break c.output_get();
                    }
                }
            };

            self.set(color);
            if turn == 0 {
                self.turn_left();
            } else {
                self.turn_right();
            }
            self.forward();
        }
    }
}

fn draw_output(image: &HashMap<Point, Color>) {
    let (min_x, min_y, max_x, max_y) = image
        .keys()
        .fold(
            None,
            |acc: Option<(isize, isize, isize, isize)>, &Point(x, y)| {
                if let Some((min_x, min_y, max_x, max_y)) = acc {
                    Some((min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y)))
                } else {
                    Some((x, y, x, y))
                }
            },
        )
        .unwrap();

    let x = (max_x - min_x) as usize + 1;
    let y = (max_y - min_y) as usize + 1;

    let mut output = vec![vec![' '; x]; y];

    for (r, row) in output.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            let old_r = r as isize + min_y;
            let old_c = c as isize + min_x;
            if let Some(color) = image.get(&Point(old_c, old_r)) {
                if *color == 1 {
                    *cell = '█';
                }
            }
        }
    }

    output
        .iter()
        .rev()
        .for_each(|row| println!("|{}|", row.iter().collect::<String>()));
}

fn main() {
    let mut file = File::open(
        std::path::PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()),
        )
        .join("input.txt"),
    )
    .expect("unable to open input.txt");
    let v = read_input(&mut file).expect("parse error");

    let mut r = Robot::new();
    r.run_robot(&v);
    let part1 = r.board.keys().count();

    println!("Squares: {}", part1);
    debug_assert_eq!(part1, 1894);

    let mut r2 = Robot::new();
    r2.set(1);
    r2.run_robot(&v);

    draw_output(&r2.board);
}
