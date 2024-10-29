/**
 * [2684] Maximum Number of Moves in a Grid
 *
 * You are given a 0-indexed m x n matrix grid consisting of positive integers.
 * You can start at any cell in the first column of the matrix, and traverse the grid in the following way:
 *
 * 	From a cell (row, col), you can move to any of the cells: (row - 1, col + 1), (row, col + 1) and (row + 1, col + 1) such that the value of the cell you move to, should be strictly bigger than the value of the current cell.
 *
 * Return the maximum number of moves that you can perform.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/04/11/yetgriddrawio-10.png" style="width: 201px; height: 201px;" />
 * Input: grid = [[2,4,3,5],[5,4,9,3],[3,4,2,11],[10,9,13,15]]
 * Output: 3
 * Explanation: We can start at the cell (0, 0) and make the following moves:
 * - (0, 0) -> (0, 1).
 * - (0, 1) -> (1, 2).
 * - (1, 2) -> (2, 3).
 * It can be shown that it is the maximum number of moves that can be made.
 * Example 2:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/04/12/yetgrid4drawio.png" />
 * Input: grid = [[3,2,4],[2,1,9],[1,1,7]]
 * Output: 0
 * Explanation: Starting from any cell in the first column we cannot perform any moves.
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	2 <= m, n <= 1000
 * 	4 <= m * n <= 10^5
 * 	1 <= grid[i][j] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/
// discuss: https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn max_moves(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        // should probably use DFS over DP
        let mut dp = vec![vec![0; n]; m];
        let mut stack = (0..m).into_iter().collect::<Vec<_>>();
        for j in 0..n - 1 {
            let mut set = HashSet::new();
            while let Some(i) = stack.pop() {
                let nv = dp[i][j] + 1;
                if 0 < i && grid[i][j] < grid[i - 1][j + 1] {
                    dp[i - 1][j + 1] = dp[i - 1][j + 1].max(nv);
                    set.insert(i - 1);
                }
                if grid[i][j] < grid[i][j + 1] {
                    dp[i][j + 1] = dp[i][j + 1].max(nv);
                    set.insert(i);
                }
                if i < m - 1 && grid[i][j] < grid[i + 1][j + 1] {
                    dp[i + 1][j + 1] = dp[i + 1][j + 1].max(nv);
                    set.insert(i + 1);
                }
            }
            stack = set.into_iter().collect::<Vec<_>>();
        }

        // Iter through the last column to find the max value
        let result = dp.iter().flatten().cloned().max().unwrap_or(0);
        /*
            for i in 0..n {
                for j in 0..m {
                    result = result.max(dp[i][j]);
                }
            }
        */
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2684() {
        time_test!();

        assert_eq!(
            Solution::max_moves(vec![
                vec![2, 4, 3, 5],
                vec![5, 4, 9, 3],
                vec![3, 4, 2, 11],
                vec![10, 9, 13, 15]
            ]),
            3
        );
        assert_eq!(
            Solution::max_moves(vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]]),
            0
        );
        assert_eq!(
            Solution::max_moves(vec![vec![2, 625152, 3, 5], vec![625151, 625152, 9, 3]]),
            1
        );
        assert_eq!(
            Solution::max_moves(vec![
                vec![207, 141, 103, 12],
                vec![148, 187, 211, 88],
                vec![116, 16, 67, 157],
                vec![210, 93, 137, 120],
                vec![52, 116, 241, 86],
                vec![58, 213, 3, 292]
            ]),
            3
        );
        assert_eq!(
            Solution::max_moves(vec![
                vec![19, 13, 5, 10, 30, 19, 28],
                vec![17, 9, 2, 26, 9, 24, 3],
                vec![1, 12, 13, 21, 18, 12, 8],
                vec![17, 10, 13, 15, 19, 30, 6],
                vec![14, 5, 24, 24, 17, 22, 6]
            ]),
            6
        );
        assert_eq!(
            Solution::max_moves(vec![
                vec![27, 134, 137, 68],
                vec![265, 270, 273, 72],
                vec![217, 41, 165, 61],
                vec![40, 269, 19, 75],
                vec![143, 152, 15, 260],
                vec![149, 173, 38, 63],
                vec![184, 151, 26, 258],
                vec![171, 278, 112, 37],
                vec![142, 191, 269, 244]
            ]),
            2
        );
        assert_eq!(
            Solution::max_moves(vec![
                vec![42, 19, 111, 153, 142, 109, 186, 159, 143, 57, 252, 103, 57, 214],
                vec![22, 110, 26, 286, 295, 2, 179, 119, 276, 101, 28, 290, 265, 11],
                vec![27, 35, 69, 186, 47, 145, 284, 250, 32, 117, 8, 161, 106, 175],
                vec![105, 192, 297, 207, 286, 187, 296, 257, 284, 293, 145, 114, 142, 263],
                vec![252, 178, 193, 114, 191, 149, 261, 97, 246, 292, 116, 170, 166, 64],
                vec![12, 155, 266, 291, 58, 91, 177, 6, 299, 29, 110, 200, 205, 170],
                vec![197, 41, 35, 241, 219, 120, 251, 121, 36, 76, 249, 14, 214, 220],
                vec![215, 105, 172, 254, 200, 34, 56, 221, 214, 196, 268, 102, 4, 265]
            ]),
            4
        );
        assert_eq!(
            Solution::max_moves(vec![
                vec![50, 251, 23, 194, 117, 113, 182, 69, 61],
                vec![97, 41, 298, 205, 269, 34, 296, 179, 51],
                vec![282, 218, 255, 119, 68, 275, 190, 269, 238],
                vec![251, 174, 164, 184, 110, 182, 298, 114, 247],
                vec![286, 104, 244, 223, 134, 206, 150, 196, 83],
                vec![18, 52, 192, 140, 188, 230, 120, 287, 63],
                vec![206, 83, 104, 187, 263, 191, 53, 98, 110],
                vec![109, 149, 115, 145, 152, 37, 116, 98, 240],
                vec![230, 97, 93, 33, 59, 139, 293, 108, 173],
                vec![125, 10, 162, 223, 220, 165, 100, 164, 197],
                vec![40, 190, 28, 30, 145, 259, 73, 102, 209],
                vec![151, 172, 194, 215, 41, 200, 202, 152, 231],
                vec![241, 218, 235, 174, 10, 51, 161, 243, 191]
            ]),
            6
        );
    }
}
