/**
 * [1840] Maximum Building Height
 *
 * You want to build n new buildings in a city. The new buildings will be built in a line and are labeled from 1 to n.
 * However, there are city restrictions on the heights of the new buildings:
 *
 * 	The height of each building must be a non-negative integer.
 * 	The height of the first building must be 0.
 * 	The height difference between any two adjacent buildings cannot exceed 1.
 *
 * Additionally, there are city restrictions on the maximum height of specific buildings. These restrictions are given as a 2D integer array restrictions where restrictions[i] = [idi, maxHeighti] indicates that building idi must have a height less than or equal to maxHeighti.
 * It is guaranteed that each building will appear at most once in restrictions, and building 1 will not be in restrictions.
 * Return the maximum possible height of the tallest building.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/ic236-q4-ex1-1.png" style="width: 400px; height: 253px;" />
 * Input: n = 5, restrictions = [[2,1],[4,1]]
 * Output: 2
 * Explanation: The green area in the image indicates the maximum allowed height for each building.
 * We can build the buildings with heights [0,1,2,1,2], and the tallest building has a height of 2.
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/ic236-q4-ex2.png" style="width: 500px; height: 269px;" />
 * Input: n = 6, restrictions = []
 * Output: 5
 * Explanation: The green area in the image indicates the maximum allowed height for each building.
 * We can build the buildings with heights [0,1,2,3,4,5], and the tallest building has a height of 5.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/ic236-q4-ex3.png" style="width: 500px; height: 187px;" />
 * Input: n = 10, restrictions = [[5,3],[2,5],[7,4],[10,3]]
 * Output: 5
 * Explanation: The green area in the image indicates the maximum allowed height for each building.
 * We can build the buildings with heights [0,1,2,3,3,4,4,5,4,3], and the tallest building has a height of 5.
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 10^9
 * 	0 <= restrictions.length <= min(n - 1, 10^5)
 * 	2 <= idi <= n
 * 	idi is unique.
 * 	0 <= maxHeighti <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-building-height/
// discuss: https://leetcode.com/problems/maximum-building-height/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        let mut restrictions = restrictions;
        restrictions.push(vec![1, 0]);
        restrictions.sort();
        restrictions.push(vec![n, n - 1]);

        let m = restrictions.len();
        for i in 1..m {
            restrictions[i][1] = restrictions[i][1]
                .min(restrictions[i - 1][1] + (restrictions[i][0] - restrictions[i - 1][0]));
        }

        for i in (0..m - 1).rev() {
            restrictions[i][1] = restrictions[i][1]
                .min(restrictions[i + 1][1] + (restrictions[i + 1][0] - restrictions[i][0]));
        }

        let mut max_height = 0;
        for i in 1..m {
            let h1 = restrictions[i - 1][1];
            let h2 = restrictions[i][1];
            let d = restrictions[i][0] - restrictions[i - 1][0];
            max_height = max_height.max((h1 + h2 + d) / 2);
        }

        max_height
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1840() {
        time_test!();

        assert_eq!(Solution::max_building(5, vec_vec![[2, 1], [4, 1]]), 2);
        assert_eq!(Solution::max_building(6, vec_vec![]), 5);
        assert_eq!(
            Solution::max_building(10, vec_vec![[5, 3], [2, 5], [7, 4], [10, 3]]),
            5
        );
    }
}
