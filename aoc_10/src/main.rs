use multimap::MultiMap;
use num::Integer;
// use std::collections::{HashMap, HashSet};

type Int = i64;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Asteroid {
    x: Int,
    y: Int,
}

impl Asteroid {
    fn new(x: Int, y: Int) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Self) -> Int {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        dx.pow(2) + dy.pow(2)
    }

    fn get_direction(&self, other: &Self) -> Direction {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let gcd = dx.gcd(&dy).abs();

        Direction(dx / gcd, dy / gcd)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Direction(Int, Int);

fn main() {
    let mut file = std::fs::File::open("input.txt").expect("unable to open input.txt");
    let asteroids = read_input(&mut file).unwrap();

    let station = asteroids
        .iter()
        .map(|station| get_directions(station, &asteroids))
        .max_by_key(|station| station.len())
        .unwrap();

    println!("Visible: {:?}", station.len());
    debug_assert_eq!(station.len(), 344);
}

fn get_directions(station: &Asteroid, asteroids: &[Asteroid]) -> MultiMap<Direction, Asteroid> {
    asteroids
        .iter()
        .filter(|a| *a != station)
        .map(|a| (a.get_direction(station), *a))
        .collect()
}

fn read_input<R: std::io::Read>(input: &mut R) -> Result<Vec<Asteroid>, String> {
    let mut buffer = String::new();
    if let Err(msg) = input.read_to_string(&mut buffer) {
        return Err(msg.to_string());
    }

    Ok(buffer
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, loc)| *loc == '#')
                .map(move |(x, _)| Asteroid::new(x as Int, y as Int))
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        let mut input: &[u8] = b".#..#\n.....\n#####\n....#\n...##";
        let v = read_input(&mut input).unwrap();

        assert_eq!(v.len(), 10);
    }
}
