use num::ToPrimitive;
use std::cell::Cell;
use std::collections::VecDeque;
use std::io::Read;

pub type Intcode = i64;

pub struct Computer {
    input: VecDeque<Intcode>,
    output: Vec<Intcode>,
    memory: Vec<Intcode>,
    program_counter: Cell<usize>,
    base_offset: isize,
    pub halted: bool,
}

#[derive(Debug)]
pub enum ComputerStatus {
    Halt,
    WaitingForInput,
    ReturnedValue,
}

impl Computer {
    pub fn new(program: &[Intcode]) -> Self {
        Self {
            input: VecDeque::new(),
            output: Vec::new(),
            memory: program.to_vec(),
            program_counter: Cell::new(0),
            halted: false,
            base_offset: 0,
        }
    }

    pub fn input_add(&mut self, input: Intcode) {
        self.input.push_back(input)
    }

    #[allow(dead_code)]
    pub fn input_add_all<'a, I: IntoIterator<Item = &'a Intcode>>(&mut self, input: I) {
        self.input.extend(input)
    }

    pub fn output_get(&mut self) -> Intcode {
        self.output.pop().unwrap()
    }

    fn write_memory(&mut self, mode: u8, value: Intcode) {
        let pos = self.next_pc();
        let mpos = match mode {
            b'0' => self.memory[pos].to_usize(),
            b'1' => unimplemented!(),
            b'2' => (self.base_offset as Intcode + self.memory[pos]).to_usize(),
            x => unreachable!("unknown mode (write): {}", x as char),
        }
        .unwrap();

        if mpos >= self.memory.len() {
            self.memory.resize(mpos + 1, 0);
        }
        self.memory[mpos] = value;
    }

    fn read_memory(&self, pos: usize, mode: u8) -> Intcode {
        let mpos = self.memory[pos];

        if let Some(location) = match mode {
            b'0' => mpos.to_usize(),
            b'1' => Some(pos),
            b'2' => (self.base_offset as Intcode + mpos).to_usize(),
            x => unreachable!("unknown mode: {}", x as char),
        } {
            self.memory.get(location).copied().unwrap_or(0)
        } else {
            0
        }
    }

    fn read_instruction(&self) -> Intcode {
        self.read_memory(self.next_pc(), b'1')
    }

    fn next_pc(&self) -> usize {
        let current = self.program_counter.get();
        self.program_counter.set(current + 1);
        current
    }

    fn prev_pc(&self) -> usize {
        let current = self.program_counter.get();
        self.program_counter.set(current - 1);
        current
    }

    fn set_pc(&self, pos: usize) {
        self.program_counter.set(pos);
    }

