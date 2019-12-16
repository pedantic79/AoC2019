mod intcode;

use intcode::{read_input, Computer, ComputerStatus, Intcode};
use std::cmp::Ordering;
use std::fmt;
use std::fs::File;

struct Screen {
    screen: Vec<Vec<Intcode>>,
    score: Intcode,
}

impl Screen {
    fn new(x: usize, y: usize) -> Self {
        Self {
            screen: vec![vec![0; x]; y],
            score: 0,
        }
    }

    fn get_mut(&mut self, y: usize, x: usize) -> &mut Intcode {
        &mut self.screen[y][x]
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", termion::cursor::Goto(1, 1))?;
        writeln!(f, "SCORE: {}", self.score)?;

        for (c, cols) in self.screen.iter().enumerate() {
            for (r, cell) in cols.iter().enumerate() {
                let character = match cell {
                    0 => ' ',
                    1 => '█',
                    2 => '░',
                    3 => '=',
                    4 => '●',
                    x => unreachable!("Unknown kind {}x{} = {}", r, c, x),
                    // _ => ' ',
                };
                write!(f, "{}", character)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("unable to open input.txt");
    let mut v = read_input(&mut file).expect("parse error");

    let max_y = v[60] + 1;
    let max_x = v[49] + 1;
    v[0] = 2;

    let mut screen = Screen::new(max_x as usize, max_y as usize);
    let mut c = Computer::new(&v);
    let mut count = 0;
    let mut ball_x = 0;
    let mut cursor_x = 0;

    println!("{}", termion::clear::All);
    loop {
        let status = c.run();
        match status {
            ComputerStatus::ReturnedValue => {
                while c.output.len() > 2 {
                    let x = c.output_get();
                    let y = c.output_get();
                    let kind = c.output_get();

                    if x < 0 {
                        screen.score = kind;
                    } else {
                        *screen.get_mut(y as usize, x as usize) = kind;
                    }

                    match kind {
                        2 => count += 1,
                        4 => ball_x = x,
                        3 => cursor_x = x,
                        _ => (),
                    }
                }
            }
            ComputerStatus::WaitingForInput => {
                std::thread::sleep(std::time::Duration::from_millis(100));
                println!("{}", screen);
                let mv = match ball_x.cmp(&cursor_x) {
                    Ordering::Equal => 0,
                    Ordering::Less => -1,
                    Ordering::Greater => 1,
                };

                c.input_add(mv);
            }
            ComputerStatus::Halt => break,
        }
    }

    println!("{}", screen);
    println!("There are {} blocks", count);
}
