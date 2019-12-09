use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let count = (264_360..=746_325u32)
        .into_par_iter()
        .filter(|&n| check_digits(n))
        .count();

    assert_eq!(count, 945);

    println!("Count of possible solutions: {}", count);

    let count = (264_360..=746_325u32)
        .into_par_iter()
        .filter(|&n| check2(&digits(n)))
        .count();

    assert_eq!(count, 617);

    println!("Count of possible solutions: {}", count);
}

fn check_digits(num: u32) -> bool {
    check(&digits(num))
}

fn check(num: &[u8]) -> bool {
    num.windows(2).all(|v| v[0] <= v[1]) && num.windows(2).any(|v| v[0] == v[1])
}

fn check2(num: &[u8]) -> bool {
    check(num) && count(num).iter().any(|&(_, count)| count == 2)
}

fn digits(num: u32) -> Vec<u8> {
    let mut num = num;
    let mut v = Vec::new();

    while num > 0 {
        let n = num % 10;
        num /= 10;
        v.push(n as u8);
    }

    v.reverse();
    v
}

fn count(num: &[u8]) -> Vec<(u8, usize)> {
    let mut v = Vec::new();
    let (last, count) = num
        .iter()
        .fold((None, 0), |(last, count): (Option<u8>, usize), &n| {
            if let Some(l) = last {
                if l == n {
                    (last, count + 1)
                } else {
                    v.push((l, count));
                    (Some(n), 1)
                }
            } else {
                (Some(n), 1)
            }
        });

    v.push((last.unwrap(), count));

    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        for &case in [111_123, 122_345, 111_111].iter() {
            assert!(check_digits(case), "is_true {}", case);
        }

        for &case in [135_679, 223_450, 123_789].iter() {
            assert!(!check_digits(case), "is_false {}", case);
        }
    }

    #[test]
    fn test_digits() {
        for case in [(111_123, vec![1, 1, 1, 1, 2, 3])].iter() {
            assert_eq!(digits(case.0), case.1);
        }
    }

    #[test]
    fn test_count() {
        assert_eq!(
            count(&[1, 1, 1, 2, 2, 3, 1]),
            vec![(1, 3), (2, 2), (3, 1), (1, 1)]
        );
    }

    #[test]
    fn sample2() {
        assert!(check2(&[1, 1, 2, 2, 3, 3]));
        assert!(!check2(&[1, 2, 3, 4, 4, 4]));
        assert!(check2(&[1, 1, 1, 1, 2, 2]));
    }
}
