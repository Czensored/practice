use std::fmt::{self, Display};

use crate::TreeNode;

pub fn build_tree_string<T: Display>(node: &TreeNode<T>) -> Vec<String> {
    let s = format!("{}", node.value);
    let width = s.len();

    // leaf node
    if node.left.is_none() && node.right.is_none() {
        return vec![s];
    }

    // process left and right children
    let left_lines = node.left.as_ref().map_or(vec![], |l| build_tree_string(l));
    let right_lines = node.right.as_ref().map_or(vec![], |r| build_tree_string(r));

    let left_width = left_lines.first().map_or(0, |l| l.len());
    let right_width = right_lines.first().map_or(0, |r| r.len());

    let total_width = left_width + 1 + right_width;
    let mut result = Vec::new();

    // Line 1: root value centered
    let root_pad_left = left_width;
    let root_pad_right = total_width - root_pad_left - width;
    let root_line = format!(
        "{}{}{}",
        " ".repeat(root_pad_left),
        s,
        " ".repeat(root_pad_right)
    );
    result.push(root_line);

    // Line 2: branches
    let branch_line = format!(
        "{}{}{}",
        if node.left.is_some() {
            " ".repeat(left_width.saturating_sub(1)) + "/"
        } else {
            " ".repeat(left_width)
        },
        " ",
        if node.right.is_some() {
            "\\".to_string() + &" ".repeat(right_width.saturating_sub(1))
        } else {
            " ".repeat(right_width)
        }
    );
    result.push(branch_line);

    // Merge children lines
    let max_lines = left_lines.len().max(right_lines.len());
    for i in 0..max_lines {
        let left_line = if i < left_lines.len() {
            left_lines[i].clone()
        } else {
            " ".repeat(left_width)
        };
        let right_line = if i < right_lines.len() {
            right_lines[i].clone()
        } else {
            " ".repeat(right_width)
        };
        result.push(format!("{} {}{}", left_line, " ", right_line));
    }

    result
}

impl<T: Display> fmt::Debug for TreeNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = build_tree_string(self);
        for line in lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}
