use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Asteroid {
    x: usize,
    y: usize,
}

impl Asteroid {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Self) -> i64 {
        let (x1, y1) = (self.x as i64, self.y as i64);
        let (x2, y2) = (other.x as i64, other.y as i64);

        (x2 - x1).pow(2) + (y2 - y1).pow(2)
    }

    fn get_angle(&self, other: &Self) -> f64 {
        let (x1, y1) = (self.x as i64, self.y as i64);
        let (x2, y2) = (other.x as i64, other.y as i64);
        let x = (x2 - x1) as f64;
        let y = (y2 - y1) as f64;

        x.atan2(y).to_degrees()
    }
}

fn main() {
    let mut file = std::fs::File::open("input.txt").expect("unable to open input.txt");
    let asteroids = read_input(&mut file).unwrap();

    let asteroid_count = asteroids
        .iter()
        .map(|ast| {
            let mut visibility: HashMap<Asteroid, Option<bool>> = asteroids
                .iter()
                .filter(|a| *a != ast)
                .map(|a| (*a, None))
                .collect();

            let by_dist = {
                let mut v: Vec<Asteroid> =
                    asteroids.iter().filter(|a| *a != ast).copied().collect();
                v.sort_by_key(|a| ast.distance(a));
                v
            };

            for nearest in by_dist.iter() {
                let process = visibility.get(nearest).unwrap();
                if process.is_none() {
                    // println!("{:?}", nearest);

                    *(visibility.get_mut(nearest).unwrap()) = Some(true);
                    let angle = ast.get_angle(nearest);

                    visibility
                        .iter_mut()
                        .filter(|(_, value)| value.is_none())
                        .filter(|(key, _)| ast.get_angle(key) == angle)
                        .for_each(|(_, value)| *value = Some(false))
                }
            }

            visibility.values().filter(|v| **v == Some(true)).count()
        })
        .max();

    println!("Visible: {:?}", asteroid_count);
    debug_assert_eq!(asteroid_count.unwrap(), 344);
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
                .map(move |(x, _)| Asteroid::new(x, y))
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
