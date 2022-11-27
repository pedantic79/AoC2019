use multimap::MultiMap;
use num::Integer;
use std::f64::consts::PI;
use std::fmt;

type Int = i64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Asteroid {
    x: Int,
    y: Int,
}

impl fmt::Display for Asteroid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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
        let gcd = dx.gcd(&dy);

        Direction(dx / gcd, dy / gcd)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Direction(Int, Int);

impl Direction {
    fn angle(&self) -> f64 {
        let radians = (self.1 as f64).atan2(self.0 as f64) - PI / 2.0;
        radians.rem_euclid(PI * 2.0)
    }
}

fn f64_total_ordering(f: f64) -> i64 {
    let mut bits = f.to_bits() as i64;
    bits ^= (((bits >> 63) as u64) >> 1) as i64;
    bits
}

fn main() {
    let mut file = std::fs::File::open(
        std::path::PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()),
        )
        .join("input.txt"),
    )
    .expect("unable to open input.txt");
    let asteroids = read_input(&mut file).unwrap();

    let (station, mut visible) = asteroids
        .iter()
        .map(|station| get_directions(station, &asteroids))
        .max_by_key(|(_, directions)| directions.len())
        .unwrap();

    println!("Station {} sees {} asteroids", station, visible.len());
    debug_assert_eq!(visible.len(), 344);

    let mut v = visible.keys().copied().collect::<Vec<Direction>>();
    v.sort_by_cached_key(|dir| f64_total_ordering(dir.angle()));

    let v = visible.get_vec_mut(&v[199]).unwrap();
    v.sort_by_key(|x| x.distance(&station));
    let answer = v[0].x * 100 + v[0].y;

    println!("200th asteroid destroyed: {} answer: {}", v[0], answer);
    debug_assert_eq!(answer, 2732);
}

fn get_directions(
    station: &Asteroid,
    asteroids: &[Asteroid],
) -> (Asteroid, MultiMap<Direction, Asteroid>) {
    (
        *station,
        asteroids
            .iter()
            .filter(|a| *a != station)
            .map(|a| (a.get_direction(station), *a))
            .collect(),
    )
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
