use std::fs::File;
use std::io::Read;
use std::iter::successors;

fn main() {
    let mut file = File::open("input.txt").expect("unable to open input.txt");
    let v = read_input(&mut file).expect("parse error");

    let total: u32 = v.iter().filter_map(|&x| fuel(x)).sum();
    println!("Total fuel: {}", total);

    let real_total: u32 = v.iter().map(|&x| fuel_of_fuel(x)).sum();
    println!("Total fuel of fuel: {}", real_total);
}

fn fuel(mass: u32) -> Option<u32> {
    (mass / 3).checked_sub(2)
}

fn fuel_of_fuel(mass: u32) -> u32 {
    successors(Some(mass), |&current_mass| fuel(current_mass))
        .skip(1)
        .sum()
}

fn read_input<R: Read>(input: &mut R) -> Result<Vec<u32>, String> {
    let mut buffer = String::new();
    if let Err(msg) = input.read_to_string(&mut buffer) {
        return Err(msg.to_string());
    }

    buffer
        .lines()
        .map(|s| {
            s.parse::<_>()
                .map_err(|e: std::num::ParseIntError| format!("{}: {}", e, s))
        })
        .collect::<Result<_, _>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn problem_part1() {
        for case in [(12, 2), (14, 2), (1969, 654), (100_756, 33583)].iter() {
            assert_eq!(fuel(case.0), Some(case.1));
        }
    }

    #[test]
    fn parsing_file() {
        let mut input: &[u8] = b"123\n9\n45678\n12\n234\n";
        assert_eq!(read_input(&mut input), Ok(vec![123, 9, 45678, 12, 234]));
    }

    #[test]
    fn problem_part2() {
        for case in [(14, 2), (1969, 966), (100_756, 50346)].iter() {
            assert_eq!(fuel_of_fuel(case.0), case.1);
        }
    }
}
