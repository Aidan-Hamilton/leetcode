/**
 * [2601] Prime Subtraction Operation
 *
 * You are given a 0-indexed integer array nums of length n.
 * You can perform the following operation as many times as you want:
 *
 * 	Pick an index i that you haven&rsquo;t picked before, and pick a prime p strictly less than nums[i], then subtract p from nums[i].
 *
 * Return true if you can make nums a strictly increasing array using the above operation and false otherwise.
 * A strictly increasing array is an array whose each element is strictly greater than its preceding element.
 *  
 * Example 1:
 *
 * Input: nums = [4,9,6,10]
 * Output: true
 * Explanation: In the first operation: Pick i = 0 and p = 3, and then subtract 3 from nums[0], so that nums becomes [1,9,6,10].
 * In the second operation: i = 1, p = 7, subtract 7 from nums[1], so nums becomes equal to [1,2,6,10].
 * After the second operation, nums is sorted in strictly increasing order, so the answer is true.
 * Example 2:
 *
 * Input: nums = [6,8,11,12]
 * Output: true
 * Explanation: Initially nums is sorted in strictly increasing order, so we don't need to make any operations.
 * Example 3:
 *
 * Input: nums = [5,8,3]
 * Output: false
 * Explanation: It can be proven that there is no way to perform operations to make nums sorted in strictly increasing order, so the answer is false.
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i] <= 1000
 * 	<font face="monospace">nums.length == n</font>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/prime-subtraction-operation/
// discuss: https://leetcode.com/problems/prime-subtraction-operation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        fn is_prime(n: i32) -> bool {
            if n <= 1 {
                return false;
            }
            for i in 2..=((n as f64).sqrt() as i32) {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }
        // Evaluate all primes lower than 1000
        let primes: Vec<i32> = (2..1000).filter(|&x| is_prime(x)).collect();
        let mut last = 0;
        // Make prime as small as possible by subtracting available primes
        for &num in &nums {
            let mut min_num = num;
            for &prime in &primes {
                if prime >= num {
                    break;
                }
                let new_num = num - prime;
                if new_num > last {
                    min_num = min_num.min(new_num);
                }
            }
            if min_num <= last {
                return false;
            }
            last = min_num;
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2601() {
        time_test!();

        assert_eq!(Solution::prime_sub_operation(vec![4, 9, 6, 10]), true);
        assert_eq!(Solution::prime_sub_operation(vec![6, 8, 11, 12]), true);
        assert_eq!(Solution::prime_sub_operation(vec![5, 8, 3]), false);
    }
}
