/**
 * [2501] Longest Square Streak in an Array
 *
 * You are given an integer array nums. A subsequence of nums is called a square streak if:
 *
 * 	The length of the subsequence is at least 2, and
 * 	after sorting the subsequence, each element (except the first element) is the square of the previous number.
 *
 * Return the length of the longest square streak in nums, or return -1 if there is no square streak.
 * A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
 *  
 * Example 1:
 *
 * Input: nums = [4,3,6,16,8,2]
 * Output: 3
 * Explanation: Choose the subsequence [4,16,2]. After sorting it, it becomes [2,4,16].
 * - 4 = 2 * 2.
 * - 16 = 4 * 4.
 * Therefore, [4,16,2] is a square streak.
 * It can be shown that every subsequence of length 4 is not a square streak.
 *
 * Example 2:
 *
 * Input: nums = [2,3,5,6,7]
 * Output: -1
 * Explanation: There is no square streak in nums so return -1.
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 10^5
 * 	2 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-square-streak-in-an-array/
// discuss: https://leetcode.com/problems/longest-square-streak-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().collect::<HashSet<_>>();
        let mut max_streak = 0;
        for num in nums.iter() {
            let mut count = 0;
            let Some(mut current) = num.checked_mul(*num) else {
                continue;
            };
            while nums.contains(&current) {
                count += 1;
                let Some(c) = current.checked_mul(current) else {
                    break;
                };
                current = c;
            }
            max_streak = max_streak.max(count + 1);
        }
        if max_streak > 1 {
            max_streak
        } else {
            -1
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2501() {
        time_test!();

        assert_eq!(Solution::longest_square_streak(vec![4, 3, 6, 16, 8, 2]), 3);
        assert_eq!(Solution::longest_square_streak(vec![2, 3, 5, 6, 7]), -1);
        assert_eq!(Solution::longest_square_streak(vec![92682, 18532]), -1);
        assert_eq!(Solution::longest_square_streak(vec![27, 3, 9, 18, 81]), 3);
        assert_eq!(
            Solution::longest_square_streak(vec![2, 49, 3, 5, 7, 2401]),
            3
        );
        assert_eq!(
            Solution::longest_square_streak(vec![81, 2, 4, 16, 3, 9, 6561]),
            4
        );
    }
}
