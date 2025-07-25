advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let directions = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    let target = ['X', 'M', 'A', 'S'];
    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            for (dx, dy) in directions {
                let mut matched = true;

                for (i, &ch) in target.iter().enumerate() {
                    let new_row = row as isize + dy * i as isize;
                    let new_col = col as isize + dx * i as isize;

                    if new_row < 0 || new_col < 0 {
                        matched = false;
                        break;
                    }

                    let (nr, nc) = (new_row as usize, new_col as usize);

                    if let Some(row) = grid.get(nr) {
                        if let Some(c) = row.get(nc) {
                            if *c != ch {
                                matched = false;
                                break;
                            }
                        } else {
                            matched = false;
                            break;
                        }
                    } else {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 1..rows.saturating_sub(1) {
        for col in 1..cols.saturating_sub(1) {
            if grid[row][col] != 'A' {
                continue;
            }

            let tl = grid[row - 1][col - 1];
            let tr = grid[row - 1][col + 1];
            let bl = grid[row + 1][col - 1];
            let br = grid[row + 1][col + 1];

            let diagonal1 = (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M');
            let diagonal2 = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');

            if diagonal1 && diagonal2 {
                sum += 1;
            }
        }
    }

    Some(sum)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn _part_two_old(input: &str) -> Option<u64> {
    let mut sum = 0;
    let input = parse_input(input);

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row][col] != 'A' || row == 0 || col == 0 {
                continue;
            }

            if ((input.get(row - 1).and_then(|r| r.get(col - 1)) == Some(&'M')
                && input.get(row + 1).and_then(|r| r.get(col + 1)) == Some(&'S'))
                || (input.get(row - 1).and_then(|r| r.get(col - 1)) == Some(&'S')
                    && input.get(row + 1).and_then(|r| r.get(col + 1)) == Some(&'M')))
                && ((input.get(row - 1).and_then(|r| r.get(col + 1)) == Some(&'M')
                    && input.get(row + 1).and_then(|r| r.get(col - 1)) == Some(&'S'))
                    || (input.get(row - 1).and_then(|r| r.get(col + 1)) == Some(&'S')
                        && input.get(row + 1).and_then(|r| r.get(col - 1)) == Some(&'M')))
            {
                sum += 1;
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
