use std::fs::File;
use std::iter::successors;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DirectionalVector(usize, Direction);

impl FromStr for DirectionalVector {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        let num = s[1..].parse().map_err(|e| format!("{}: {}", e, s))?;

        Ok(DirectionalVector(
            num,
            match &s[0..1] {
                "U" => Up,
                "D" => Down,
                "L" => Left,
                "R" => Right,
                letter => unreachable!("unknown direction: {}", letter),
            },
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Grid(i32, i32);

impl Grid {
    fn wire(self, directions: &[DirectionalVector]) -> Vec<Grid> {
        directions
            .iter()
            .fold((self, Vec::new()), |(start, mut v), d| {
                v.extend(start.wire_segment(d));
                (*v.last().unwrap(), v)
            })
            .1
    }

    fn wire_segment(self, direction: &DirectionalVector) -> impl Iterator<Item = Grid> + '_ {
        successors(Some(self), move |grid| Some(grid.next_grid(direction)))
            .skip(1) // ignore initial position
            .take(direction.0)
    }

    fn next_grid(self, direction: &DirectionalVector) -> Grid {
        use Direction::*;

        match direction.1 {
            Up => Grid(self.0, self.1 + 1),
            Down => Grid(self.0, self.1 - 1),
            Left => Grid(self.0 - 1, self.1),
            Right => Grid(self.0 + 1, self.1),
        }
    }

    fn distance(self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

fn process<R: std::io::Read>(input: &mut R) -> Option<(Grid, usize)> {
    let v = read_input(input).expect("parse error");
    assert_eq!(v.len(), 2);

    let wire1 = Grid(0, 0).wire(&v[0]);
    let wire2 = Grid(0, 0).wire(&v[1]);

    let intersections = intersect(&wire1, &wire2).collect::<Vec<_>>();

    let closest = intersections
        .iter()
        .map(|intersection| {
            let a = wire1.iter().position(|g| g == intersection).unwrap();
            let b = wire2.iter().position(|g| g == intersection).unwrap();
            a + b + 2
        })
        .min()?;

    Some((
        intersections.into_iter().min_by_key(|g| g.distance())?,
        closest,
    ))
}

fn intersect<'a>(one: &'a [Grid], two: &'a [Grid]) -> impl Iterator<Item = Grid> + 'a {
    one.iter()
        .flat_map(move |grid| two.iter().filter(move |&x| x == grid))
        .copied()
}

fn read_input<R: std::io::Read>(input: &mut R) -> Result<Vec<Vec<DirectionalVector>>, String> {
    let mut buffer = String::new();
    if let Err(msg) = input.read_to_string(&mut buffer) {
        return Err(msg.to_string());
    }

    buffer
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.trim())
                .map(|s| s.parse())
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()
}

fn main() {
    let mut file = File::open("input.txt").expect("unable to open input.txt");
    let (intersection, steps) = process(&mut file).unwrap();

    assert_eq!(intersection, Grid(24, 1650));
    assert_eq!(steps, 14012);

    println!(
        "Closest point is {:?} with distance {} in {} steps",
        intersection,
        intersection.distance(),
        steps
    )
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_helper<R: std::io::Read>(input: &mut R, answer: i32, distance: usize) {
        assert_eq!(
            process(input).map(|(x, y)| (x.distance(), y)),
            Some((answer, distance))
        )
    }

    #[test]
    fn sample1() {
        let mut input: &[u8] = b"R8,U5,L5,D3\nU7,R6,D4,L4";
        sample_helper(&mut input, 6, 30)
    }

    #[test]
    fn sample2() {
        let mut input: &[u8] =
            b"R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        sample_helper(&mut input, 159, 610)
    }

    #[test]
    fn sample3() {
        let mut input: &[u8] =
            b"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        sample_helper(&mut input, 135, 410)
    }

    #[test]
    fn parsing_file() {
        use Direction::*;

        let mut input: &[u8] =
            b"R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n";
        assert_eq!(
            read_input(&mut input).unwrap(),
            vec![
                [
                    (75, Right),
                    (30, Down),
                    (83, Right),
                    (83, Up),
                    (12, Left),
                    (49, Down),
                    (71, Right),
                    (7, Up),
                    (72, Left)
                ]
                .iter()
                .map(|(mag, d)| DirectionalVector(*mag, *d))
                .collect::<Vec<_>>(),
                [
                    (62, Up),
                    (66, Right),
                    (55, Up),
                    (34, Right),
                    (71, Down),
                    (55, Right),
                    (58, Down),
                    (83, Right)
                ]
                .iter()
                .map(|(mag, d)| DirectionalVector(*mag, *d))
                .collect::<Vec<_>>()
            ]
        );
    }

    #[test]
    fn simple_wire() {
        use Direction::*;

        let start = Grid(0, 0);

        let d = DirectionalVector(4, Right);
        let v = start.wire_segment(&d).collect::<Vec<_>>();

        assert_eq!(v, vec![Grid(1, 0), Grid(2, 0), Grid(3, 0), Grid(4, 0)])
    }

    #[test]
    fn multiple_segments() {
        use Direction::*;

        let start = Grid(0, 0);

        let d = [
            DirectionalVector(2, Right),
            DirectionalVector(1, Up),
            DirectionalVector(3, Left),
            DirectionalVector(2, Down),
        ];
        let v = start.wire(&d);

        assert_eq!(
            v,
            vec![
                Grid(1, 0),
                Grid(2, 0),
                Grid(2, 1),
                Grid(1, 1),
                Grid(0, 1),
                Grid(-1, 1),
                Grid(-1, 0),
                Grid(-1, -1)
            ]
        )
    }

    #[test]
    fn intersection() {
        assert_eq!(
            intersect(
                &[Grid(0, 0), Grid(1, 0), Grid(2, 0)],
                &[Grid(0, 0), Grid(1, 0), Grid(4, 0)]
            )
            .collect::<Vec<_>>(),
            vec![Grid(0, 0), Grid(1, 0)]
        )
    }
}
