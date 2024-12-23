/**
 * [2583] Kth Largest Sum in a Binary Tree
 *
 * You are given the root of a binary tree and a positive integer k.
 * The level sum in the tree is the sum of the values of the nodes that are on the same level.
 * Return the k^th largest level sum in the tree (not necessarily distinct). If there are fewer than k levels in the tree, return -1.
 * Note that two nodes are on the same level if they have the same distance from the root.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/12/14/binaryytreeedrawio-2.png" style="width: 301px; height: 284px;" />
 * Input: root = [5,8,9,2,1,3,7,4,6], k = 2
 * Output: 13
 * Explanation: The level sums are the following:
 * - Level 1: 5.
 * - Level 2: 8 + 9 = 17.
 * - Level 3: 2 + 1 + 3 + 7 = 13.
 * - Level 4: 4 + 6 = 10.
 * The 2^nd largest level sum is 13.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/12/14/treedrawio-3.png" style="width: 181px; height: 181px;" />
 * Input: root = [1,2,null,3], k = 1
 * Output: 3
 * Explanation: The largest level sum is 3.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is n.
 * 	2 <= n <= 10^5
 * 	1 <= Node.val <= 10^6
 * 	1 <= k <= n
 *
 */
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/
// discuss: https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        // Depth-First Search
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i64>, depth: usize) {
            if let Some(node) = root {
                let node = node.borrow_mut();
                match res.get_mut(depth) {
                    Some(value) => *value += node.val as i64,
                    None => res.push(node.val as i64),
                }
                dfs(node.right.clone(), res, depth + 1);
                dfs(node.left.clone(), res, depth + 1);
            }
        }

        let mut res = vec![];
        dfs(root, &mut res, 0);
        res.sort_unstable_by(|a, b| b.cmp(a));
        *res.get(k as usize - 1).unwrap_or(&-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2583() {
        time_test!();
        assert_eq!(
            Solution::kth_largest_level_sum(tree![5, 8, 9, 2, 1, 3, 7, 4, 6], 2),
            13
        );
        assert_eq!(Solution::kth_largest_level_sum(tree![1, 2, null, 3], 1), 3);
    }
}
