use std::cell::Cell;
use std::collections::VecDeque;

type Intcode = i32;

pub struct Computer {
    pub input: VecDeque<Intcode>,
    pub output: VecDeque<Intcode>,
    memory: Vec<Intcode>,
    program_counter: Cell<usize>,
    pub halted: bool,
}

impl Computer {
    pub fn new(program: &[Intcode]) -> Self {
        Self {
            input: VecDeque::new(),
            output: VecDeque::new(),
            memory: program.to_vec(),
            program_counter: Cell::new(0),
            halted: false,
        }
    }

    fn read_memory(&self, pos: usize, mode: u8) -> Intcode {
        match mode {
            b'0' => self.memory[self.memory[pos] as usize],
            b'1' => self.memory[pos],
            x => unreachable!("unknown mode: {}", x),
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

    fn set_pc(&self, pos: usize) {
        self.program_counter.set(pos);
    }

    fn write_memory(&mut self, modes: &[u8], value: Intcode) {
        let pos = self.next_pc();
        match modes[0] {
            b'0' => {
                let a = self.memory[pos] as usize;
                self.memory[a] = value
            }
            b'1' => self.memory[pos] = value,
            x => unreachable!("unknown mode (write): {}", x),
        }
    }

    pub fn run(&mut self) {
        loop {
            let mode_instruction = format!("{:05}", self.read_instruction())
                .bytes()
                .collect::<Vec<u8>>();
            // println!(
            //     "PC:{:02} INS:{:?} MEM:{:?} INPUT:{:?} OUTPUT:{:?}",
            //     self.program_counter.get() - 1,
            //     std::str::from_utf8(&mode_instruction).unwrap(),
            //     self.memory,
            //     self.input,
            //     self.output
            // );

            let modes = &mode_instruction[..3];
            let mode_a = modes[2];
            let mode_b = modes[1];

            match std::str::from_utf8(&mode_instruction[3..]).unwrap() {
                "01" => {
                    // ADD
                    let a = self.read_memory(self.next_pc(), mode_a);
                    let b = self.read_memory(self.next_pc(), mode_b);
                    self.write_memory(modes, a + b);
                }
                "02" => {
                    // MULT
                    let a = self.read_memory(self.next_pc(), mode_a);
                    let b = self.read_memory(self.next_pc(), mode_b);
                    self.write_memory(modes, a * b);
                }
                "03" => {
                    // STORE INPUT
                    if self.input.is_empty() {
                        return;
                    }

                    let a = self.input.pop_front().unwrap();
                    self.write_memory(modes, a);
                }
                "04" => {
                    // OUTPUT
                    let a = self.read_memory(self.next_pc(), mode_a);
                    self.output.push_back(a);
                    return;
                }
                "05" => {
                    // jump if true
                    let a = self.read_memory(self.next_pc(), mode_a);
                    let b = self.read_memory(self.next_pc(), mode_b);
                    if a != 0 {
                        self.set_pc(b as usize);
                    }
                }
                "06" => {
                    // jump if false
                    let a = self.read_memory(self.next_pc(), mode_a);
                    let b = self.read_memory(self.next_pc(), mode_b);
                    if a == 0 {
                        self.set_pc(b as usize);
                    }
                }
                "07" => {
                    // less-than
                    let a = self.read_memory(self.next_pc(), mode_a);
                    let b = self.read_memory(self.next_pc(), mode_b);
                    self.write_memory(modes, if a < b { 1 } else { 0 });
                }
                "08" => {
                    // equal
                    let a = self.read_memory(self.next_pc(), mode_a);
                    let b = self.read_memory(self.next_pc(), mode_b);
                    self.write_memory(modes, if a == b { 1 } else { 0 });
                }
                "99" => {
                    self.halted = true;
                    return;
                }
                x => unreachable!("opcode unknown: {:?}", x),
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_example() {
        for case in [(2, 1), (1, 1), (0, 0)].iter() {
            let mut c = Computer::new(&[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);
            c.input.push_back(case.0);
            c.run();

            assert_eq!(c.output.pop_front().unwrap(), case.1);
        }
    }

    #[test]
    fn sample_4() {
        for case in [(9, 1001), (8, 1000), (7, 999)].iter() {
            let mut c = Computer::new(&[
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]);
            c.input.push_back(case.0);
            c.run();
            assert_eq!(c.output.pop_front().unwrap(), case.1);
        }
    }
}
