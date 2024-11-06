/**
 * [3003] Maximize the Number of Partitions After Operations
 *
 * You are given a string s and an integer k.
 * First, you are allowed to change at most one index in s to another lowercase English letter.
 * After that, do the following partitioning operation until s is empty:
 *
 * 	Choose the longest prefix of s containing at most k distinct characters.
 * 	Delete the prefix from s and increase the number of partitions by one. The remaining characters (if any) in s maintain their initial order.
 *
 * Return an integer denoting the maximum number of resulting partitions after the operations by optimally choosing at most one index to change.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">s = "accca", k = 2</span>
 * Output: <span class="example-io">3</span>
 * Explanation:
 * The optimal way is to change s[2] to something other than a and c, for example, b. then it becomes "acbca".
 * Then we perform the operations:
 *
 * 	The longest prefix containing at most 2 distinct characters is "ac", we remove it and s becomes "bca".
 * 	Now The longest prefix containing at most 2 distinct characters is "bc", so we remove it and s becomes "a".
 * 	Finally, we remove "a" and s becomes empty, so the procedure ends.
 *
 * Doing the operations, the string is divided into 3 partitions, so the answer is 3.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">s = "aabaab", k = 3</span>
 * Output: <span class="example-io">1</span>
 * Explanation:
 * Initially s contains 2 distinct characters, so whichever character we change, it will contain at most 3 distinct characters, so the longest prefix with at most 3 distinct characters would always be all of it, therefore the answer is 1.
 * </div>
 * Example 3:
 * <div class="example-block">
 * Input: <span class="example-io">s = "xxyz", k = 1</span>
 * Output: <span class="example-io">4</span>
 * Explanation:
 * The optimal way is to change s[0] or s[1] to something other than characters in s, for example, to change s[0] to w.
 * Then s becomes "wxyz", which consists of 4 distinct characters, so as k is 1, it will divide into 4 partitions.
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^4
 * 	s consists only of lowercase English letters.
 * 	1 <= k <= 26
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-the-number-of-partitions-after-operations/
// discuss: https://leetcode.com/problems/maximize-the-number-of-partitions-after-operations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // I spent a couple hours on this crap
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        fn dfs(
            s: &Vec<char>,
            i: usize,
            mask: i32,
            changed: bool,
            k: i32,
            memo: &mut std::collections::HashMap<(usize, i32, bool), i32>,
        ) -> i32 {
            if i == s.len() && changed {
                return 1;
            }
            if i == s.len() && !changed {
                return i32::MIN;
            }
            if let Some(&res) = memo.get(&(i, mask, changed)) {
                return res;
            }
            let a = s[i];
            let od = (a as u8 - b'a') as i32;
            let mut res = 0;
            let newmask = mask | (1 << od);
            if newmask.count_ones() as i32 > k {
                res = res.max(dfs(s, i + 1, 1 << od, changed, k, memo) + 1);
            } else {
                res = res.max(dfs(s, i + 1, newmask, changed, k, memo));
            }
            if changed {
                memo.insert((i, mask, changed), res);
                return res;
            }
            for q in 0..26 {
                if q == od {
                    continue;
                }
                let newmask = mask | (1 << q);
                if newmask.count_ones() as i32 > k {
                    res = res.max(dfs(s, i + 1, 1 << q, true, k, memo) + 1);
                } else {
                    res = res.max(dfs(s, i + 1, newmask, true, k, memo));
                }
            }
            memo.insert((i, mask, changed), res);
            res
        }

        let s: Vec<char> = s.chars().collect();
        let mut memo = std::collections::HashMap::new();
        dfs(&s, 0, 0, false, k, &mut memo)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3003() {
        time_test!();

        assert_eq!(
            Solution::max_partitions_after_operations("accca".to_owned(), 2),
            3
        );
        assert_eq!(
            Solution::max_partitions_after_operations("aabaab".to_owned(), 3),
            1
        );
        assert_eq!(
            Solution::max_partitions_after_operations("xxyz".to_owned(), 1),
            4
        );
    }
}
