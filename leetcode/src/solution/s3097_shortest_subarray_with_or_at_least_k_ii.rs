/**
 * [3097] Shortest Subarray With OR at Least K II
 *
 * You are given an array nums of non-negative integers and an integer k.
 * An array is called special if the bitwise OR of all of its elements is at least k.
 * Return the length of the shortest special non-empty <span data-keyword="subarray-nonempty">subarray</span> of nums, or return -1 if no special subarray exists.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [1,2,3], k = 2</span>
 * Output: <span class="example-io">1</span>
 * Explanation:
 * The subarray [3] has OR value of 3. Hence, we return 1.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [2,1,8], k = 10</span>
 * Output: <span class="example-io">3</span>
 * Explanation:
 * The subarray [2,1,8] has OR value of 11. Hence, we return 3.
 * </div>
 * Example 3:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [1,2], k = 0</span>
 * Output: <span class="example-io">1</span>
 * Explanation:
 * The subarray [1] has OR value of 1. Hence, we return 1.
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2 * 10^5
 * 	0 <= nums[i] <= 10^9
 * 	0 <= k <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-ii/
// discuss: https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut minimum_length = i32::MAX;
        let mut bitwise_or = 0;
        let mut bits = vec![0; 32];

        let transform = |bits: &mut Vec<i32>, num: i32, delta: i32| -> i32 {
            let mut result = 0;
            // I just learned that Rust has a built-in constant i32::BITS and can also use trailing_zeros() and count_ones()
            for i in 0..32 {
                // rust also has std::ops::BitAnd trait num.bitand(1 << i) != 0
                if (num >> i) & 1 == 1 {
                    bits[i] += delta;
                }
                if bits[i] > 0 {
                    result |= 1 << i;
                }
            }
            result
        };

        // Sliding window approach (Rust has .windows() method but I chose not to use it)
        let mut left = 0;
        for right in 0..nums.len() {
            bitwise_or = transform(&mut bits, nums[right], 1);
            while bitwise_or >= k && left <= right {
                minimum_length = minimum_length.min((right - left + 1) as i32);
                bitwise_or = transform(&mut bits, nums[left], -1);
                left += 1;
            }
        }

        if minimum_length == i32::MAX {
            -1
        } else {
            minimum_length
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3097() {
        time_test!();

        assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
        assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1);
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 32, 21], 55), 3);
    }
}
