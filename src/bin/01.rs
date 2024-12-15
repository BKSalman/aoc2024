use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = input.lines().fold((Vec::new(), Vec::new()), |mut acc, l| {
        let mut split = l.split_whitespace();
        let Some(left) = split.next() else {
            return acc;
        };

        let Some(right) = split.next() else {
            return acc;
        };

        acc.0.push(left.parse::<i32>().unwrap());
        acc.1.push(right.parse::<i32>().unwrap());

        acc
    });

    assert!(left.len() == right.len());

    left.sort();
    right.sort();

    let mut total_distance = 0;

    for (left, right) in left.into_iter().zip(right) {
        total_distance += left.abs_diff(right);
    }

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
