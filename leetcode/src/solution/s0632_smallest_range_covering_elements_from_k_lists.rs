/**
 * [632] Smallest Range Covering Elements from K Lists
 *
 * You have k lists of sorted integers in non-decreasing order. Find the smallest range that includes at least one number from each of the k lists.
 * We define the range [a, b] is smaller than range [c, d] if b - a < d - c or a < c if b - a == d - c.
 *  
 * Example 1:
 *
 * Input: nums = [[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]
 * Output: [20,24]
 * Explanation:
 * List 1: [4, 10, 15, 24,26], 24 is in range [20,24].
 * List 2: [0, 9, 12, 20], 20 is in range [20,24].
 * List 3: [5, 18, 22, 30], 22 is in range [20,24].
 *
 * Example 2:
 *
 * Input: nums = [[1,2,3],[1,2,3],[1,2,3]]
 * Output: [1,1]
 *
 *  
 * Constraints:
 *
 * 	nums.length == k
 * 	1 <= k <= 3500
 * 	1 <= nums[i].length <= 50
 * 	-10^5 <= nums[i][j] <= 10^5
 * 	nums[i] is sorted in non-decreasing order.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/
// discuss: https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut min_heap = BinaryHeap::new();
        let mut max_value = i32::MIN;

        for (i, list) in nums.iter().enumerate() {
            min_heap.push(Reverse((list[0], i, 0)));
            max_value = max_value.max(list[0]);
        }

        let mut range = (i32::MIN, i32::MAX);

        while let Some(Reverse((min_value, list_index, element_index))) = min_heap.pop() {
            if max_value.saturating_sub(min_value) < range.1.saturating_sub(range.0)
                || (max_value.saturating_sub(min_value) == range.1.saturating_sub(range.0)
                    && min_value < range.0)
            {
                range = (min_value, max_value);
            }

            if element_index + 1 < nums[list_index].len() {
                let next_value = nums[list_index][element_index + 1];
                min_heap.push(Reverse((next_value, list_index, element_index + 1)));
                max_value = max_value.max(next_value);
            } else {
                break;
            }
        }

        vec![range.0, range.1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_632() {
        let start = std::time::Instant::now();

        assert_eq!(
            Solution::smallest_range(vec![
                vec![4, 10, 15, 24, 26],
                vec![0, 9, 12, 20],
                vec![5, 18, 22, 30]
            ]),
            vec![20, 24]
        );
        assert_eq!(
            Solution::smallest_range(vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]),
            vec![1, 1]
        );

        println!("Time elapsed in {} is: {:?}", file!(), start.elapsed());
    }
}
