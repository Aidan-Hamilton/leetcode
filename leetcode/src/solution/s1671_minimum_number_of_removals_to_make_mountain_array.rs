/**
 * [1671] Minimum Number of Removals to Make Mountain Array
 *
 * You may recall that an array arr is a mountain array if and only if:
 *
 * 	arr.length >= 3
 * 	There exists some index i (0-indexed) with 0 < i < arr.length - 1 such that:
 *
 * 		arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
 * 		arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
 *
 *
 *
 * Given an integer array nums​​​, return the minimum number of elements to remove to make nums​​​ a mountain array.
 *  
 * Example 1:
 *
 * Input: nums = [1,3,1]
 * Output: 0
 * Explanation: The array itself is a mountain array so we do not need to remove any elements.
 *
 * Example 2:
 *
 * Input: nums = [2,1,1,5,6,2,3,1]
 * Output: 3
 * Explanation: One solution is to remove the elements at indices 0, 1, and 5, making the array nums = [1,5,6,3,1].
 *
 *  
 * Constraints:
 *
 * 	3 <= nums.length <= 1000
 * 	1 <= nums[i] <= 10^9
 * 	It is guaranteed that you can make a mountain array out of nums.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/
// discuss: https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // To remove the elements to form a mountain, seek for the largest mountain first.
    // Use LIS from the left and from the right to find overall biggest mountain
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lis_left = vec![1; n];
        let mut lis_right = vec![1; n];

        // Calculate LIS from the left
        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    lis_left[i] = lis_left[i].max(lis_left[j] + 1);
                }
            }
        }

        // Calculate LIS from the right
        for i in (0..n - 1).rev() {
            for j in (i + 1..n).rev() {
                if nums[i] > nums[j] {
                    lis_right[i] = lis_right[i].max(lis_right[j] + 1);
                }
            }
        }

        // Find the maximum mountain array length
        let mut max_mountain_len = 0;
        for i in 1..n - 1 {
            if lis_left[i] > 1 && lis_right[i] > 1 {
                max_mountain_len = max_mountain_len.max(lis_left[i] + lis_right[i] - 1);
            }
        }

        (n - max_mountain_len) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1671() {
        time_test!();

        assert_eq!(Solution::minimum_mountain_removals(vec![1, 3, 1]), 0);
        assert_eq!(
            Solution::minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]),
            3
        );
        assert_eq!(
            Solution::minimum_mountain_removals(vec![100, 92, 89, 77, 74, 66, 64, 66, 64]),
            6
        );
        assert_eq!(
            Solution::minimum_mountain_removals(vec![
                4, 5, 13, 17, 1, 7, 6, 11, 2, 8, 10, 15, 3, 9, 12, 14, 16
            ]),
            10
        );
        assert_eq!(
            Solution::minimum_mountain_removals(vec![1, 16, 84, 9, 29, 71, 86, 79, 72, 12]),
            2
        );
        assert_eq!(
            Solution::minimum_mountain_removals(vec![9, 8, 1, 7, 6, 5, 4, 3, 2, 1]),
            2
        );
    }
}
