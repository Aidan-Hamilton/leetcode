use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    let nodes = vec
        .iter()
        .map(|&num| {
            if num == None {
                None
            } else {
                Some(Rc::new(RefCell::new(TreeNode::new(num.unwrap()))))
            }
        })
        .collect::<Vec<_>>();
    let head = nodes.get(0)?.clone();

    let mut child_node_ptr = 1;
    for node in nodes.iter().flatten() {
        if let Some(child_node) = nodes.get(child_node_ptr) {
            node.borrow_mut().left = child_node.clone();
            child_node_ptr += 1;
        } else {
            break;
        }
        if let Some(child_node) = nodes.get(child_node_ptr) {
            node.borrow_mut().right = child_node.clone();
            child_node_ptr += 1;
        } else {
            break;
        }
    }

    head
}

#[macro_export]
macro_rules! tree {
    () => {
        None
    };
   ($($e:expr), *) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}
