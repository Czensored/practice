use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (rules_map, updates_list) = parse_input(input);
    let mut sum = 0;

    for update in updates_list {
        if is_valid_update(&rules_map, &update) {
            sum += update[update.len() / 2];
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules_map, updates_list) = parse_input(input);
    let mut sum = 0;

    let mut incorrect_updates: Vec<Vec<u64>> = updates_list
        .iter()
        .filter(|x| !is_valid_update(&rules_map, x))
        .cloned()
        .collect();

    for update in incorrect_updates.iter_mut() {
        fix_update(&rules_map, update);
        sum += update[update.len() / 2];
    }

    Some(sum)
}

fn parse_input(input: &str) -> (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) {
    let mut lines = input.lines();

    let mut map = HashMap::new();
    let mut lists = Vec::new();

    for line in lines.by_ref() {
        let line = line.trim();
        if line.is_empty() {
            break;
        }

        if let Some((k, v)) = line.split_once('|') {
            let key = k.trim().parse::<u64>().unwrap();
            let value = v.trim().parse::<u64>().unwrap();
            map.entry(key).or_insert_with(Vec::new).push(value);
        }
    }

    for line in lines {
        let numbers = line
            .split(',')
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        if !numbers.is_empty() {
            lists.push(numbers);
        }
    }

    (map, lists)
}

fn is_valid_update(rules_map: &HashMap<u64, Vec<u64>>, update: &Vec<u64>) -> bool {
    let mut pages = HashSet::new();
    for num in update {
        if let Some(before_rules) = rules_map.get(&num) {
            for i in before_rules {
                if pages.contains(i) {
                    return false;
                }
            }
        }
        pages.insert(*num);
    }
    return true;
}

fn fix_update(rules_map: &HashMap<u64, Vec<u64>>, update: &mut Vec<u64>) {
    use std::collections::{HashMap, HashSet, VecDeque};

    let update_set: HashSet<u64> = update.iter().copied().collect();

    // Build subgraph
    let mut graph: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut indegree: HashMap<u64, u64> = HashMap::new();

    for &page in update.iter() {
        indegree.entry(page).or_insert(0);
    }

    for &from in update.iter() {
        if let Some(dependents) = rules_map.get(&from) {
            for &to in dependents {
                if update_set.contains(&to) {
                    graph.entry(from).or_default().push(to);
                    *indegree.entry(to).or_insert(0) += 1;
                }
            }
        }
    }

    // Kahn's algorithm
    let mut queue: VecDeque<u64> = indegree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&k, _)| k)
        .collect();

    let mut sorted = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                let entry = indegree.get_mut(&neighbor).unwrap();
                *entry -= 1;
                if *entry == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    // Replace update with sorted version
    *update = sorted;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
