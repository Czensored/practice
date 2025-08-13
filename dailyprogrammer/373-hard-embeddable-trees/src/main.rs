// Ref: https://www.reddit.com/r/dailyprogrammer/comments/ajunxe/20190125_challenge_373_hard_embeddable_trees/

use itertools::Itertools;
use reqwest::blocking::get;

pub struct TreeNode {
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new() -> Self {
        TreeNode {
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    pub fn print(&self) {
        let (lines, _, _) = Self::render(self);
        for line in lines {
            println!("{}", line.trim_end());
        }
    }

    /// Returns (lines, root_pos, width)
    fn render(node: &TreeNode) -> (Vec<String>, usize, usize) {
        let label = "o";
        let label_width = label.len();

        if node.children.is_empty() {
            return (vec![label.to_string()], label_width / 2, label_width);
        }

        let spacing = 2;

        // Render all children
        let mut child_renders = vec![];
        let mut total_width = 0;
        for child in &node.children {
            let (lines, root_pos, width) = Self::render(child);
            child_renders.push((lines, root_pos, width));
            total_width += width + spacing;
        }
        total_width -= spacing; // remove extra spacing after last child

        // Determine positions of each child root
        let mut child_root_positions = vec![];
        let mut offset = 0;
        for (_, root_pos, width) in &child_renders {
            child_root_positions.push(offset + *root_pos);
            offset += *width + spacing;
        }

        // Determine this node's root position (centered over children)
        let min = *child_root_positions.first().unwrap();
        let max = *child_root_positions.last().unwrap();
        let root_pos = (min + max) / 2;

        // Build first line with the label
        let mut first_line = " ".repeat(total_width);
        first_line.replace_range(root_pos..root_pos + label_width, label);

        // Build second line with connectors
        let mut second_line = " ".repeat(total_width);
        for &child_pos in &child_root_positions {
            if child_pos == root_pos {
                second_line.replace_range(child_pos..child_pos + 1, "|");
            } else if child_pos < root_pos {
                second_line.replace_range(child_pos..child_pos + 1, "/");
            } else {
                second_line.replace_range(child_pos..child_pos + 1, "\\");
            }
        }

        // Merge all child lines
        let max_child_height = child_renders
            .iter()
            .map(|(lines, _, _)| lines.len())
            .max()
            .unwrap_or(0);

        let mut merged_lines = vec![];
        for i in 0..max_child_height {
            let mut line = String::new();
            for (lines, _, width) in &child_renders {
                if i < lines.len() {
                    line += &format!("{:<width$}", lines[i], width = *width);
                } else {
                    line += &" ".repeat(*width);
                }
                line += &" ".repeat(spacing);
            }
            line.truncate(total_width);
            merged_lines.push(line);
        }

        let mut result = vec![first_line, second_line];
        result.extend(merged_lines);

        (result, root_pos, total_width)
    }
}

fn parse_tree(s: &str) -> TreeNode {
    fn parse(chars: &[u8], pos: &mut usize) -> Option<TreeNode> {
        if *pos >= chars.len() || chars[*pos] != b'(' {
            return None;
        }
        *pos += 1; // skip '('

        let mut node = TreeNode::new();

        while *pos < chars.len() && chars[*pos] != b')' {
            if let Some(child) = parse(chars, pos) {
                node.add_child(child);
            }
        }

        if *pos < chars.len() && chars[*pos] == b')' {
            *pos += 1; // skip ')'
        }

        Some(node)
    }

    let chars = s.as_bytes();
    let mut pos = 0;
    parse(chars, &mut pos).unwrap_or_else(TreeNode::new)
}

fn equal(s1: &str, s2: &str) -> bool {
    let t1 = parse_tree(s1);
    let t2 = parse_tree(s2);
    canonicalize(&t1) == canonicalize(&t2)
}

fn canonicalize(node: &TreeNode) -> String {
    let mut child_reprs: Vec<String> = node
        .children
        .iter()
        .map(|child| canonicalize(child))
        .collect();
    child_reprs.sort();
    format!("({})", child_reprs.concat())
}

fn embeddable(pattern: &str, target: &str) -> bool {
    let t1 = parse_tree(pattern);
    let t2 = parse_tree(target);
    is_embeddable(&t1, &t2)
}

fn is_embeddable(pattern: &TreeNode, target: &TreeNode) -> bool {
    // Base case: if t1 has more children than t2, impossible
    if pattern.children.len() > target.children.len() {
        return false;
    }

    // Try to match each child of t1 to a distinct child or descendant in t2
    fn match_children(pattern_children: &[TreeNode], target_children: &[TreeNode]) -> bool {
        if pattern_children.is_empty() {
            return true;
        }

        // Try all possible injective matchings
        for indices in target_children
            .iter()
            .enumerate()
            .combinations(pattern_children.len())
        {
            if indices
                .iter()
                .zip(pattern_children)
                .all(|((_, tc), pc)| is_embeddable(pc, tc))
            {
                return true;
            }
        }
        false
    }

    match_children(&pattern.children, &target.children)
        || target
            .children
            .iter()
            .any(|child| is_embeddable(pattern, child))
}

fn load_tree_pairs(url: &str) -> Vec<(String, String)> {
    let response = get(url).unwrap();
    let content = response.text().unwrap();

    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let s1 = parts.next().unwrap().to_string();
            let s2 = parts.next().unwrap().to_string();
            (s1, s2)
        })
        .collect()
}

fn main() {
    // let tree_str = "(()()()()())";
    // let tree_str = "((((()()))(()))((((()()))))((())(())(())))";
    // let root = parse_tree(tree_str);
    // root.print();

    let equal_trees = load_tree_pairs(
        "https://gist.githubusercontent.com/cosmologicon/be38523b48f7da5ab9c886fca94a57b4/raw/37abb03b7365ff17056f7f59beb77d999fd6c81b/tree-equal.txt",
    );

    let sum = equal_trees.iter().filter(|(a, b)| equal(a, b)).count();
    println!("Number of equal trees: {}", sum);

    let embeddable_trees = load_tree_pairs(
        "https://gist.githubusercontent.com/cosmologicon/dcf49d29c563dfc36a3d1c5053124be4/raw/9c663c5a9071571f041d11b08bf3c8958a22b3dd/tree-embed.txt",
    );

    let sum = embeddable_trees
        .iter()
        .filter(|(a, b)| embeddable(a, b))
        .count();
    println!("Number of embeddable trees: {}", sum);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_equal() {
        assert_eq!(equal("((()((())()))(()))", "((())(()(()(()))))"), true);
        assert_eq!(equal("((()))", "(()())"), false);
        assert_eq!(equal("(((()())())()())", "(((()())()())())"), false);
    }

    #[test]
    fn test_embeddable() {
        assert_eq!(embeddable("(())", "(()())"), true);
        assert_eq!(embeddable("(()()())", "((()())())"), false);
    }
}
