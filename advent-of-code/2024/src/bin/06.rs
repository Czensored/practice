use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let (pos, mut dir) = locate_guard(&grid);
    let mut pos = (pos.0 as isize, pos.1 as isize);

    let mut visited = HashSet::new();
    visited.insert(pos);

    loop {
        let (dx, dy) = dir.delta();
        let next_x = pos.0 + dx;
        let next_y = pos.1 + dy;

        if next_x < 0 || next_x >= width || next_y < 0 || next_y >= height {
            break;
        }

        match grid[next_y as usize][next_x as usize] {
            '#' => dir = dir.turn_right(),
            _ => {
                pos = (next_x, next_y);
                visited.insert(pos);
            }
        }
    }

    Some(visited.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    let mut grid = parse_input(input);

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '.' {
                grid[y][x] = '#';
                if is_a_loop(&grid) {
                    sum += 1;
                }
                grid[y][x] = '.';
            }
        }
    }

    Some(sum)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn delta(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

fn locate_guard(grid: &[Vec<char>]) -> ((usize, usize), Direction) {
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            let dir = match ch {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => continue,
            };
            return ((x, y), dir);
        }
    }
    unreachable!("Guard not found")
}

fn is_a_loop(grid: &[Vec<char>]) -> bool {
    let (start_pos, start_dir) = locate_guard(grid);
    let (width, height) = (grid[0].len() as isize, grid.len() as isize);

    let mut pos = (start_pos.0 as isize, start_pos.1 as isize);
    let mut dir = start_dir;

    let mut seen = HashSet::new();
    seen.insert((pos, dir));

    loop {
        let (dx, dy) = dir.delta();
        let next = (pos.0 + dx, pos.1 + dy);

        if next.0 < 0 || next.0 >= width || next.1 < 0 || next.1 >= height {
            return false;
        }

        match grid[next.1 as usize][next.0 as usize] {
            '#' => {
                dir = dir.turn_right();
            }
            _ => {
                pos = next;
                if !seen.insert((pos, dir)) {
                    return true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
