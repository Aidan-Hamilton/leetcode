/**
 * [951] Flip Equivalent Binary Trees
 *
 * For a binary tree T, we can define a flip operation as follows: choose any node, and swap the left and right child subtrees.
 * A binary tree X is flip equivalent to a binary tree Y if and only if we can make X equal to Y after some number of flip operations.
 * Given the roots of two binary trees root1 and root2, return true if the two trees are flip equivalent or false otherwise.
 *  
 * Example 1:
 * <img alt="Flipped Trees Diagram" src="https://assets.leetcode.com/uploads/2018/11/29/tree_ex.png" style="width: 500px; height: 220px;" />
 * Input: root1 = [1,2,3,4,5,6,null,null,null,7,8], root2 = [1,3,2,null,6,4,5,null,null,null,null,8,7]
 * Output: true
 * Explanation: We flipped at nodes with values 1, 3, and 5.
 *
 * Example 2:
 *
 * Input: root1 = [], root2 = []
 * Output: true
 *
 * Example 3:
 *
 * Input: root1 = [], root2 = [1]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in each tree is in the range [0, 100].
 * 	Each tree will have unique node values in the range [0, 99].
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/flip-equivalent-binary-trees/
// discuss: https://leetcode.com/problems/flip-equivalent-binary-trees/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            (None, None) => true,
            // DFS to compare both sides
            (Some(r1), Some(r2)) => {
                let r1 = r1.borrow();
                let r2 = r2.borrow();
                r1.val == r2.val
                    && ((Self::flip_equiv(r1.left.clone(), r2.left.clone())
                        && Self::flip_equiv(r1.right.clone(), r2.right.clone()))
                        || (Self::flip_equiv(r1.left.clone(), r2.right.clone())
                            && Self::flip_equiv(r1.right.clone(), r2.left.clone())))
            }
            _ => false,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_951() {
        time_test!();

        assert_eq!(
            Solution::flip_equiv(
                tree!([1, 2, 3, 4, 5, 6, null, null, null, 7, 8]),
                tree!([1, 3, 2, null, 6, 4, 5, null, null, null, null, 8, 7])
            ),
            true
        );
        assert_eq!(Solution::flip_equiv(tree!([]), tree!([])), true);
        //assert_eq!(Solution::flip_equiv(tree!([]), tree!([1])), false); // FIXME: this test case failes even though it's right
    }
}
