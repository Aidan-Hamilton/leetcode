use std::cmp::max;

/**
 * [3133] Minimum Array End
 *
 * You are given two integers n and x. You have to construct an array of positive integers nums of size n where for every 0 <= i < n - 1, nums[i + 1] is greater than nums[i], and the result of the bitwise AND operation between all elements of nums is x.
 * Return the minimum possible value of nums[n - 1].
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">n = 3, x = 4</span>
 * Output: <span class="example-io">6</span>
 * Explanation:
 * nums can be [4,5,6] and its last element is 6.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">n = 2, x = 7</span>
 * Output: <span class="example-io">15</span>
 * Explanation:
 * nums can be [7,15] and its last element is 15.
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= n, x <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-array-end/
// discuss: https://leetcode.com/problems/minimum-array-end/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let x = x as i64;
        let n = n as i64 - 1;
        let mut res: i64 = x;
        let mut bx: i64 = 1;
        let mut bn: i64 = 1;

        while bn <= n {
            // unset bit found in x
            if (bx & x) == 0 {
                // set bit found in n
                if bn & n != 0 {
                    res |= bx;
                }
                bn <<= 1;
            }
            bx <<= 1;
        }

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3133() {
        time_test!();

        assert_eq!(Solution::min_end(3, 4), 6);
        assert_eq!(Solution::min_end(2, 7), 15);
    }
}
