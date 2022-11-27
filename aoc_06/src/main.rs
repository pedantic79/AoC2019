use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::iter::successors;

fn main() {
    let mut file = File::open(
        std::path::PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()),
        )
        .join("input.txt"),
    )
    .expect("unable to open input.txt");
    let hm = read_input(&mut file).unwrap();

    let dist = total_distance(&hm);
    println!("{}", dist);
    assert_eq!(dist, 271_151);

    let between = point_to_point("YOU", "SAN", &hm);
    println!("BETWEEN: {}", between);
    assert_eq!(between, 388);
}

fn read_input<R: Read>(input: &mut R) -> Result<HashMap<String, String>, String> {
    let mut buffer = String::new();
    if let Err(msg) = input.read_to_string(&mut buffer) {
        return Err(msg.to_string());
    }

    buffer
        .lines()
        .map(|line| {
            let mut v = line.split(')').map(|x| x.into()).collect::<Vec<_>>();
            if v.len() == 2 {
                let v1 = v.remove(1); // Moves the value out
                let v0 = v.remove(0);

                // Child(Parent
                Ok((v1, v0))
            } else {
                Err("parse error".into())
            }
        })
        .collect()
}

fn path<'a>(
    node: &'a str,
    solar_system: &'a HashMap<String, String>,
) -> impl Iterator<Item = String> + 'a {
    successors(Some(node.to_string()), move |current| {
        let s: &String = current;
        solar_system.get(s).cloned()
    })
    .skip(1)
}

fn distance(node: &str, solar_system: &HashMap<String, String>) -> usize {
    path(node, solar_system).count()
}

fn total_distance(solar_system: &HashMap<String, String>) -> usize {
    solar_system
        .keys()
        .map(|object| distance(object, solar_system))
        .sum()
}

fn point_to_point(source: &str, dest: &str, solar_system: &HashMap<String, String>) -> usize {
    let mut source_path: Vec<_> = path(source, solar_system).collect();
    let mut dest_path: Vec<_> = path(dest, solar_system).collect();

    source_path.reverse();
    dest_path.reverse();

    let common = source_path
        .iter()
        .zip(dest_path.iter())
        .filter(|(x, y)| x == y)
        .count();

    source_path.len() + dest_path.len() - 2 * common
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let mut input: &[u8] = b"COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n";
        let v = read_input(&mut input).unwrap();
        assert_eq!(v.len(), 11);
        assert_eq!(v.get("B"), Some(&"COM".into()));
        assert_eq!(v.get("H"), Some(&"G".into()));
        assert_eq!(distance("D", &v), 3);
        assert_eq!(total_distance(&v), 42);
    }
}
