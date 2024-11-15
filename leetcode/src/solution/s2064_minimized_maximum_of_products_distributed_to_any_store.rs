/**
 * [2064] Minimized Maximum of Products Distributed to Any Store
 *
 * You are given an integer n indicating there are n specialty retail stores. There are m product types of varying amounts, which are given as a 0-indexed integer array quantities, where quantities[i] represents the number of products of the i^th product type.
 * You need to distribute all products to the retail stores following these rules:
 *
 * 	A store can only be given at most one product type but can be given any amount of it.
 * 	After distribution, each store will have been given some number of products (possibly 0). Let x represent the maximum number of products given to any store. You want x to be as small as possible, i.e., you want to minimize the maximum number of products that are given to any store.
 *
 * Return the minimum possible x.
 *  
 * Example 1:
 *
 * Input: n = 6, quantities = [11,6]
 * Output: 3
 * Explanation: One optimal way is:
 * - The 11 products of type 0 are distributed to the first four stores in these amounts: 2, 3, 3, 3
 * - The 6 products of type 1 are distributed to the other two stores in these amounts: 3, 3
 * The maximum number of products given to any store is max(2, 3, 3, 3, 3, 3) = 3.
 *
 * Example 2:
 *
 * Input: n = 7, quantities = [15,10,10]
 * Output: 5
 * Explanation: One optimal way is:
 * - The 15 products of type 0 are distributed to the first three stores in these amounts: 5, 5, 5
 * - The 10 products of type 1 are distributed to the next two stores in these amounts: 5, 5
 * - The 10 products of type 2 are distributed to the last two stores in these amounts: 5, 5
 * The maximum number of products given to any store is max(5, 5, 5, 5, 5, 5, 5) = 5.
 *
 * Example 3:
 *
 * Input: n = 1, quantities = [100000]
 * Output: 100000
 * Explanation: The only optimal way is:
 * - The 100000 products of type 0 are distributed to the only store.
 * The maximum number of products given to any store is max(100000) = 100000.
 *
 *  
 * Constraints:
 *
 * 	m == quantities.length
 * 	1 <= m <= n <= 10^5
 * 	1 <= quantities[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/
// discuss: https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // The discription is horrible, the comments said it's similar to 875. Koko Eating Bananas
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        fn can_distribute(quantities: &Vec<i32>, n: i32, max_products: i32) -> bool {
            let mut required_stores = 0;
            for &quantity in quantities {
                required_stores += (quantity + max_products - 1) / max_products;
                if required_stores > n {
                    return false;
                }
            }
            true
        }

        let mut left = 1;
        let mut right = *quantities.iter().max().unwrap();
        while left < right {
            let mid = left + (right - left) / 2;
            if can_distribute(&quantities, n, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2064() {
        time_test!();

        assert_eq!(Solution::minimized_maximum(6, vec![11, 6]), 3);
        assert_eq!(Solution::minimized_maximum(7, vec![15, 10, 10]), 5);
        assert_eq!(Solution::minimized_maximum(1, vec![100000]), 100000);
        assert_eq!(Solution::minimized_maximum(2, vec![5, 8]), 8);
        assert_eq!(Solution::minimized_maximum(100000, vec![1]), 1);
        assert_eq!(
            Solution::minimized_maximum(15, vec![18, 28, 11, 8, 22, 16, 24, 18, 26, 26, 21, 24]),
            24
        );
        assert_eq!(
            Solution::minimized_maximum(
                100000,
                vec![4, 5, 4, 2, 1, 1, 4, 5, 2, 5, 3, 1, 2, 5, 2, 4, 2, 2, 2, 3, 1, 4, 1, 3, 3]
            ),
            1
        );
        assert_eq!(
            Solution::minimized_maximum(
                100000,
                vec![1, 5, 4, 5, 4, 1, 1, 2, 2, 4, 1, 1, 4, 5, 3, 3, 4, 1, 4, 4, 4, 2, 4, 2, 4]
            ),
            1
        );
    }
}
