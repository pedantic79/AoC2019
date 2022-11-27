mod intcode;

use intcode::{read_input, run_program};
use std::fs::File;

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

    println!("Part 1: {:?}", output);
    debug_assert_eq!(output, 2_941_952_859);

    let output = run_program(&v, 2);
    println!("Part 2: {:?}", output);
    debug_assert_eq!(output, 66_113);
}
