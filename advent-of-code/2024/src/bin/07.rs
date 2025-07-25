advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut sum = 0;

    for (target, nums) in input {
        if check_part_one(target, &nums) {
            sum += target;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut sum = 0;

    for (target, nums) in input {
        if check_part_two(target, &nums) {
            sum += target;
        }
    }

    Some(sum)
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .filter_map(|line| {
            let (key, rest) = line.split_once(':')?;
            let id = key.trim().parse::<u64>().ok()?;
            let values = rest
                .split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect::<Vec<u64>>();
            Some((id, values))
        })
        .collect()
}

fn check_part_one(target: u64, nums: &[u64]) -> bool {
    fn dfs(index: usize, current: u64, nums: &[u64], target: u64) -> bool {
        if index == nums.len() {
            return current == target;
        }

        let next = nums[index];

        dfs(index + 1, current + next, nums, target) || dfs(index + 1, current * next, nums, target)
    }

    if nums.is_empty() {
        return false;
    }

    dfs(1, nums[0], nums, target)
}

fn concat(a: u64, b: u64) -> u64 {
    let mut multiplier = 1;
    let mut b_copy = b;
    while b_copy > 0 {
        multiplier *= 10;
        b_copy /= 10;
    }
    a * multiplier + b
}

fn check_part_two(target: u64, nums: &[u64]) -> bool {
    fn dfs(index: usize, current: u64, nums: &[u64], target: u64) -> bool {
        if index == nums.len() {
            return current == target;
        }

        let next = nums[index];

        dfs(index + 1, current + next, nums, target)
            || dfs(index + 1, current * next, nums, target)
            || dfs(index + 1, concat(current, next), nums, target)
    }

    if nums.is_empty() {
        return false;
    }

    dfs(1, nums[0], nums, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
