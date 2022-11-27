use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open(
        std::path::PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()),
        )
        .join("input.txt"),
    )
    .expect("unable to open input.txt");
    let v = read_input(&mut file).expect("parse error");

    let output = run_program(&v, 1);
    println!("OUTPUT: {}", output);
    assert_eq!(output, 8_332_629);

    let output = run_program(&v, 5);
    println!("OUTPUT: {}", output);
    assert_eq!(output, 8_805_067);
}

fn read_input<R: Read>(input: &mut R) -> Result<Vec<i32>, String> {
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

fn get(opcodes: &[i32], mode: char, pos: usize) -> usize {
    match mode {
        '0' => opcodes[pos] as usize,
        '1' => pos,
        x => unreachable!("unknown mode: {}", x),
    }
}

fn get_multiple(opcodes: &[i32], modes: &str, pos: usize, count: usize) -> Vec<usize> {
    let v: Vec<_> = (0..count)
        .zip(modes.chars().rev())
        .map(|(offset, mode)| get(opcodes, mode, pos + offset + 1))
        .collect();
    assert_eq!(v.len(), count);
    v
}

fn run_ops(instructions: &[i32], input: i32, return_pos: usize) -> i32 {
    let mut opcodes = instructions.to_vec();
    let mut pos = 0;
    let mut output = None;

    loop {
        let mode_instruction = format!("{:05}", opcodes[pos]);
        // println!("PC:{:02} INS:{} MEM:{:?}", pos, mode_instruction, opcodes);

        let modes = &mode_instruction[..3];

        let increment = match &mode_instruction[3..] {
            "01" => {
                // ADD
                let v = get_multiple(&opcodes, modes, pos, 3);
                opcodes[v[2]] = opcodes[v[0]] + opcodes[v[1]];
                4
            }
            "02" => {
                // MULT
                let v = get_multiple(&opcodes, modes, pos, 3);
                opcodes[v[2]] = opcodes[v[0]] * opcodes[v[1]];
                4
            }
            "03" => {
                // STORE INPUT
                let v = get_multiple(&opcodes, modes, pos, 1);
                opcodes[v[0]] = input;
                2
            }
            "04" => {
                // OUTPUT
                let v = get_multiple(&opcodes, modes, pos, 1);
                output = Some(opcodes[v[0]]);
                2
            }
            "05" => {
                // jump if true
                let v = get_multiple(&opcodes, modes, pos, 2);
                if opcodes[v[0]] != 0 {
                    pos = opcodes[v[1]] as usize;
                    0
                } else {
                    3
                }
            }
            "06" => {
                // jump if false
                let v = get_multiple(&opcodes, modes, pos, 2);
                if opcodes[v[0]] == 0 {
                    pos = opcodes[v[1]] as usize;
                    0
                } else {
                    3
                }
            }
            "07" => {
                // less-than
                let v = get_multiple(&opcodes, modes, pos, 3);
                opcodes[v[2]] = i32::from(opcodes[v[0]] < opcodes[v[1]]);
                4
            }
            "08" => {
                // equal
                let v = get_multiple(&opcodes, modes, pos, 3);
                opcodes[v[2]] = i32::from(opcodes[v[0]] == opcodes[v[1]]);
                4
            }
            "99" => {
                return if let Some(output) = output {
                    output
                } else {
                    opcodes[return_pos]
                };
            }
            x => unreachable!("opcode unknown: {}", x),
        };

        pos += increment;
    }
}

#[inline]
fn run_program(instructions: &[i32], input: i32) -> i32 {
    run_ops(instructions, input, 0)
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
    fn simple_example() {
        assert_eq!(
            run_program(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], 0),
            3500
        );
    }

    #[test]
    fn sample_1() {
        assert_eq!(run_program(&[1, 0, 0, 0, 99], 0), 2);
    }

    #[test]
    fn advanced() {
        for case in [
            (&[2, 4, 4, 5, 99, 0][..], 5, 9801),
            (&[2, 3, 0, 3, 99][..], 3, 6),
            (&[1, 1, 1, 4, 99, 5, 6, 0, 99][..], 0, 30),
            (&[1, 1, 1, 4, 99, 5, 6, 0, 99][..], 4, 2),
        ]
        .iter()
        {
            assert_eq!(run_ops(case.0, 0, case.1), case.2);
        }
    }

    #[test]
    fn sample_2() {
        assert!(run_program(&[1002, 4, 3, 4, 33], 0) > 0);
        assert_eq!(run_ops(&[1101, 100, -1, 4, 0], 0, 4), 99);
    }

    #[test]
    fn sample_3() {
        for case in [(2, 1), (1, 1), (0, 0)].iter() {
            assert_eq!(
                run_program(
                    &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                    case.0,
                ),
                case.1
            );

            assert_eq!(
                run_program(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], case.0,),
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
                        46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                    ],
                    case.0,
                ),
                case.1
            );
        }
    }
}
