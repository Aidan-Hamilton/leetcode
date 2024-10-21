/**
 * [1593] Split a String Into the Max Number of Unique Substrings
 *
 * Given a string s<var>,</var> return the maximum number of unique substrings that the given string can be split into.
 * You can split string s into any list of non-empty substrings, where the concatenation of the substrings forms the original string. However, you must split the substrings such that all of them are unique.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * Example 1:
 *
 * Input: s = "ababccc"
 * Output: 5
 * Explanation: One way to split maximally is ['a', 'b', 'ab', 'c', 'cc']. Splitting like ['a', 'b', 'a', 'b', 'c', 'cc'] is not valid as you have 'a' and 'b' multiple times.
 *
 * Example 2:
 *
 * Input: s = "aba"
 * Output: 2
 * Explanation: One way to split maximally is ['a', 'ba'].
 *
 * Example 3:
 *
 * Input: s = "aa"
 * Output: 1
 * Explanation: It is impossible to split the string any further.
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= s.length <= 16
 *
 *
 * 	s contains only lower case English letters.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/
// discuss: https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
// not sure if I should import std::collections::HashSet as there are only two references to it
impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        // Depth-First Search (DFS) backtracking to find all possible splits
        fn dfs(s: &str, set: &mut std::collections::HashSet<String>, result: &mut i32) {
            if s.len() == 0 {
                *result = std::cmp::max(*result, set.len() as i32);
                return;
            }
            for i in 1..=s.len() {
                let (left, right) = s.split_at(i);
                if !set.contains(left) {
                    set.insert(left.to_string());
                    dfs(right, set, result);
                    set.remove(left);
                }
            }
        }
        let mut result = 0;
        let mut set = std::collections::HashSet::new();
        dfs(&s, &mut set, &mut result);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1593() {
        let start = std::time::Instant::now();

        // default test cases
        assert_eq!(Solution::max_unique_split("ababccc".to_owned()), 5);
        assert_eq!(Solution::max_unique_split("aba".to_owned()), 2);
        assert_eq!(Solution::max_unique_split("aa".to_owned()), 1);
        // custom test cases
        assert_eq!(Solution::max_unique_split("dbbpaaaab".to_owned()), 6);
        assert_eq!(Solution::max_unique_split("aacaccckaaaaaa".to_owned()), 7);
        assert_eq!(Solution::max_unique_split("jnafaeffbehaif".to_owned()), 11);
        assert_eq!(Solution::max_unique_split("aaaaaaaaaaaaaaaa".to_owned()), 5);
        assert_eq!(Solution::max_unique_split("wwwzfvedwfvhsww".to_owned()), 11);

        println!("Time elapsed in {} is: {:?}", file!(), start.elapsed());
    }
}
