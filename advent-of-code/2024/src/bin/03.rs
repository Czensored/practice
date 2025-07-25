use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").ok()?;
    let sum = re.captures_iter(input).map(|caps| {
        let x: u64 = caps[1].parse().unwrap();
        let y: u64 = caps[2].parse().unwrap();
        x * y
    }).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"(?P<mul>mul\((\d{1,3}),(\d{1,3})\))|(?P<do>do\(\))|(?P<dont>don't\(\))").ok()?;

    let mut enabled = true;
    let mut sum = 0;

    for cap in re.captures_iter(input) {
        if cap.name("do").is_some() {
            enabled = true;
        } else if cap.name("dont").is_some() {
            enabled = false;
        } else if let Some(_) = cap.name("mul") {
            if enabled {
                let x: u64 = cap.get(2)?.as_str().parse().ok()?;
                let y: u64 = cap.get(3)?.as_str().parse().ok()?;
                sum += x * y;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
