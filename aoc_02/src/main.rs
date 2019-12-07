use std::fs::File;
use std::io::Read;

const TARGET: usize = 19_690_720;

fn main() {
    let mut file = File::open("input.txt").expect("unable to open input.txt");
    let mut v = read_input(&mut file).expect("parse error");

    v[1] = 12;
    v[2] = 2;
    assert_eq!(run_program(&v), 10_566_835);
    println!("Output: {}", run_program(&v));

    v[2] = 0;
    let noun = (0..99)
        .rev()
        .find(|&noun| {
            v[1] = noun;
            run_program(&v) <= TARGET
        })
        .expect("no valid nouns found");

    let verb = (0..99)
        .rev()
        .find(|&noun| {
            v[2] = noun;
            run_program(&v) == TARGET
        })
        .expect("no valid verbs found");

    assert_eq!(run_program(&v), TARGET);
    println!("Noun Verb: {}", noun * 100 + verb);
}

fn read_input<R: Read>(input: &mut R) -> Result<Vec<usize>, String> {
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

fn run_ops(input: &[usize], return_pos: usize) -> usize {
    let mut opcodes = input.to_vec();
    let mut pos = 0;

    loop {
        match opcodes[pos] {
            op @ 1 | op @ 2 => {
                let a = opcodes[pos + 1];
                let b = opcodes[pos + 2];
                let c = opcodes[pos + 3];

                opcodes[c] = if op == 1 {
                    opcodes[a] + opcodes[b]
                } else {
                    opcodes[a] * opcodes[b]
                };
            }
            99 => return opcodes[return_pos],
            x => unreachable!("opcode unknown: {}", x),
        }
        pos += 4;
    }
}

#[inline]
fn run_program(input: &[usize]) -> usize {
    run_ops(input, 0)
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
            run_program(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            3500
        );
    }

    #[test]
    fn sample_1() {
        assert_eq!(run_program(&[1, 0, 0, 0, 99]), 2);
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
            assert_eq!(run_ops(case.0, case.1), case.2);
        }
    }
}
