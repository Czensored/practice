advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut sum = 0;

    for report in input {
        if is_valid_sequence(&report) {
            sum += 1;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut sum = 0;

    'reports: for report in input {
        if is_valid_sequence(&report) {
            sum += 1;
            continue;
        }

        for i in 0..report.len() {
            let mut mini_report = report.to_vec();
            mini_report.remove(i);

            if is_valid_sequence(&mini_report) {
                sum += 1;
                continue 'reports;
            }
        }
    }

    Some(sum)
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<u64>().ok())
                .collect()
        })
        .collect()
}

fn is_valid_sequence(report: &[u64]) -> bool {
    if report.len() < 2 {
        return false;
    }

    let dir = direction(report[0], report[1]);
    if dir == 0 {
        return false;
    }

    report.windows(2).all(|w| {
        let &[a, b] = w else { return false };
        let diff = b as i64 - a as i64;
        diff.signum() == dir && diff.abs() <= 3
    })
}

fn direction(a: u64, b: u64) -> i64 {
    (b as i64 - a as i64).signum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
