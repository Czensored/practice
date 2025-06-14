mod pretty_print;

pub struct TreeNode<T> {
    pub value: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn set_left(&mut self, child: TreeNode<T>) {
        self.left = Some(Box::new(child));
    }

    pub fn set_right(&mut self, child: TreeNode<T>) {
        self.right = Some(Box::new(child));
    }

    pub fn invert_tree(&mut self) {
        if let Some(ref mut left_node) = self.left {
            left_node.invert_tree();
        }

        if let Some(ref mut right_node) = self.right {
            right_node.invert_tree();
        }

        std::mem::swap(&mut self.left, &mut self.right);
    }
}

#[allow(unused_mut)]
fn main() {
    let mut root = TreeNode::new(1);
    let mut left = TreeNode::new(2);
    let mut right = TreeNode::new(3);

    left.set_left(TreeNode::new(4));
    left.set_right(TreeNode::new(5));

    root.set_left(left);
    root.set_right(right);

    println!("Original tree:\n{:?}", root);

    root.invert_tree();

    println!("Reversed tree:\n{:?}", root);
}
