/**
 * [670] Maximum Swap
 *
 * You are given an integer num. You can swap two digits at most once to get the maximum valued number.
 * Return the maximum valued number you can get.
 *  
 * Example 1:
 *
 * Input: num = 2736
 * Output: 7236
 * Explanation: Swap the number 2 and the number 7.
 *
 * Example 2:
 *
 * Input: num = 9973
 * Output: 9973
 * Explanation: No swap.
 *
 *  
 * Constraints:
 *
 * 	0 <= num <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-swap/
// discuss: https://leetcode.com/problems/maximum-swap/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut chars: Vec<char> = num.to_string().chars().collect();
        let mut max_idx = chars.len() - 1;
        let mut x = 0;
        let mut y = 0;

        for i in (0..chars.len()).rev() {
            if chars[i] > chars[max_idx] {
                max_idx = i;
            } else if chars[i] < chars[max_idx] {
                x = i;
                y = max_idx;
            }
        }

        chars.swap(x, y);
        chars
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_670() {
        let start = std::time::Instant::now();

        assert_eq!(Solution::maximum_swap(2736), 7236);
        assert_eq!(Solution::maximum_swap(9973), 9973);

        println!("Time elapsed in {} is: {:?}", file!(), start.elapsed());
    }
}
