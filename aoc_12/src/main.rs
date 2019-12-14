use itertools::{izip, Itertools};
use num::Integer;
use std::{cmp::Ordering, fs::File, io::Read, num::ParseIntError, slice::from_raw_parts_mut};

const DIMENSIONS: usize = 3;

trait PhysicsSimulation {
    type Body;

    fn simulate(&mut self) {
        let len = self.data().len() as isize;

        for indices in (0..len).combinations(2) {
            assert_eq!(indices.len(), 2);
            assert_ne!(indices[0], indices[1]);
            let (one, two) = unsafe {
                let ptr = self.data().as_mut_ptr();
                let one = &mut from_raw_parts_mut(ptr.offset(indices[0]), 1)[0];
                let two = &mut from_raw_parts_mut(ptr.offset(indices[1]), 1)[0];
                (one, two)
            };
            Self::update_position(one, two);
        }

        for body in self.data().iter_mut() {
            Self::update_velocity(body);
        }
    }

    fn data(&mut self) -> &mut [Self::Body];
    fn update_position(a: &mut Self::Body, b: &mut Self::Body);
    fn update_velocity(a: &mut Self::Body);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SpaceBody {
    location: [i32; DIMENSIONS],
    velocity: [i32; DIMENSIONS],
}

impl SpaceBody {
    fn new(x: i32, y: i32, z: i32) -> Self {
        SpaceBody {
            location: [x, y, z],
            velocity: [0, 0, 0],
        }
    }

    fn update(&mut self, other: &mut Self) {
        for (pos_a, pos_b, vol_a, vol_b) in izip!(
            self.location.iter(),
            other.location.iter(),
            self.velocity.iter_mut(),
            other.velocity.iter_mut(),
        ) {
            match pos_a.cmp(&pos_b) {
                Ordering::Equal => (),
                Ordering::Less => {
                    *vol_a += 1;
                    *vol_b -= 1;
                }
                Ordering::Greater => {
                    *vol_a -= 1;
                    *vol_b += 1;
                }
            }
        }
    }

    fn apply_velocity(&mut self) {
        for (loc, vol) in self.location.iter_mut().zip(self.velocity.iter()) {
            *loc += vol;
        }
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.iter().map(|vel| vel.abs()).sum()
    }

    fn potential_energy(&self) -> i32 {
        self.location.iter().map(|loc| loc.abs()).sum()
    }

    fn energy(&self) -> i32 {
        self.kinetic_energy() * self.potential_energy()
    }
}

struct SpaceSystem {
    bodies: Vec<SpaceBody>,
}

impl PhysicsSimulation for SpaceSystem {
    type Body = SpaceBody;

    fn data(&mut self) -> &mut [Self::Body] {
        &mut self.bodies
    }

    fn update_position(a: &mut Self::Body, b: &mut Self::Body) {
        a.update(b)
    }

    fn update_velocity(a: &mut Self::Body) {
        a.apply_velocity();
    }
}

impl SpaceSystem {
    fn new(coordinates: &[(i32, i32, i32)]) -> Self {
        let bodies = coordinates
            .iter()
            .map(|(x, y, z)| SpaceBody::new(*x, *y, *z))
            .collect();

        Self { bodies }
    }

    fn step(&mut self) {
        self.simulate();
    }

    fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    fn total_energy(&self) -> i32 {
        self.bodies.iter().map(|m| m.energy()).sum()
    }

