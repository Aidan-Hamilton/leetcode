/**
 * [1277] Count Square Submatrices with All Ones
 *
 * Given a m * n matrix of ones and zeros, return how many square submatrices have all ones.
 *  
 * Example 1:
 *
 * Input: matrix =
 * [
 *   [0,1,1,1],
 *   [1,1,1,1],
 *   [0,1,1,1]
 * ]
 * Output: 15
 * Explanation:
 * There are 10 squares of side 1.
 * There are 4 squares of side 2.
 * There is  1 square of side 3.
 * Total number of squares = 10 + 4 + 1 = 15.
 *
 * Example 2:
 *
 * Input: matrix =
 * [
 *   [1,0,1],
 *   [1,1,0],
 *   [1,1,0]
 * ]
 * Output: 7
 * Explanation:
 * There are 6 squares of side 1.  
 * There is 1 square of side 2.
 * Total number of squares = 6 + 1 = 7.
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 300
 * 	1 <= arr[0].length <= 300
 * 	0 <= arr[i][j] <= 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-square-submatrices-with-all-ones/
// discuss: https://leetcode.com/problems/count-square-submatrices-with-all-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n]; m];
        let mut result = 0;
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 1 {
                    dp[i][j] = 1;
                    if i > 0 && j > 0 {
                        dp[i][j] += dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1]));
                    }
                    result += dp[i][j];
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1277() {
        time_test!();

        assert_eq!(
            Solution::count_squares(vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]]),
            15
        );
        assert_eq!(
            Solution::count_squares(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]),
            7
        );
    }
}
