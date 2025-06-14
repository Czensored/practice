// Ref: https://www.reddit.com/r/dailyprogrammer/comments/ajunxe/20190125_challenge_373_hard_embeddable_trees/

pub struct TreeNode {
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new() -> Self {
        TreeNode {
            left: None,
            right: None,
        }
    }

    pub fn set_left(&mut self, child: TreeNode) {
        self.left = Some(Box::new(child));
    }

    pub fn set_right(&mut self, child: TreeNode) {
        self.right = Some(Box::new(child));
    }
}

fn tree_from_str(s: &str) -> TreeNode {
}

fn main() {
    let tree_str = "((())(()))";
    let root = tree_from_str(tree_str);
}
