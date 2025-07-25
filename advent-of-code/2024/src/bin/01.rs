use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right) = parse_input(input);
    left.sort_unstable();
    right.sort_unstable();
    let mut sum = 0;

    for (&a, &b) in left.iter().zip(right.iter()) {
        sum += a.abs_diff(b);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sim_score = HashMap::new();
    let (left, right) = parse_input(input);

    for x in &right {
        *sim_score.entry(*x).or_insert(0) += 1;
    }

    let mut sum = 0;
    for x in &left {
        sum += x * sim_score.get(x).copied().unwrap_or(0);
    }

    Some(sum)
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() != 2 {
                return None;
            }
            let a = parts[0].parse::<u64>().ok()?;
            let b = parts[1].parse::<u64>().ok()?;
            Some((a, b))
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
