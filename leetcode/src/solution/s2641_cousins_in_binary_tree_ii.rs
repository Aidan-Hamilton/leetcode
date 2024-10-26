/**
 * [2641] Cousins in Binary Tree II
 *
 * Given the root of a binary tree, replace the value of each node in the tree with the sum of all its cousins' values.
 * Two nodes of a binary tree are cousins if they have the same depth with different parents.
 * Return the root of the modified tree.
 * Note that the depth of a node is the number of edges in the path from the root node to it.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/01/11/example11.png" style="width: 571px; height: 151px;" />
 * Input: root = [5,4,9,1,10,null,7]
 * Output: [0,0,0,7,7,null,11]
 * Explanation: The diagram above shows the initial binary tree and the binary tree after changing the value of each node.
 * - Node with value 5 does not have any cousins so its sum is 0.
 * - Node with value 4 does not have any cousins so its sum is 0.
 * - Node with value 9 does not have any cousins so its sum is 0.
 * - Node with value 1 has a cousin with value 7 so its sum is 7.
 * - Node with value 10 has a cousin with value 7 so its sum is 7.
 * - Node with value 7 has cousins with values 1 and 10 so its sum is 11.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/01/11/diagram33.png" style="width: 481px; height: 91px;" />
 * Input: root = [3,1,2]
 * Output: [0,0,0]
 * Explanation: The diagram above shows the initial binary tree and the binary tree after changing the value of each node.
 * - Node with value 3 does not have any cousins so its sum is 0.
 * - Node with value 1 does not have any cousins so its sum is 0.
 * - Node with value 2 does not have any cousins so its sum is 0.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^5].
 * 	1 <= Node.val <= 10^4
 *
 */
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/cousins-in-binary-tree-ii/
// discuss: https://leetcode.com/problems/cousins-in-binary-tree-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs_sums(depth: usize, node: Option<&Rc<RefCell<TreeNode>>>, ds: &mut Vec<i32>) {
            let Some(node) = node.as_ref().map(|n| n.borrow()) else {
                return;
            };
            if let Some(v) = ds.get_mut(depth) {
                *v += node.val;
            } else {
                ds.push(node.val);
            }
            dfs_sums(depth + 1, node.left.as_ref(), ds);
            dfs_sums(depth + 1, node.right.as_ref(), ds);
        }

        fn dfs_update(depth: usize, node: Option<&mut Rc<RefCell<TreeNode>>>, ds: &Vec<i32>) {
            let Some(mut node) = node.map(|n| n.borrow_mut()) else {
                return;
            };
            let left_val = node.left.as_ref().map_or(0, |n| n.borrow().val);
            let right_val = node.right.as_ref().map_or(0, |n| n.borrow().val);
            let up = ds.get(depth + 1).unwrap_or(&0) - left_val - right_val;
            if let Some(mut left) = node.left.as_mut().map(|n| n.borrow_mut()) {
                left.val = up;
            }
            if let Some(mut right) = node.right.as_mut().map(|n| n.borrow_mut()) {
                right.val = up;
            }
            dfs_update(depth + 1, node.left.as_mut(), ds);
            dfs_update(depth + 1, node.right.as_mut(), ds);
        }
        let mut root = root;
        if let Some(mut node) = root.as_mut().map(|n| n.borrow_mut()) {
            node.val = 0;
        } else {
            return None;
        }
        let mut level_sums = Vec::new();
        dfs_sums(0, root.as_ref(), &mut level_sums);
        dfs_update(0, root.as_mut(), &level_sums);
        root
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2641() {
        time_test!();
        assert_eq!(
            Solution::replace_value_in_tree(tree![5, 4, 9, 1, 10, null, 7]),
            tree![0, 0, 0, 7, 7, null, 11]
        );
        assert_eq!(
            Solution::replace_value_in_tree(tree![3, 1, 2]),
            tree![0, 0, 0]
        );
    }
}
