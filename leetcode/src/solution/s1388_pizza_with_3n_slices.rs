/**
 * [1388] Pizza With 3n Slices
 *
 * There is a pizza with 3n slices of varying size, you and your friends will take slices of pizza as follows:
 *
 * 	You will pick any pizza slice.
 * 	Your friend Alice will pick the next slice in the anti-clockwise direction of your pick.
 * 	Your friend Bob will pick the next slice in the clockwise direction of your pick.
 * 	Repeat until there are no more slices of pizzas.
 *
 * Given an integer array slices that represent the sizes of the pizza slices in a clockwise direction, return the maximum possible sum of slice sizes that you can pick.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/18/sample_3_1723.png" style="width: 500px; height: 266px;" />
 * Input: slices = [1,2,3,4,5,6]
 * Output: 10
 * Explanation: Pick pizza slice of size 4, Alice and Bob will pick slices with size 3 and 5 respectively. Then Pick slices with size 6, finally Alice and Bob will pick slice of size 2 and 1 respectively. Total = 4 + 6.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/18/sample_4_1723.png" style="width: 500px; height: 299px;" />
 * Input: slices = [8,9,8,6,1,1]
 * Output: 16
 * Explanation: Pick pizza slice of size 8 in each turn. If you pick slice with size 9 your partners will pick slices of size 8.
 *
 *  
 * Constraints:
 *
 * 	3 * n == slices.length
 * 	1 <= slices.length <= 500
 * 	1 <= slices[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pizza-with-3n-slices/
// discuss: https://leetcode.com/problems/pizza-with-3n-slices/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let k = slices.len();
        let mut prev1 = vec![0; k + 2];
        let mut curr1 = vec![0; k + 2];
        let mut next1 = vec![0; k + 2];

        let mut prev2 = vec![0; k + 2];
        let mut curr2 = vec![0; k + 2];
        let mut next2 = vec![0; k + 2];

        for index in (0..=k - 2).rev() {
            for n in 1..=k / 3 {
                let take = slices[index] + next1[n - 1];
                let nottake = curr1[n];
                prev1[n] = take.max(nottake);
            }
            next1 = curr1.clone();
            curr1 = prev1.clone();
        }

        let case1 = curr1[k / 3];

        for index in (1..=k - 1).rev() {
            for n in 1..=k / 3 {
                let take = slices[index] + next2[n - 1];
                let nottake = curr2[n];
                prev2[n] = take.max(nottake);
            }
            next2 = curr2.clone();
            curr2 = prev2.clone();
        }

        let case2 = curr2[k / 3];

        case1.max(case2)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1388() {
        time_test!();

        assert_eq!(Solution::max_size_slices(vec![1, 2, 3, 4, 5, 6]), 10);
        assert_eq!(Solution::max_size_slices(vec![8, 9, 8, 6, 1, 1]), 16);
    }
}
