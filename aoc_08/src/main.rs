use bytecount::naive_count_32;
use itertools::Itertools;
use std::fs::File;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let mut file = File::open("input.txt").expect("unable to open input.txt");
    let layers = read_input(&mut file).expect("parse error");

    {
        let target = layers
            .iter()
            .min_by_key(|buffer| naive_count_32(buffer, b'0'))
            .expect("no min found");

        let ones = naive_count_32(target, b'1');
        let twos = naive_count_32(target, b'2');
        println!("Answer Part1: {}", ones * twos);
        assert_eq!(ones * twos, 2480);
    }

    let merged = layers
        .into_iter()
        .rev()
        .fold1(|mut acc, layer| {
            merge(&mut acc, &layer);
            acc
        })
        .unwrap();

    merged.chunks(WIDTH).for_each(|row| {
        let s: String = row
            .iter()
            .map(|&byte| if byte == b'0' { ' ' } else { byte as char })
            .collect();
        println!("{}", s);
    });
}

fn merge(old_layer: &mut [u8], new_layer: &[u8]) {
    for (old, &new) in old_layer.iter_mut().zip(new_layer.iter()) {
        if new != b'2' {
            *old = new
        }
    }
}

fn read_input<R: std::io::Read>(input: &mut R) -> Result<Vec<Vec<u8>>, String> {
    let mut buffer = Vec::new();
    if let Err(msg) = input.read_to_end(&mut buffer) {
        return Err(msg.to_string());
    }

    buffer.retain(|c| c.is_ascii_alphanumeric());

    Ok(buffer
        .chunks(WIDTH * HEIGHT)
        .map(|chunk| chunk.to_vec())
        .collect())
}