    fn to_axes(&self) -> Vec<SingleAxis> {
        (0..DIMENSIONS)
            .map(|dimension| {
                let v = self
                    .bodies
                    .iter()
                    .map(|sb| (sb.location[dimension], sb.velocity[dimension]))
                    .collect::<Vec<(i32, i32)>>();

                SingleAxis::new(&v)
            })
            .collect()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct SingleAxis {
    axis: Vec<(i32, i32)>,
}

impl PhysicsSimulation for SingleAxis {
    type Body = (i32, i32);

    fn data(&mut self) -> &mut [Self::Body] {
        &mut self.axis
    }

    fn update_position(one: &mut Self::Body, two: &mut Self::Body) {
        match one.0.cmp(&two.0) {
            Ordering::Equal => (),
            Ordering::Less => {
                one.1 += 1;
                two.1 -= 1;
            }
            Ordering::Greater => {
                one.1 -= 1;
                two.1 += 1;
            }
        }
    }

    fn update_velocity(a: &mut Self::Body) {
        a.0 += a.1;
    }
}

impl SingleAxis {
    fn new(axis: &[(i32, i32)]) -> Self {
        Self {
            axis: axis.to_vec(),
        }
    }

    fn count_cycle(&self) -> usize {
        let mut copy = self.clone();
        let mut count = 0;
        loop {
            count += 1;
            copy.simulate();

            if copy.axis == self.axis {
                break;
            }
        }

        count
    }
}

fn read_input<R: Read>(input: &mut R) -> Result<Vec<(i32, i32, i32)>, String> {
    let mut buffer = String::new();
    if let Err(msg) = input.read_to_string(&mut buffer) {
        return Err(msg.to_string());
    }

    buffer
        .lines()
        .map(|line| {
            let filtered = line
                .chars()
                .filter(|x| x.is_ascii_digit() || *x == ',' || *x == '-')
                .collect::<String>();

            let values = filtered
                .split(',')
                .map(|s| s.trim())
                .map(|s| {
                    s.parse::<_>()
                        .map_err(|e: ParseIntError| format!("{}: {}", e, s))
                })
                .collect::<Result<Vec<_>, _>>()?;

            assert_eq!(values.len(), 3);
            Ok((values[0], values[1], values[2]))
        })
        .collect::<Result<_, _>>()
}

fn main() {
    let mut file = File::open("input.txt").expect("unable to open input.txt");
    let v = read_input(&mut file).unwrap();
    let mut ss = SpaceSystem::new(&v);
    ss.step_n(1000);
    let energy = ss.total_energy();
    println!("total_energy: {}", energy);
    debug_assert_eq!(energy, 10944);

    let axes = SpaceSystem::new(&v).to_axes();
    let cycles = axes
        .iter()
        .map(|axis| axis.count_cycle())
        .fold1(|acc, x| acc.lcm(&x))
        .unwrap();

    println!("Repeats in {} cycles", cycles);
    debug_assert_eq!(cycles, 484_244_804_958_744);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let mut space_objects =
            SpaceSystem::new(&[(-1, 0, 2), (2, -10, -7), (4, -8, 8), (3, 5, -1)]);

        space_objects.step();

        assert_eq!(
            space_objects.bodies,
            &[
                SpaceBody {
                    location: [2, -1, 1],
                    velocity: [3, -1, -1]
                },
                SpaceBody {
                    location: [3, -7, -4],
                    velocity: [1, 3, 3]
                },
                SpaceBody {
                    location: [1, -7, 5],
                    velocity: [-3, 1, -3]
                },
                SpaceBody {
                    location: [2, 2, 0],
                    velocity: [-1, -3, 1]
                },
            ]
        );

        space_objects.step();
        assert_eq!(
            space_objects.bodies,
            &[
                SpaceBody {
                    location: [5, -3, -1],
                    velocity: [3, -2, -2]
                },
                SpaceBody {
                    location: [1, -2, 2],
                    velocity: [-2, 5, 6]
                },
                SpaceBody {
                    location: [1, -4, -1],
                    velocity: [0, 3, -6]
                },
                SpaceBody {
                    location: [1, -4, 2],
                    velocity: [-1, -6, 2]
                },
            ]
        );

        space_objects.step_n(2770);
        assert_eq!(
            space_objects.bodies,
            &[
                SpaceBody {
                    location: [-1, 0, 2],
                    velocity: [0, 0, 0]
                },
                SpaceBody {
                    location: [2, -10, -7],
                    velocity: [0, 0, 0]
                },
                SpaceBody {
                    location: [4, -8, 8],
                    velocity: [0, 0, 0]
                },
                SpaceBody {
                    location: [3, 5, -1],
                    velocity: [0, 0, 0]
                },
            ]
        );
    }

    #[test]
    fn sample2() {
        let mut space_objects =
            SpaceSystem::new(&[(-8, -10, 0), (5, 5, 10), (2, -7, 3), (9, -8, -3)]);

        space_objects.step_n(100);
        assert_eq!(space_objects.total_energy(), 1940);
    }
}
