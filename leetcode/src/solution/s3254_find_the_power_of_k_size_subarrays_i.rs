/**
 * [3254] Find the Power of K-Size Subarrays I
 *
 * You are given an array of integers nums of length n and a positive integer k.
 * The power of an array is defined as:
 *
 * 	Its maximum element if all of its elements are consecutive and sorted in ascending order.
 * 	-1 otherwise.
 *
 * You need to find the power of all <span data-keyword="subarray-nonempty">subarrays</span> of nums of size k.
 * Return an integer array results of size n - k + 1, where results[i] is the power of nums[i..(i + k - 1)].
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [1,2,3,4,3,2,5], k = 3</span>
 * Output: [3,4,-1,-1,-1]
 * Explanation:
 * There are 5 subarrays of nums of size 3:
 *
 * 	[1, 2, 3] with the maximum element 3.
 * 	[2, 3, 4] with the maximum element 4.
 * 	[3, 4, 3] whose elements are not consecutive.
 * 	[4, 3, 2] whose elements are not sorted.
 * 	[3, 2, 5] whose elements are not consecutive.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [2,2,2,2,2], k = 4</span>
 * Output: <span class="example-io">[-1,-1]</span>
 * </div>
 * Example 3:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [3,2,3,2,3,2], k = 2</span>
 * Output: <span class="example-io">[-1,3,-1,3,-1]</span>
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= n == nums.length <= 500
 * 	1 <= nums[i] <= 10^5
 * 	1 <= k <= n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-power-of-k-size-subarrays-i/
// discuss: https://leetcode.com/problems/find-the-power-of-k-size-subarrays-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut results = Vec::new();
        for i in 0..=nums.len() - k as usize {
            let subarray = &nums[i..i + k as usize];
            if subarray.windows(2).all(|w| w[0] + 1 == w[1]) {
                results.push(*subarray.iter().max().unwrap());
            } else {
                results.push(-1);
            }
        }
        results
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3254() {
        time_test!();

        assert_eq!(
            Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3),
            vec![3, 4, -1, -1, -1]
        );
        assert_eq!(
            Solution::results_array(vec![2, 2, 2, 2, 2], 4),
            vec![-1, -1]
        );
        assert_eq!(
            Solution::results_array(vec![3, 2, 3, 2, 3, 2], 2),
            vec![-1, 3, -1, 3, -1]
        );
    }
}