    pub fn run(&mut self) -> ComputerStatus {
        loop {
            let full_instruction = format!("{:05}", self.read_instruction())
                .bytes()
                .collect::<Vec<u8>>();

            assert_eq!(full_instruction.len(), 5);
            // println!(
            //     "PC:{:02} BO:{} INS:{:?} NXT:{:?} MEM:{:?} IN:{:?} OUT:{:?}",
            //     self.program_counter.get() - 1,
            //     self.base_offset,
            //     std::str::from_utf8(&full_instruction).unwrap(),
            //     &self.memory[self.program_counter.get()..(self.program_counter.get() + 4)],
            //     self.memory,
            //     self.input,
            //     self.output
            // );

            let (mode_c, mode_b, mode_a) = (
                full_instruction[0],
                full_instruction[1],
                full_instruction[2],
            );

            let instruction = std::str::from_utf8(&full_instruction[3..])
                .expect("invalid utf8")
                .parse()
                .expect("not a number");

            match instruction {
                1 => {
                    // ADD
                    let a = self.read_memory(self.next_pc(), mode_a);
                    let b = self.read_memory(self.next_pc(), mode_b);
                    self.write_memory(mode_c, a + b);
                }
                2 => {
                    // MULT
                    let a = self.read_memory(self.next_pc(), mode_a);
                    let b = self.read_memory(self.next_pc(), mode_b);
                    self.write_memory(mode_c, a * b);
                }
                3 => {
                    // STORE INPUT
                    if self.input.is_empty() {
                        self.prev_pc();
                        return ComputerStatus::WaitingForInput;
                    }

                    let a = self.input.pop_front().unwrap();
                    self.write_memory(mode_a, a);
                }
                4 => {
                    // OUTPUT
                    let a = self.read_memory(self.next_pc(), mode_a);
                    self.output.push(a);
                    return ComputerStatus::ReturnedValue;
                }
                5 => {
                    // jump if true
                    let a = self.read_memory(self.next_pc(), mode_a);
                    let b = self.read_memory(self.next_pc(), mode_b);
                    if a != 0 {
                        self.set_pc(b as usize);
                    }
                }
                6 => {
                    // jump if false
                    let a = self.read_memory(self.next_pc(), mode_a);
                    let b = self.read_memory(self.next_pc(), mode_b);
                    if a == 0 {
                        self.set_pc(b as usize);
                    }
                }
                7 => {
                    // less-than
                    let a = self.read_memory(self.next_pc(), mode_a);
                    let b = self.read_memory(self.next_pc(), mode_b);
                    self.write_memory(mode_c, if a < b { 1 } else { 0 });
                }
                8 => {
                    // equal
                    let a = self.read_memory(self.next_pc(), mode_a);
                    let b = self.read_memory(self.next_pc(), mode_b);
                    self.write_memory(mode_c, if a == b { 1 } else { 0 });
                }
                9 => {
                    let a = self.read_memory(self.next_pc(), mode_a);
                    let base = self.base_offset as Intcode + a;
                    self.base_offset = base as isize;
                }
                99 => {
                    self.halted = true;
                    return ComputerStatus::Halt;
                }
                x => unreachable!("opcode unknown: {:?}", x),
            };
        }
    }
}

pub fn read_input<R: Read>(input: &mut R) -> Result<Vec<Intcode>, String> {
    let mut buffer = String::new();
    if let Err(msg) = input.read_to_string(&mut buffer) {
        return Err(msg.to_string());
    }

    buffer
        .split(',')
        .map(|s| s.trim())
        .map(|s| {
            s.parse::<_>()
                .map_err(|e: std::num::ParseIntError| format!("{}: {}", e, s))
        })
        .collect::<Result<_, _>>()
}

pub fn run_program(instructions: &[Intcode], input: Intcode) -> Intcode {
    let mut c = Computer::new(instructions);
    c.input_add(input);
    c.run();
    c.output_get()
}

#[allow(dead_code)]
pub fn run_program_n(instructions: &[Intcode], input: &[Intcode]) -> Intcode {
    let mut c = Computer::new(instructions);
    c.input_add_all(input.iter());
    c.run();
    c.output_get()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing_file() {
        let mut input: &[u8] = b"123, 9,45678 ,12,234,2,3,4";
        assert_eq!(
            read_input(&mut input),
            Ok(vec![123, 9, 45678, 12, 234, 2, 3, 4])
        );
    }

    #[test]
    fn reddit() {
        for case in [
            (vec![109, -1, 4, 1, 99], -1),
            (vec![109, -1, 104, 1, 99], 1),
            (vec![109, -1, 204, 1, 99], 109),
            (vec![109, 1, 9, 2, 204, -6, 99], 204),
            (vec![109, 1, 109, 9, 204, -6, 99], 204),
            (vec![109, 1, 209, -1, 204, -106, 99], 204),
            (vec![109, 1, 3, 3, 204, 2, 99], 1),
        ]
        .iter()
        {
            assert_eq!(run_program(&case.0, 1), case.1);
            assert_eq!(run_program(&case.0, 1), run_program_n(&case.0, &[1]));
        }
    }

    #[test]
    fn debug() {
        assert_eq!(run_program(&[109, 1, 203, 2, 204, 2, 99], 42), 42);
    }

    #[test]
    fn simple_example() {
        for case in [(2, 1), (1, 1), (0, 0)].iter() {
            assert_eq!(
                run_program(
                    &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                    case.0,
                ),
                case.1
            );
        }
    }

    #[test]
    fn sample_4() {
        for case in [(9, 1001), (8, 1000), (7, 999)].iter() {
            assert_eq!(
                run_program(
                    &[
                        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106,
                        0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1,
                        46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                    ],
                    case.0,
                ),
                case.1
            );
        }
    }
}
