mod intcode;

use intcode::{Computer, Intcode};
use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open(
        std::path::PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()),
        )
        .join("input.txt"),
    )
    .expect("unable to open input.txt");
    let v = read_input(&mut file).expect("parse error");

    let max_amp = permute::permute(vec![0, 1, 2, 3, 4])
        .iter()
        .map(|phases| run_amplifier(&v, 0, phases))
        .max();

    println!("MAX_AMP: {:?}", max_amp);
    debug_assert_eq!(max_amp, Some(255_590));

    let max_feedback = permute::permute(vec![9, 8, 7, 6, 5])
        .iter()
        .map(|phases| {
            let mut amps: Vec<_> = phases
                .iter()
                .map(|&i| {
                    let mut c = Computer::new(&v);
                    c.input_add(i);
                    c
                })
                .collect();

            let mut input = 0;
            for (idx, _) in phases.iter().enumerate().cycle() {
                amps[idx].input_add(input);
                amps[idx].run();
                if amps[idx].halted {
                    break;
                }

                input = amps[idx].output_get();
            }
            input
        })
        .max();

    println!("FEEDBACK: {:?}", max_feedback);
    debug_assert_eq!(max_feedback, Some(58_285_150))
}

fn read_input<R: Read>(input: &mut R) -> Result<Vec<Intcode>, String> {
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

fn run_amplifier(opcodes: &[Intcode], input: Intcode, phase: &[Intcode]) -> Intcode {
    phase.iter().fold(input, |next_input, next_phase| {
        run_program(opcodes, &[*next_phase, next_input])
    })
}

fn run_program(instructions: &[Intcode], input: &[Intcode]) -> Intcode {
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
    fn sample_aoc7() {
        assert_eq!(
            run_amplifier(
                &[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,],
                0,
                &[4, 3, 2, 1, 0],
            ),
            43210
        )
    }
}
